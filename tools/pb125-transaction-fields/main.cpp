// PowerBuilder 12.5 PBNI transaction property metadata probe.
// This program does NOT access or read the value of any field.
// No application RunApplication(), SQL CONNECT, or database calls are made.
//
// Build (MSVC x86 Developer Command Prompt):
// cl /nologo /EHsc /W4 /DUNICODE /D_UNICODE /I"<PBNI SDK>\include" main.cpp /Fe:pb125-transaction-fields.exe
//
// Run in an isolated, authorized Windows test environment:
// pb125-transaction-fields.exe <application-name> <absolute-path-to-app-library> <absolute-path-to-pbvm125.dll>

#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <tchar.h>
#include <iostream>
#include <string>
#include "pbni.h"

#ifdef UNICODE
#define OUT std::wcout
#else
#define OUT std::cout
#endif

namespace {
void help() {
    OUT << TEXT("Usage: pb125-transaction-fields.exe <application-name> ")
        << TEXT("<absolute-library-path> <absolute-pbvm125.dll-path>\n");
    OUT << TEXT("This only queries PowerBuilder type metadata; it never reads field values.\n");
}
}

int _tmain(int argc, TCHAR* argv[]) {
    if (argc != 4) {
        help();
        return 2;
    }

    const TCHAR* application_name=argv[1];
    const TCHAR* library_path=argv[2];
    const TCHAR* dll_path=argv[3];

    if (GetFileAttributes(library_path)==INVALID_FILE_ATTRIBUTES ||
        GetFileAttributes(dll_path)==INVALID_FILE_ATTRIBUTES) {
        OUT << TEXT("Library or PBVM DLL not found; supply full local paths.\n");
        return 2;
    }

    HMODULE module=LoadLibrary(dll_path);
    if (!module) {
        OUT << TEXT("Failed to load pbvm125.dll (Win32 error ") << GetLastError()
            << TEXT("). Check matching 32-bit architecture and runtime dependencies.\n");
        return 3;
    }

    typedef PBXRESULT (*GetVmProc)(IPB_VM**);
    FARPROC exported=GetProcAddress(module,"PB_GetVM");
    if (!exported) {
        OUT << TEXT("PB_GetVM export not found.\n");
        FreeLibrary(module);
        return 4;
    }

    GetVmProc getvm=reinterpret_cast<GetVmProc>(exported);
    IPB_VM* vm=nullptr;
    PBXRESULT status=getvm(&vm);
    if (status!=PBX_OK || !vm) {
        OUT << TEXT("PB_GetVM failed with code ") << status << TEXT(".\n");
        FreeLibrary(module);
        return 5;
    }

    IPB_Session* session=nullptr;
    LPCTSTR library_list[]={library_path};
    status=vm->CreateSession(application_name,library_list,1,&session);
    if (status!=PBX_OK || !session) {
        OUT << TEXT("CreateSession failed with code ") << status
            << TEXT(". Verify the application object name and its library dependencies.\n");
        FreeLibrary(module);
        return 6;
    }

    pbgroup system_group=session->GetSystemGroup();
    pbclass transaction_class=
        system_group ? session->FindClass(system_group,TEXT("transaction")) : nullptr;

    if (!transaction_class) {
        OUT << TEXT("Could not locate transaction class in system group.\n");
        session->Release();
        FreeLibrary(module);
        return 7;
    }

    OUT << TEXT("=== POWERBUILDER 12.5 TRANSACTION PROPERTY METADATA ===\n");
    OUT << TEXT("Only public property names and numeric field IDs; no field values.\n");
    OUT << TEXT("P-code indices may differ from PBNI field IDs: compare cautiously.\n\n");

    // Enumerate only a bounded range. PBNI returns NULL for invalid IDs.
    for (unsigned i=0;i<48;++i) {
        const pbfieldID id=static_cast<pbfieldID>(i);
        LPCTSTR name=session->GetFieldName(transaction_class,id);
        if (!name) continue;
        OUT << TEXT("PBNI field ID ") << i << TEXT(": ") << name << TEXT("\n");
        session->ReleaseString(name);
    }

    // Reverse lookup helps identify whether PBNI and P-code use matching IDs.
    const TCHAR* candidates[]={
        TEXT("DBMS"),TEXT("Database"),TEXT("DBParm"),TEXT("DBPass"),
        TEXT("LogID"),TEXT("LogPass"),TEXT("UserID"),TEXT("ServerName"),
        TEXT("AutoCommit"),TEXT("SQLCode"),TEXT("SQLDBCode"),
        TEXT("SQLErrText"),TEXT("SQLNRows"),TEXT("SQLReturnData"),
        TEXT("Lock")
    };
    OUT << TEXT("\n=== REVERSE LOOKUP FOR STANDARD TRANSACTION FIELDS ===\n");
    for (const TCHAR* candidate:candidates) {
        const pbfieldID id=session->GetFieldID(transaction_class,candidate);
        if (id==static_cast<pbfieldID>(0xFFFF)) continue;
        OUT << candidate << TEXT(" -> PBNI field ID ")
            << static_cast<unsigned>(id) << TEXT("\n");
    }

    session->Release();
    FreeLibrary(module);
    return 0;
}
