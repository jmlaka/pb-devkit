# PB333 system metadata from PbdViewer (Linux, no PowerBuilder runtime)

The legacy PowerBuilder 12.5 application has P-code compiled entry version
**333 decimal = 0x014D hex**. The open-source
[Hucxy/PbdViewer](https://github.com/Hucxy/PbdViewer) project includes a
corresponding **GZIP-compressed system type group** in
[resoures/014d.bin](https://github.com/Hucxy/PbdViewer/blob/main/resoures/014d.bin).

PbdViewer's \`PbFile.cs\` explicitly loads this resource, decompresses it with
\`GZipStream\`, and constructs \`PbEntry("_typedef.grp", entryData)\`.
The group contains public runtime *type metadata*, including property/member
names and function-definition names. We do **not** copy the binary resource
into this repository.

## Download and decode on Ubuntu

Run from the root of this repository:

\`\`\`bash
curl -fLSs https://raw.githubusercontent.com/Hucxy/PbdViewer/main/resoures/014d.bin \
  -o /tmp/pb333_014d.bin
gzip -dc /tmp/pb333_014d.bin > /tmp/pb333_typedef.grp

cargo test -p pb-devkit-cli --example probe_pcode
cargo run --release -p pb-devkit-cli --example probe_pcode -- \
  --system-metadata /tmp/pb333_typedef.grp
\`\`\`

The Rust example accepts the **uncompressed** \`.grp\` bytes. This operation
does not require a Windows machine, \`pbvm125.dll\`, PBNI SDK, ODBC, or a
database connection; it also does not read \`proamb.exe\`.

## Output

Only public catalog metadata is emitted:

- \`transaction\` property names at P-code member indices \`#2\`, \`#14\`,
  \`#15\`, including inherited fields if present.
- The system function name stored in object \`0x40D0\`, definition \`#339\`.

These are the two unresolved references discovered in \`f_connect.fun\` and
\`f_heslo.fun\`.

**Caution**: the returned property name and function name must be checked
against their runtime semantics. No secret values, function execution, DB
connections, or transformations of credentials occur. If the system group's
layout differs, the parser reports an error instead of inventing names.

Reference implementation: \`pb-devkit-2.x/pb-devkit-cli/examples/probe_pcode.rs\`.


## Diagnose CONNECT route predicates without exposing literals

The DB connection function has three alternative first connection sites:
0x01D6, 0x03FE and 0x04F4. Their selectors are conditional jumps at
0x009C and 0x02D8. The ordinary flow renderer can fail to reconstruct a
predicate when a basic block contains an unsupported instruction.

Use the fully redacted metadata trace to identify the missing opcode and
the types of anonymous local variables:

```bash
cargo test -p pb-devkit-cli --example probe_pcode
cargo run --release -p pb-devkit-cli --example probe_pcode -- \
  --connect-gate-trace f_connect.fun 0
```

The output never prints string literals, numeric literal values, connection
parameters, raw instruction operands or database credentials. It prints
instruction offsets and types, opcode names, validated symbol names, and
branch destination offsets only. Unknown operations remain labeled as such;
they are not assumed to have any particular stack effect.


## Verify the connection function's return contract

The three failed paths in \`f_connect.fun\` return transaction member \`#9\`,
while the common completion path executes a dynamic SQL statement and then
returns a literal \`0\`. Do not assume that member \`#9\` is \`SQLCode\`
until checked against the external catalog.

```bash
cargo test -p pb-devkit-cli --example probe_pcode

cargo run --release -p pb-devkit-cli --example probe_pcode -- \
  --system-metadata /tmp/pb333_typedef.grp

cargo run --release -p pb-devkit-cli --example probe_pcode -- \
  --connect-map f_connect.fun 0 /tmp/pb333_typedef.grp \
  | sed -n '/=== CONNECT RETURN CONTRACT/,$p'
```

This displays only catalog-backed names (such as \`at_sql.sqlcode\` if
the catalog confirms the mapping), control-flow evidence and a short
whitelisted status-literal description. It does not read SQL strings or
credentials.

**Caution:** \`SqlExecuteImmediate\` is a real DB operation if the application
itself is executed. This tool is static and does not execute it. A successful
connection does not establish that the subsequent dynamic statement is
read-only or that its result is checked before the \`0\` return.


## Inspect post-CONNECT EXECUTE IMMEDIATE without SQL disclosure

After successful connection, \`f_connect.fun\` contains an
\`SqlExecuteImmediate\` instruction at \`0x0660\`. The SQL text is supplied by
local variable \`ls_pom\`, using local transaction \`at_sql\`. This is a
potentially writing operation; running the application just to inspect it is
not recommended.

Static, offline analysis:

```bash
cargo test -p pb-devkit-cli --example probe_pcode
cargo run --release -p pb-devkit-cli --example probe_pcode -- \
  --post-connect-sql f_connect.fun 0
```

Only sanitized instruction metadata, symbol names, known referenced function
names, and **coarse allowlisted SQL-looking literal-prefix classes** are
reported. No SQL text, literal fragments, credentials or query parameters are
emitted. A prefix category found in a literal is **not evidence** that the
resulting SQL command has that category: SQL strings can be constructed
dynamically. The analysis performs no database operations.

For production read-only access, create a separately authorized, minimally
privileged database user and verify read-only permissions at the database
server, rather than relying on application-level flags or reverse-engineered
query text.


## Local opt-in: reveal compiled literals with \`--show-sensitive\`

The CLI recognizes the **same** \`--show-sensitive\` option in all modes,
regardless of where you put it. Without the flag, existing redaction stays
enabled. With it, diagnostically relevant **statically decoded literals**
(including strings or integers) are printed to **your local terminal**.

Examples (run from the repository root):

```bash
cargo run --release -p pb-devkit-cli --example probe_pcode -- \
  --connect-map f_connect.fun 0 /tmp/pb333_typedef.grp --show-sensitive

cargo run --release -p pb-devkit-cli --example probe_pcode -- \
  --connect-gate-trace f_connect.fun 0 --show-sensitive

cargo run --release -p pb-devkit-cli --example probe_pcode -- \
  --post-connect-sql f_connect.fun 0 --show-sensitive

cargo run --release -p pb-devkit-cli --example probe_pcode -- \
  f_heslo.fun 0 1000 --flow --show-sensitive
```

For commands that report *only metadata* (such as
\`--system-metadata\`, \`--transaction-catalog\`, and
\`--transaction-field-evidence\`), the global flag is accepted but no secret
data is available to reveal.

**Limits:** This opt-in exposes constants already stored in compiled P-code.
It does **not** execute functions, calculate dynamic strings, determine
runtime passwords, connect to a database or promise a reconstructed final
\`DBParm\`/SQL statement. Unknown literal encodings remain unparsed rather
than guessed.

**Security:** Sensitive output may contain database credentials or personal
information. Never paste the unredacted log into chats, tickets, or public
repositories. For a private local log on Linux, use \`umask 077\` before
redirecting output and protect/delete that log when finished. The tool does
not transmit its output.
