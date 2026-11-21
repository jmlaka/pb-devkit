#!/usr/bin/env python3
"""Offline reproduction of observed PowerBuilder string operations.

--mode-a-json assembles the pre-CONNECT Mode A ls_pom expression from
manually supplied local literals. It prints the assembled string verbatim,
including any UID/PWD fields present in those inputs. It does not derive
credentials, evaluate f_heslo, or make a database connection.

Keep recipes and output private: do not commit application-derived literals,
connection strings, or credentials to a repository.
PowerBuilder length is not necessarily identical to Python's code-point len()
for every non-ASCII character; these examples use ASCII.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import unittest


MODE_A_OFFSETS = (
    "00B6", "00BC", "00C6", "00D0", "00DA",
    "00FC", "0106", "0118", "0122", "012C",
)


def pb_fill(chars: str, length: int) -> str:
    """Repeat/truncate a character pattern to make a length-N string."""
    if length < 0:
        raise ValueError("length must be nonnegative")
    if not chars:
        raise ValueError("pattern must not be empty")
    repeats = (length + len(chars) - 1) // len(chars)
    return (chars * repeats)[:length]


def f_nahrad_znaky(text: str, old: str, new: str) -> str:
    """PB-equivalent Pos/Replace/Pos loop for a nonempty old pattern.

    After replacement, search resumes at the end of the inserted fragment.
    This deliberately avoids repeatedly replacing text *inside* new.
    """
    if not old:
        raise ValueError("old pattern must not be empty")
    pos = text.find(old)
    while pos >= 0:
        text = text[:pos] + new + text[pos + len(old):]
        pos = text.find(old, pos + len(new))
    return text


def mode_a_template(
    literals: dict[str, str], n_00e0: int, as_conname: str
) -> str:
    """Assemble ls_pom at f_connect P-code 0x00B2..0x0136.

    Offsets identify PushConstant *instructions*, not literal-pool offsets.
    No replacement or credential derivation is performed here.
    """
    if set(literals) != set(MODE_A_OFFSETS):
        missing = sorted(set(MODE_A_OFFSETS) - set(literals))
        extra = sorted(set(literals) - set(MODE_A_OFFSETS))
        raise ValueError(f"Incorrect literal keys: missing={missing}, extra={extra}")
    if any(not isinstance(value, str) for value in literals.values()):
        raise ValueError("All literal fragments must be strings")
    if type(n_00e0) is not int or not isinstance(as_conname, str):
        raise ValueError("n_00E0 must be integer and as_conname must be string")

    s = literals
    result = (
        s["00B6"] + s["00BC"] + s["00C6"] + s["00D0"]
        + pb_fill(s["00DA"], n_00e0)
        + s["00FC"] + s["0106"] + as_conname
        + s["0118"] + s["0122"] + s["012C"]
    )
    return result


def load_mode_a_recipe(path: Path) -> str:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, dict):
        raise ValueError("Recipe must be a JSON object")
    if set(payload) != {"literals", "n_00E0", "as_conname"}:
        raise ValueError("Recipe keys must be literals, n_00E0, as_conname")
    if not isinstance(payload["literals"], dict):
        raise ValueError("literals must be a JSON object")
    result = mode_a_template(
        payload["literals"], payload["n_00E0"], payload["as_conname"]
    )
    return result


def synthetic_example() -> str:
    """Synthetic exercise; neither a real DB connection nor a password."""
    template = "example-key=[****]; other-key=test"
    old_pattern = "[" + pb_fill("*", 4) + "]"
    new_fragment = "[DEMO]"
    return f_nahrad_znaky(template, old_pattern, new_fragment)


class StringOperationsTests(unittest.TestCase):
    def test_fill(self) -> None:
        self.assertEqual(pb_fill("ab", 5), "ababa")
        self.assertEqual(pb_fill("*", 0), "")
        with self.assertRaises(ValueError):
            pb_fill("", 3)
        with self.assertRaises(ValueError):
            pb_fill("x", -1)

    def test_replace(self) -> None:
        self.assertEqual(f_nahrad_znaky("a-xx-b-xx", "xx", "Q"),
                         "a-Q-b-Q")
        self.assertEqual(f_nahrad_znaky("aaa", "a", "aa"),
                         "aaaaaa")
        self.assertEqual(f_nahrad_znaky("abc", "z", "Q"), "abc")
        with self.assertRaises(ValueError):
            f_nahrad_znaky("abc", "", "Q")

    def test_demo(self) -> None:
        self.assertEqual(synthetic_example(),
                         "example-key=[DEMO]; other-key=test")

    def test_mode_a_exact_operand_order(self) -> None:
        pieces = {key: "<" + key + ">" for key in MODE_A_OFFSETS}
        pieces["00DA"] = "*"
        result = mode_a_template(pieces, 3, "<CONNAME>")
        self.assertEqual(
            result, (
                "<00B6><00BC><00C6><00D0>***"
                "<00FC><0106><CONNAME><0118><0122><012C>"
            )
        )

    def test_recipe_is_strict(self) -> None:
        with self.assertRaises(ValueError):
            mode_a_template({"00B6": "abc"}, 4, "demo")
        with self.assertRaises(ValueError):
            mode_a_template({key: "x" for key in MODE_A_OFFSETS}, -1, "demo")

    def test_mode_a_outputs_full_local_string(self) -> None:
        pieces = {key: "" for key in MODE_A_OFFSETS}
        pieces["00B6"] = "DSN=EXAMPLE;"
        pieces["00BC"] = "UID=test-user;"
        pieces["00C6"] = "PWD=test-password;"
        pieces["00DA"] = "*"
        self.assertEqual(
            mode_a_template(pieces, 0, ""),
            "DSN=EXAMPLE;UID=test-user;PWD=test-password;"
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--test", action="store_true",
                       help="run synthetic unit tests")
    group.add_argument("--mode-a-json", type=Path, metavar="PATH",
                       help="print pre-CONNECT Mode A ls_pom from local JSON verbatim")
    args = parser.parse_args()
    if args.test:
        unittest.main(argv=["pb_string_ops_demo"], exit=False)
    elif args.mode_a_json is not None:
        print(load_mode_a_recipe(args.mode_a_json))
    else:
        print(synthetic_example())
