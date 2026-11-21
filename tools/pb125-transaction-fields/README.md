# PowerBuilder 12.5 transaction property index probe (optional)

This is a **metadata-only** Windows console tool intended to help correlate compiled
PowerBuilder P-code transaction member indices (for example `#2`, `#14`, `#15`)
with native PowerBuilder class property names.

It calls **only** PBNI metadata methods: `GetSystemGroup`, `FindClass`,
`GetFieldName`, and `GetFieldID`. It **never** requests field values,
passwords, connection strings, or database records. It uses `CreateSession`
to initialize the VM; it does **not** call `RunApplication` or `CONNECT`.
For safety, run it with an offline copy of the application in an isolated
authorized Windows environment, not in a live clinical workstation.

## Requirements

- Windows with the **32-bit PowerBuilder 12.5** runtime (`pbvm125.dll` and its dependencies).
- The PowerBuilder **12.5 PBNI SDK**, specifically the proprietary `pbni.h`
  header; it is not included in this repository.
- Visual Studio **x86 Native Tools Command Prompt** (MSVC C++ compiler).
- A local copy of the application library/EXE containing its application object.

## Build

In the x86 Native Tools Command Prompt, from this directory, substitute
the actual SDK include directory:

```bat
cl /nologo /EHsc /W4 /DUNICODE /D_UNICODE /I"C:\Path\To\PowerBuilder12.5\SDK\PBNI\include" main.cpp /Fe:pb125-transaction-fields.exe
```

The official PBNI 12.5 guide documents the `pbni.h` header and
`CreateSession`/system reflection APIs:

- https://documentation.help/PBNI/pbnigrefp8.htm
- https://documentation.help/PBNI/pbnigrefp233.htm
- https://documentation.help/PBNI/pbnigrefp150.htm
- https://documentation.help/PBNI/pbnigrefp129.htm

## Run (offline)

Pass the **application object name**, an absolute path to the local app
library or EXE, and an absolute path to `pbvm125.dll`:

```bat
pb125-transaction-fields.exe <application-object-name> "C:\Path\To\app.exe" "C:\Path\To\pbvm125.dll"
```

The application object name may differ from the EXE filename. If
`CreateSession` fails, validate the application object name and all
runtime/library dependencies rather than executing the application.

## Interpretation

The output contains:

- Native PBNI field IDs and the corresponding **property names**.
- Reverse lookups for the standard names `DBMS`, `Database`, `DBParm`,
  `DBPass`, `LogID`, `LogPass`, and others.

**Do not silently equate PBNI field IDs with compiled P-code member numbers.**
The internal representations could differ; compare with known properties
before associating compiled member `#15` with any property name.

The Rust scanner remains the main read-only tool for locating `CONNECT`
instructions. This helper is optional and should only be used if PBNI
headers and a safe Windows runtime environment are readily available.
