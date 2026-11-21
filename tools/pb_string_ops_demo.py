#!/usr/bin/env python3
"""Offline, synthetic demonstration of PB string operations.

Imitates the observed PB333 Fill() operation and the separately recovered
f_nahrad_znaky() replacement routine. It deliberately contains no AMBIS
credentials, P-code literals, password material, or binary-extraction logic.

The fixtures use ASCII strings; Python's len() is not a full emulation of
PowerBuilder's Unicode string-length semantics.
"""

from __future__ import annotations

import argparse
import unittest


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
        self.assertEqual(f_nahrad_znaky("abc", "z", "Q"),
                         "abc")
        with self.assertRaises(ValueError):
            f_nahrad_znaky("abc", "", "Q")

    def test_demo(self) -> None:
        self.assertEqual(synthetic_example(),
                         "example-key=[DEMO]; other-key=test")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--test", action="store_true",
                        help="run synthetic unit tests")
    args = parser.parse_args()
    if args.test:
        unittest.main(argv=["pb_string_ops_demo"], exit=False)
    else:
        print(synthetic_example())
