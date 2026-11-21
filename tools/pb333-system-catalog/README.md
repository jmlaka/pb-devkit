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
