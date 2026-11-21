//! Read-only PowerBuilder 12.x P-code boundary verification.
//! Based on PbEntry.cs and PCodeParser110.cs from Hucxy/PbdViewer.
//! Default: tests instruction boundaries without printing operands or strings.
//! With a .fun name argument: prints a local, limited opcode/operand listing.
//!
//! Opcode-length table adapted from:
//! https://github.com/Hucxy/PbdViewer/blob/main/Uitils/PCode/PCodeParser110.cs
//! Copyright (c) 2022 Hucxy, MIT License.
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
//! THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use pb_devkit_core::pbl::PblParser;
use std::collections::BTreeMap;

const EXE: &str = "/home/juraj/Dev/promis/proamb.exe";

// An entry is the number of u16 operands AFTER the u16 opcode.
// Zero is a valid operand count; only an opcode >= 583 is unknown.
const OPERAND_WORDS: [u8; 583] = [
    0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 3, 3, 1, 3, 3,
    4, 0, 1, 5, 0, 3, 0, 3, 3, 0, 4, 3, 5, 4, 1, 1,
    2, 0, 0, 0, 0, 0, 0, 1, 0, 3, 2, 3, 4, 2, 3, 1,
    1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 2, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2,
    2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2,
    2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2,
    2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 1, 1, 1,
    1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 0, 0, 0, 0, 3, 3, 2, 2, 3, 3, 4, 4, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 2, 3, 2,
    3, 4, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 2, 0, 0,
    0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 5, 1, 4, 1, 0,
    2, 3, 3, 5, 3, 5, 1, 4, 2, 2, 2, 2, 2, 3, 3, 3,
    3, 0, 3, 0, 2, 2, 2, 3, 1, 1, 4, 3, 1, 1, 1, 0,
    0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 2,
    1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 3, 4, 3, 1, 1, 0,
    1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 2, 2, 1, 2, 2,
    2, 0, 0, 0, 0, 0, 0,
];

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn bytes(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self.pos.checked_add(n)
            .ok_or_else(|| format!("offset overflow at 0x{:X}", self.pos))?;
        let result = self.data.get(self.pos..end)
            .ok_or_else(|| format!(
                "out of bounds at 0x{:X}, need {} bytes; object has {}",
                self.pos, n, self.data.len()
            ))?;
        self.pos = end;
        Ok(result)
    }

    fn u16(&mut self) -> Result<u16, String> {
        let b = self.bytes(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    fn u32(&mut self) -> Result<u32, String> {
        let b = self.bytes(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn struct_buffer(&mut self) -> Result<(), String> {
        let size = self.u32()? as usize;
        let extra = self.u32()? as usize;
        self.bytes(size)?;
        self.bytes(extra)?;
        Ok(())
    }

    fn records20(&mut self) -> Result<usize, String> {
        self.bytes(6)?;
        self.struct_buffer()?;
        let bytes = self.u16()? as usize;
        if bytes % 20 != 0 {
            return Err(format!("20-byte table has {} bytes", bytes));
        }
        self.bytes(bytes)?;
        Ok(bytes / 20)
    }
}

fn word(buffer: &[u8], pos: usize) -> Result<usize, String> {
    let b = buffer.get(pos..pos + 2)
        .ok_or_else(|| format!("short object header at +{}", pos))?;
    Ok(u16::from_le_bytes([b[0], b[1]]) as usize)
}

#[derive(Default)]
struct ObjectResult {
    bodies: usize,
    bytes: usize,
    decoded_bodies: usize,
    decoded_instructions: usize,
    opcode_counts: BTreeMap<u16, usize>,
    problems: Vec<String>,
}

fn decode_pcode(code: &[u8]) -> Result<(usize, BTreeMap<u16, usize>), String> {
    let mut pos = 0usize;
    let mut count = 0usize;
    let mut frequencies = BTreeMap::new();

    while pos < code.len() {
        let pair = code.get(pos..pos + 2)
            .ok_or_else(|| format!("truncated opcode at +0x{:X}", pos))?;
        let opcode = u16::from_le_bytes([pair[0], pair[1]]);
        let operands = *OPERAND_WORDS.get(opcode as usize)
            .ok_or_else(|| format!(
                "opcode 0x{:04X} at +0x{:X} outside 583-entry table",
                opcode, pos
            ))? as usize;
        let next = pos + 2 + 2 * operands;
        if next > code.len() {
            return Err(format!(
                "opcode 0x{:04X} at +0x{:X} requires {} bytes, {} remain",
                opcode, pos, next - pos, code.len() - pos
            ));
        }

        *frequencies.entry(opcode).or_insert(0) += 1;
        count += 1;
        pos = next;
    }

    Ok((count, frequencies))
}

fn inspect(data: &[u8]) -> Result<ObjectResult, String> {
    let mut r = Cursor::new(data);
    if r.u16()? != 333 {
        return Err("not a version-333 compiled entry".into());
    }

    // Same verified PB12 entry layout as probe_pbentry.rs.
    r.bytes(22)?; // flag, kind, timestamps, unknowns: position now 0x18
    let descriptors = r.u16()? as usize;
    r.bytes(descriptors * 12)?;
    r.struct_buffer()?;
    r.records20()?; // global variables
    let objects = r.u16()? as usize;
    let extended_count = r.u16()? as usize;
    r.struct_buffer()?; // function name buffer
    r.struct_buffer()?; // parameter buffer
    r.records20()?; // types
    r.records20()?; // enums

    let indices = r.bytes(objects * 16)?;
    let extended = r.bytes(extended_count * 32)?;

    let mut used_extended = 0usize;
    let mut result = ObjectResult::default();

    for index in indices.chunks_exact(16) {
        match (index[0] >> 1) & 7 {
            0 => {
                let start = used_extended * 32;
                let object_header = extended.get(start..start + 32)
                    .ok_or_else(|| "missing extended object header".to_string())?;
                used_extended += 1;

                let functions = r.u16()? as usize;
                r.bytes(functions * 4)?; // index for function bodies

                for function_number in 0..functions {
                    let pcode_len = r.u16()? as usize;
                    let debug_count = r.u16()? as usize;
                    r.u16()?; // other function metadata
                    let pcode = r.bytes(pcode_len)?;
                    r.bytes(debug_count * 4)?;
                    r.records20()?; // function-local variables
                    r.struct_buffer()?; // stack / local data buffer

                    result.bodies += 1;
                    result.bytes += pcode_len;

                    match decode_pcode(pcode) {
                        Ok((instructions, frequencies)) => {
                            result.decoded_bodies += 1;
                            result.decoded_instructions += instructions;
                            for (opcode, frequency) in frequencies {
                                *result.opcode_counts.entry(opcode).or_insert(0) += frequency;
                            }
                        }
                        Err(message) => {
                            result.problems.push(format!(
                                "body {} ({} bytes): {}",
                                function_number, pcode_len, message
                            ));
                        }
                    }
                }

                r.bytes(word(object_header, 24)? * 6)?;
                r.bytes(word(object_header, 22)? * 4)?;
                r.records20()?; // referenced functions
                r.records20()?; // instance variables
                r.bytes(word(object_header, 28)? * 8)?;
                r.bytes(word(object_header, 26)? * 16)?;
                r.bytes(word(object_header, 4)? * 48)?;
            }
            1 => {
                r.bytes(word(index, 4)? * 8)?;
            }
            _ => {} // other kinds have no additional payload here
        }
    }

    if r.pos != data.len() {
        return Err(format!(
            "end mismatch: 0x{:X} vs 0x{:X}",
            r.pos, data.len()
        ));
    }

    Ok(result)
}


// Operation names from the PbdViewer decoder dispatch (not full decompilation).
// PbdViewer by Hucxy, Copyright (c) 2022, MIT License.
// https://github.com/Hucxy/PbdViewer
// PB333 delegates through PCodeParser110 -> 105 -> 100 -> 90.
fn opcode_action(op: u16) -> &'static str {
    match op {
        0x011f => "BeginAssignGlobalVariable",
        0x0122 => "BeginAssignInstanceVariable",
        0x011d | 0x0120 => "BeginAssignLocalVariable",
        0x011e => "BeginAssignSharedVariable",
        0x0189 | 0x018a | 0x018b | 0x018c | 0x018d | 0x018e | 0x018f | 0x0190 | 0x0191 | 0x0192 | 0x0193 | 0x0194 | 0x0195 | 0x0196 | 0x0197 | 0x0198 | 0x0199 | 0x019a | 0x019b | 0x019c | 0x019d | 0x019e | 0x019f | 0x01a1 | 0x01a2 | 0x01a3 | 0x01a4 | 0x01b8 | 0x01b9 | 0x01d3 | 0x01d4 | 0x01d5 | 0x01d6 | 0x01d7 | 0x01d8 | 0x01d9 | 0x01da | 0x01db | 0x01dc | 0x01dd | 0x01de | 0x01df | 0x01e0 | 0x01e1 | 0x01e2 | 0x01e3 | 0x01e4 | 0x020d | 0x0215 | 0x0216 | 0x0217 => "CallBuiltinFunction",
        0x002c | 0x014d | 0x014e | 0x0171 | 0x021b => "CallFunction",
        0x01bd | 0x01be | 0x01bf | 0x01c0 | 0x021c => "CallGlobalFunction",
        0x0013 => "CallSuper",
        0x003e | 0x003f | 0x0040 | 0x0041 | 0x0042 | 0x0043 | 0x0044 | 0x0045 | 0x0046 | 0x0047 | 0x0048 | 0x0049 | 0x004a | 0x004b | 0x004c | 0x004d | 0x004e | 0x004f | 0x0050 | 0x0051 | 0x0052 | 0x008d | 0x008e | 0x008f | 0x0090 | 0x0091 | 0x0092 | 0x0093 | 0x0094 | 0x0095 | 0x0096 | 0x0097 | 0x0098 | 0x0099 | 0x009a | 0x009b | 0x009c | 0x009d | 0x009e | 0x009f | 0x00a0 | 0x00a1 | 0x00a2 | 0x00a3 | 0x00a4 | 0x00a5 | 0x0123 | 0x0124 | 0x0125 | 0x0126 | 0x0127 | 0x0128 | 0x0129 | 0x012a | 0x012b | 0x012d | 0x012e | 0x012f | 0x0130 | 0x0131 | 0x0132 | 0x0133 | 0x0134 | 0x0135 | 0x0136 | 0x0137 | 0x0138 | 0x0139 | 0x013a | 0x013b | 0x013c | 0x013d | 0x013e | 0x0143 | 0x0144 | 0x0145 | 0x0146 | 0x015a | 0x015b | 0x015c | 0x015d | 0x015e | 0x015f | 0x0160 | 0x0161 | 0x0162 | 0x0163 | 0x0164 | 0x0165 | 0x0166 | 0x0167 | 0x0168 | 0x0169 | 0x016a | 0x016b | 0x016c | 0x0187 | 0x0188 | 0x01ab | 0x01ae | 0x01b6 | 0x01c9 | 0x01cb | 0x01cd | 0x01ce | 0x01cf | 0x01eb | 0x01ec | 0x01ed | 0x01ee | 0x01ef | 0x01f0 | 0x01f1 | 0x01f2 | 0x01f3 | 0x01f4 | 0x01f5 | 0x01f6 | 0x01f7 | 0x01f8 | 0x020c | 0x021f | 0x0220 | 0x0222 => "Cast",
        0x01e7 => "Catch",
        0x01b2 | 0x01b3 => "CreateArray",
        0x002d | 0x016d => "CreateObject",
        0x01c8 => "CreateObjectUsingName",
        0x0011 => "DestroyObject",
        0x007f | 0x0080 | 0x0081 | 0x0082 | 0x0083 | 0x0084 | 0x0085 | 0x0086 | 0x0087 | 0x0088 | 0x0089 | 0x008a | 0x008b | 0x008c | 0x0104 | 0x0105 | 0x0106 | 0x0107 | 0x0108 | 0x0109 | 0x010a | 0x010b | 0x010c | 0x010d | 0x010e | 0x010f | 0x0110 | 0x0111 | 0x0112 | 0x0113 | 0x0114 | 0x0115 | 0x0116 | 0x0117 | 0x0118 | 0x0159 | 0x0205 | 0x0207 | 0x0208 | 0x0209 => "EndAssign",
        0x00f6 | 0x00f7 | 0x00f8 | 0x00f9 | 0x00fa | 0x00fb | 0x00fc | 0x00fd | 0x00fe | 0x00ff | 0x0100 | 0x0101 | 0x0102 | 0x0103 | 0x01ba | 0x01bb | 0x020a | 0x020b => "EndAssign2",
        0x01e6 => "EndTry",
        0x01e9 => "EnterFinally",
        0x0012 => "Halt",
        0x01af => "Index",
        0x01b0 => "Index2",
        0x01b1 => "Index3",
        0x0002 | 0x0003 | 0x0004 => "Jump",
        0x01ea => "LeaveFinally",
        0x0028 | 0x0140 | 0x01c1 => "NoOp",
        0x0024 | 0x0025 | 0x0053 | 0x0054 | 0x0055 | 0x0056 | 0x0057 | 0x0058 | 0x0059 | 0x005a | 0x005b | 0x005c | 0x005d | 0x005e | 0x005f | 0x0060 | 0x0061 | 0x0062 | 0x0063 | 0x0064 | 0x0065 | 0x0066 | 0x0067 | 0x0068 | 0x0069 | 0x006a | 0x006b | 0x006c | 0x006d | 0x006e | 0x006f | 0x0070 | 0x0071 | 0x0072 | 0x0073 | 0x0074 | 0x0075 | 0x007d | 0x007e | 0x00a6 | 0x00a7 | 0x00a8 | 0x00a9 | 0x00aa | 0x00ab | 0x00ac | 0x00ad | 0x00ae | 0x00af | 0x00b0 | 0x00b1 | 0x00b2 | 0x00b3 | 0x00b4 | 0x00b5 | 0x00b6 | 0x00b7 | 0x00b8 | 0x00b9 | 0x00ba | 0x00bb | 0x00bc | 0x00bd | 0x00be | 0x00bf | 0x00c0 | 0x00c1 | 0x00c2 | 0x00c3 | 0x00c4 | 0x00c5 | 0x00c6 | 0x00c7 | 0x00c8 | 0x00c9 | 0x00ca | 0x00cb | 0x00cc | 0x00cd | 0x00ce | 0x00cf | 0x00d0 | 0x00d1 | 0x00d2 | 0x00d3 | 0x00d4 | 0x00d5 | 0x00d6 | 0x00d7 | 0x00d8 | 0x00d9 | 0x00da | 0x00db | 0x00dc | 0x00dd | 0x00de | 0x00df | 0x00e0 | 0x00e1 | 0x00e2 | 0x00e3 | 0x00e4 | 0x00e5 | 0x00e6 | 0x00e7 | 0x00e8 | 0x00e9 | 0x00ea | 0x00eb | 0x00ec | 0x00ed | 0x00ee | 0x00ef | 0x00f0 | 0x00f1 | 0x00f2 | 0x00f3 | 0x00f4 | 0x00f5 | 0x0177 | 0x0178 | 0x0179 | 0x017a | 0x017b | 0x017d | 0x017e | 0x017f | 0x0180 | 0x0181 | 0x0182 | 0x0183 | 0x0184 | 0x01f9 | 0x01fa | 0x01fb | 0x01fc | 0x01fd | 0x020f | 0x0210 | 0x0211 | 0x0212 | 0x0213 | 0x0214 => "OperateStack",
        0x0026 | 0x0076 | 0x0077 | 0x0078 | 0x0079 | 0x007a | 0x007b | 0x007c | 0x017c | 0x0185 | 0x01fe => "OperateStackSingle",
        0x0014 => "PopFunction",
        0x0032 | 0x0033 | 0x0034 | 0x0035 | 0x0036 | 0x0037 | 0x0038 | 0x0039 | 0x003a | 0x003b | 0x003c | 0x01ff => "PushConstant",
        0x003d => "PushEnum",
        0x01bc => "PushGlobalFunctionName",
        0x0031 => "PushGlobalSharedVariable",
        0x002f | 0x0153 | 0x0154 | 0x0174 | 0x01a6 | 0x0201 => "PushGlobalVariable",
        0x0027 | 0x0141 | 0x0142 | 0x0186 | 0x01d0 | 0x021e => "PushInstanceVariable",
        0x0020 | 0x01d2 => "PushInstanceVariableName",
        0x001e | 0x0030 | 0x014f | 0x0150 | 0x0172 | 0x01a9 | 0x0200 => "PushLocalVariable",
        0x0022 => "PushParent",
        0x001f | 0x0151 | 0x0152 | 0x0173 | 0x01aa | 0x0203 => "PushSharedVariable",
        0x0021 => "PushThis",
        0x0119 => "ResetAssign",
        0x0000 | 0x0001 => "Return",
        0x0009 => "SqlClose",
        0x0019 => "SqlDescribe",
        0x000b | 0x000c | 0x000f | 0x001b => "SqlDirectInsertUpdateDelete",
        0x0010 | 0x001a => "SqlDirectSelect",
        0x000d => "SqlExecute",
        0x0018 => "SqlExecuteDynamic",
        0x01c4 => "SqlExecuteDynamicDescriptor",
        0x01c3 => "SqlExecuteImmediate",
        0x0015 => "SqlExecuteSqlsa",
        0x000e => "SqlFetch",
        0x01c5 => "SqlFetchDynamicDescriptor",
        0x000a => "SqlOpen",
        0x0017 => "SqlOpenDynamic",
        0x01c6 => "SqlOpenDynamicDescriptor",
        0x0005 | 0x0006 | 0x0007 | 0x0008 => "SqlOperateTransaction",
        0x0016 => "SqlPrepareSqlsa",
        0x01e8 => "Throw",
        0x01e5 => "Try",
        _ => "Unmapped",
    }
}


// These annotations are intentionally narrow and correspond to specific PB333
// opcode cases from PbdViewer's version conversion and PCodeParser90/105:
// 0x0034 -> PushConstant((int32) operand), 0x0001 -> Return(flag),
// 0x0000 -> Return(void), 0x0002..0x0004 -> jump family.
fn operand_u16(bytes: &[u8]) -> Option<u16> {
    let pair = bytes.get(..2)?;
    Some(u16::from_le_bytes([pair[0], pair[1]]))
}

fn operand_i32(bytes: &[u8]) -> Option<i32> {
    let quad = bytes.get(..4)?;
    Some(i32::from_le_bytes([quad[0], quad[1], quad[2], quad[3]]))
}

fn opcode_note(opcode: u16, operands: &[u8]) -> String {
    match opcode {
        0x0000 => "return (no value)".into(),
        0x0005 => "CONNECT USING transaction object".into(),
        0x0006 => "COMMIT USING transaction object".into(),
        0x0007 => "ROLLBACK USING transaction object".into(),
        0x0008 => "DISCONNECT USING transaction object".into(),
        0x0001 => match operand_u16(operands) {
            Some(1) => "return value from stack".into(),
            Some(0) => "return (no value)".into(),
            Some(n) => format!("return flag={n} (unknown meaning)"),
            None => "truncated return flag".into(),
        },
        0x0002 | 0x0003 | 0x0004 => match operand_u16(operands) {
            Some(target) => {
                let condition = match opcode {
                    0x0002 => "if true",
                    0x0003 => "if false",
                    _ => "unconditional",
                };
                format!("jump {} to byte offset 0x{:04X}", condition, target)
            }
            None => "truncated jump target".into(),
        },
        0x0034 => match operand_i32(operands) {
            Some(value) => format!("signed 32-bit integer literal: {value}"),
            None => "truncated integer constant".into(),
        },
        _ => String::new(),
    }
}

// Only claim a source-level preview when the *entire* body matches a known
// stack pattern, optionally followed by the compiler's unreachable epilogue.
// Do not generalize to unknown control flow or object/function resolution.
fn simple_constant_return(code: &[u8]) -> Option<i32> {
    if code.len() != 10 && code.len() != 16 {
        return None;
    }
    if operand_u16(code.get(0..2)?)? != 0x0034
        || operand_u16(code.get(6..8)?)? != 0x0001
        || operand_u16(code.get(8..10)?)? != 1
    {
        return None;
    }

    if code.len() == 16 {
        // The jump following the return is unreachable; its sole target is
        // the terminal zero-operand return at offset 0x000E.
        if operand_u16(code.get(10..12)?)? != 0x0004
            || operand_u16(code.get(12..14)?)? != 0x000E
            || operand_u16(code.get(14..16)?)? != 0x0000
        {
            return None;
        }
    }

    operand_i32(code.get(2..6)?)
}


// Name records and strings follow PbVariable.cs and BufferHelper.GetString()
// (Hucxy/PbdViewer, MIT). Offsets into string pools are byte offsets.
#[derive(Clone)]
struct VariableSymbol {
    name: String,
    global_index: Option<u16>,
    type_id: u16,
}

struct FunctionSymbols {
    code: Vec<u8>,
    stack_buffer: Vec<u8>,
    locals: Vec<VariableSymbol>,
    globals: Vec<VariableSymbol>,
    referenced_functions: Vec<String>,
    sql_pool: Vec<u8>,
    type_names: Vec<String>,
}

fn pool_buffer(r: &mut Cursor<'_>) -> Result<Vec<u8>, String> {
    let size = r.u32()? as usize;
    let extra = r.u32()? as usize;
    let bytes = r.bytes(size)?.to_vec();
    r.bytes(extra)?;
    Ok(bytes)
}

fn utf16_string(pool: &[u8], offset: u32) -> Option<String> {
    let begin = (offset & 0x7fff_ffff) as usize;
    let mut words = Vec::new();
    let mut pos = begin;
    for _ in 0..4096 {
        let unit = pool.get(pos..pos.checked_add(2)?)?;
        let ch = u16::from_le_bytes([unit[0], unit[1]]);
        if ch == 0 {
            return String::from_utf16(&words).ok();
        }
        words.push(ch);
        pos += 2;
    }
    None
}

fn named_variables(r: &mut Cursor<'_>) -> Result<Vec<VariableSymbol>, String> {
    r.bytes(6)?;
    let names = pool_buffer(r)?;
    let len = r.u16()? as usize;
    if len % 20 != 0 {
        return Err(format!("variable record size {} is not divisible by 20", len));
    }
    let records = r.bytes(len)?;
    Ok(records.chunks_exact(20).enumerate().map(|(i, rec)| {
        let offset = u32::from_le_bytes([rec[8], rec[9], rec[10], rec[11]]);
        let name = utf16_string(&names, offset)
            .filter(|s| !s.chars().any(char::is_control))
            .unwrap_or_else(|| format!("<variable#{i}>"));
        let global_index = if rec[17] & 2 != 0 {
            Some(u16::from_le_bytes([rec[12], rec[13]]))
        } else { None };
        let type_id = u16::from_le_bytes([rec[18], rec[19]]);
        VariableSymbol { name, global_index, type_id }
    }).collect())
}


// Type records use the same 6-byte prefix, name pool and 20-byte records
// as PbEntry.ReadTypes(). Type IDs 0x4xxx refer to external/system types;
// 0x8xxx refer to entries in this local table.
fn named_types(r: &mut Cursor<'_>) -> Result<Vec<String>, String> {
    r.bytes(6)?;
    let pool=pool_buffer(r)?;
    let len=r.u16()? as usize;
    if len % 20 != 0 {
        return Err(format!("type table size {len} not divisible by 20"));
    }
    let records=r.bytes(len)?;
    Ok(records.chunks_exact(20).enumerate().map(|(i,rec)| {
        let off=u32::from_le_bytes([rec[8],rec[9],rec[10],rec[11]]);
        utf16_string(&pool,off)
            .filter(|name|!name.is_empty() && !name.chars().any(char::is_control))
            .unwrap_or_else(||format!("<type#{i}>"))
    }).collect())
}

fn describe_type_id(type_id: u16, local_types: &[String]) -> String {
    match type_id & 0xF000 {
        0x4000 => format!("system type 0x{type_id:04X} (system catalog required for name)"),
        0x8000 => {
            let index=(type_id & 0x3FFF) as usize;
            match local_types.get(index) {
                Some(name)=>format!("local type 0x{type_id:04X} ({name})"),
                None=>format!("local type 0x{type_id:04X} (name unresolved)")
            }
        }
        0xC000 => format!("unknown/external type 0x{type_id:04X}"),
        _ =>format!("primitive type 0x{type_id:04X}")
    }
}

fn named_references(r: &mut Cursor<'_>) -> Result<Vec<String>, String> {
    // PbdViewer ReadReferencedFunctions(): 6 bytes, name pool,
    // u16 byte count, then 20-byte records (name offset at +8).
    r.bytes(6)?;
    let pool = pool_buffer(r)?;
    let len = r.u16()? as usize;
    if len % 20 != 0 {
        return Err(format!("referenced-function table size {} not divisible by 20",len));
    }
    let records = r.bytes(len)?;
    Ok(records.chunks_exact(20).enumerate().map(|(i,rec)| {
        let offset = u32::from_le_bytes([rec[8],rec[9],rec[10],rec[11]]);
        utf16_string(&pool,offset)
            .filter(|name| !name.is_empty() && !name.chars().any(char::is_control))
            .unwrap_or_else(|| format!("<unresolved-function-ref#{i}>"))
    }).collect())
}

// PbdViewer BufferHelper.GetCursor walks a link at +8 until 0xFFFF.
// A terminal cursor record has the template offset at +24 and the list of
// (start, end) UTF-16 *code unit* placeholder ranges at +16.
// We use the ranges, never blind string replacement, so repeated parameter
// names and non-ASCII text retain their original meaning.
fn sql_cursor_record(sym: &FunctionSymbols, args: &[u8]) -> Option<usize> {
    let raw = args.get(0..4)?;
    let mut off = (u32::from_le_bytes([raw[0],raw[1],raw[2],raw[3]])
        & 0x7fff_ffff) as usize;
    for _ in 0..32 {
        let next = sym.sql_pool.get(off.checked_add(8)?..off.checked_add(12)?)?;
        let ptr = u32::from_le_bytes([next[0],next[1],next[2],next[3]]);
        if ptr == 65535 {
            return Some(off);
        }
        off = (ptr & 0x7fff_ffff) as usize;
    }
    None
}

fn sql_template(sym: &FunctionSymbols, args: &[u8]) -> Option<String> {
    let off = sql_cursor_record(sym,args)?;
    let pos = sym.sql_pool.get(off.checked_add(24)?..off.checked_add(28)?)?;
    let text_offset = u32::from_le_bytes([pos[0],pos[1],pos[2],pos[3]]);
    utf16_string(&sym.sql_pool,text_offset)
}

fn sql_info(sym: &FunctionSymbols, args: &[u8], reveal: bool) -> String {
    match sql_template(sym,args) {
        Some(text) if reveal => {
            let escaped=text.replace('\n'," ").replace('\r'," ");
            let mut trimmed: String=escaped.chars().take(600).collect();
            if escaped.chars().count()>600 { trimmed.push_str("..."); }
            trimmed
        }
        Some(text) => format!("<SQL template hidden, {} chars; use --strings>",text.chars().count()),
        None => "<SQL template not resolved>".into(),
    }
}

fn substitute_sql_ranges(
    template: &str,
    pool: &[u8],
    positions: usize,
    params: &[String],
) -> Result<String, String> {
    let units: Vec<u16> = template.encode_utf16().collect();
    let mut out: Vec<u16> = Vec::with_capacity(units.len() + params.len() * 12);
    let mut consumed = 0usize;
    for (i,param) in params.iter().enumerate() {
        let record=positions.checked_add(i.checked_mul(4)
            .ok_or("parameter table offset overflow")?)
            .ok_or("parameter table offset overflow")?;
        let pair=pool.get(record..record.checked_add(4)
            .ok_or("parameter table range overflow")?)
            .ok_or("parameter table exceeds cursor buffer")?;
        let start=u16::from_le_bytes([pair[0],pair[1]]) as usize;
        let end=u16::from_le_bytes([pair[2],pair[3]]) as usize;
        if start==0 && end==0 {
            return Err(format!("placeholder table ended before parameter {}",i+1));
        }
        if start<consumed || end<start || end>units.len() {
            return Err(format!(
                "invalid placeholder span {}..{} for parameter {} (UTF-16 length {})",
                start,end,i+1,units.len()
            ));
        }
        out.extend_from_slice(&units[consumed..start]);
        out.extend(format!(":{param}").encode_utf16());
        consumed=end;
    }
    out.extend_from_slice(&units[consumed..]);
    String::from_utf16(&out).map_err(|_| "invalid UTF-16 placeholder substitution".into())
}

fn sql_with_bound_params(
    sym: &FunctionSymbols,
    args: &[u8],
    params: &[String],
    reveal: bool,
) -> String {
    if !reveal { return sql_info(sym,args,false); }
    let template=match sql_template(sym,args) {
        Some(value)=>value,
        None=>return "<SQL template not resolved>".into(),
    };
    if params.is_empty() {return template;}
    let param_position=sql_cursor_record(sym,args).and_then(|off|{
        let pos=sym.sql_pool.get(off.checked_add(16)?..off.checked_add(20)?)?;
        // PB333 marks in-buffer offsets with 0x80000000; it is not part
        // of the address (same convention as BufferHelper.GetCursor).
        Some((u32::from_le_bytes([pos[0],pos[1],pos[2],pos[3]]) & 0x7FFF_FFFF) as usize)
    });
    match param_position {
        Some(pos)=>match substitute_sql_ranges(&template,&sym.sql_pool,pos,params) {
            Ok(bound)=>bound,
            Err(err)=>format!("/* SQL placeholder substitution refused: {err} */ {template}"),
        },
        None=>format!("/* SQL placeholder table not found */ {template}"),
    }
}

// Diagnostic-only metadata about SQL cursor parameter tables.
// Prints offsets and sizes, never template text, parameter values or byte dumps.
fn print_sql_cursor_diagnostics(sym: &FunctionSymbols) -> Result<(), String> {
    let lines=flow_instructions(&sym.code)?;
    println!("\n=== SQL CURSOR METADATA (NO SQL TEXT) ===");
    println!("Variable buffer length: {} bytes",sym.sql_pool.len());
    for ins in lines.iter().filter(|x| matches!(x.opcode,0x0010|0x000f)) {
        let name=if ins.opcode==0x0010 {"SELECT"} else {"DML"};
        let params=flow_operand_word(&ins.operands,4);
        let into=if ins.opcode==0x0010 {flow_operand_word(&ins.operands,6)} else {0};
        println!("\n0x{:04X} {}: bind parameters={}, INTO variables={}",
            ins.position,name,params,into);
        let Some(cursor)=sql_cursor_record(sym,&ins.operands) else {
            println!("  cursor record: not resolved");
            continue;
        };
        println!("  terminal cursor offset: 0x{:08X}",cursor);
        let mut read_u32=|delta:usize| -> Option<u32> {
            let p=cursor.checked_add(delta)?;
            let b=sym.sql_pool.get(p..p.checked_add(4)?)?;
            Some(u32::from_le_bytes([b[0],b[1],b[2],b[3]]))
        };
        let link=read_u32(8);
        let raw_param=read_u32(16);
        let template=read_u32(24);
        println!("  link (+8): {:?}; param ptr (+16): {:?}; text ptr (+24): {:?}",
            link.map(|x|format!("0x{x:08X}")),
            raw_param.map(|x|format!("0x{x:08X}")),
            template.map(|x|format!("0x{x:08X}")));
        if let Some(pointer)=raw_param {
            for (label,pos) in [
                ("raw",pointer as usize),
                ("high-bit-masked",(pointer&0x7FFF_FFFF) as usize)
            ] {
                let room=pos.checked_add(params.saturating_mul(4))
                    .map(|end|end<=sym.sql_pool.len()).unwrap_or(false);
                println!("  candidate {label}: 0x{pos:08X}, {}",
                    if room {"fits buffer"} else {"outside buffer"});
                if pos.checked_add(4).map(|end|end<=sym.sql_pool.len())
                    .unwrap_or(false)
                {
                    let b=&sym.sql_pool[pos..pos+4];
                    let a=u16::from_le_bytes([b[0],b[1]]);
                    let z=u16::from_le_bytes([b[2],b[3]]);
                    println!("    first span: {}..{} (UTF-16 code units)",a,z);
                }
            }
        }
        let size=sql_template(sym,&ins.operands)
            .map(|x|x.encode_utf16().count());
        println!("  SQL template length (UTF-16 units): {:?}",size);
    }
    Ok(())
}

fn sql_with_into(mut statement: String, destinations: &[String], reveal: bool) -> String {
    if !reveal {return statement;}
    let lower=statement.to_ascii_lowercase();
    if let Some(pos)=lower.find(" from ") {
        let insertion=format!(" INTO :{}",destinations.join(", :"));
        statement.insert_str(pos,&insertion);
        statement
    } else {
        format!("/* SELECT INTO not inserted: FROM not found */ {statement}")
    }
}

fn find_pcode_symbols(data: &[u8], selected: usize) -> Result<FunctionSymbols, String> {
    let mut r = Cursor::new(data);
    if r.u16()? != 333 {
        return Err("not a version-333 compiled entry".into());
    }
    r.bytes(22)?;
    let descriptor_count = r.u16()? as usize;
    r.bytes(descriptor_count * 12)?;
    let sql_pool = pool_buffer(&mut r)?;
    let globals = named_variables(&mut r)?;
    let object_count = r.u16()? as usize;
    let extended_count = r.u16()? as usize;
    r.struct_buffer()?;
    r.struct_buffer()?;
    let type_names=named_types(&mut r)?;
    r.records20()?;

    let indices = r.bytes(object_count * 16)?;
    let extended = r.bytes(extended_count * 32)?;
    let mut used_extended = 0usize;
    let mut function_index = 0usize;

    for index in indices.chunks_exact(16) {
        match (index[0] >> 1) & 7 {
            0 => {
                let p = used_extended * 32;
                let header = extended.get(p..p + 32)
                    .ok_or_else(|| "missing extended header".to_string())?;
                used_extended += 1;
                let bodies = r.u16()? as usize;
                r.bytes(bodies * 4)?;
                let mut selected_data: Option<FunctionSymbols> = None;
                for _ in 0..bodies {
                    let code_len = r.u16()? as usize;
                    let debug_count = r.u16()? as usize;
                    r.u16()?;
                    let code = r.bytes(code_len)?.to_vec();
                    r.bytes(debug_count * 4)?;

                    if function_index == selected {
                        let locals = named_variables(&mut r)?;
                        let stack_buffer = pool_buffer(&mut r)?;
                        selected_data = Some(FunctionSymbols {
                            code, stack_buffer, locals,
                            globals: globals.clone(), referenced_functions: Vec::new(),
                            sql_pool: sql_pool.clone(),
                            type_names: type_names.clone(),
                        });
                    } else {
                        r.records20()?;
                        r.struct_buffer()?;
                    }
                    function_index += 1;
                }

                r.bytes(word(header, 24)? * 6)?;
                r.bytes(word(header, 22)? * 4)?;
                let referenced_functions = named_references(&mut r)?;
                if let Some(mut data) = selected_data {
                    data.referenced_functions = referenced_functions;
                    return Ok(data);
                }
                r.records20()?;
                r.bytes(word(header, 28)? * 8)?;
                r.bytes(word(header, 26)? * 16)?;
                r.bytes(word(header, 4)? * 48)?;
            }
            1 => { r.bytes(word(index, 4)? * 8)?; }
            _ => {}
        }
    }
    Err(format!("function body index {} not found", selected))
}

fn annotated_opcode(
    opcode: u16,
    operands: &[u8],
    symbols: &FunctionSymbols,
    show_strings: bool,
) -> String {
    match opcode {
        0x001e | 0x0030 | 0x01a9 | 0x0150 | 0x011d | 0x0120 => {
            let Some(index) = operand_u16(operands) else { return "missing variable index".into() };
            match symbols.locals.get(index as usize) {
                Some(v) => format!("local [{}]: {}", index, v.name),
                None => format!("local [{}]: unresolved", index),
            }
        }
        0x002f => {
            let Some(index) = operand_u16(operands) else { return "missing variable index".into() };
            let found = symbols.locals.iter()
                .chain(symbols.globals.iter())
                .find(|v| v.global_index == Some(index))
                .map(|v| v.name.as_str());
            match found {
                Some(name) => format!("global [{}]: {}", index, name),
                None => format!("global [{}]: unresolved", index),
            }
        }
        0x003b => {
            if operands.len() < 4 { return "missing string offset".into(); }
            let offset = u32::from_le_bytes([
                operands[0], operands[1], operands[2], operands[3]
            ]);
            match utf16_string(&symbols.stack_buffer, offset) {
                Some(value) => {
                    if show_strings {
                        let escaped = value.escape_debug().to_string();
                        let truncated: String = escaped.chars().take(160).collect();
                        let suffix = if escaped.chars().count() > 160 { "..." } else { "" };
                        format!("string literal: \"{}{}\"", truncated, suffix)
                    } else {
                        format!("string literal, {} chars [hidden; use --strings]", value.chars().count())
                    }
                }
                None => format!("string offset 0x{:X} could not be decoded", offset),
            }
        }
        0x01a4 => "builtin Upper(value)".into(),
        0x01bc => {
            let idx = flow_operand_word(operands,0);
            let object_id = flow_operand_word(operands,2);
            if object_id & 0x8000 != 0 {
                match symbols.referenced_functions.get(idx) {
                    Some(name) => format!("function reference [{idx}]: {name}"),
                    None => format!("unresolved function reference [{idx}]")
                }
            } else if object_id & 0x4000 != 0 {
                format!("system function reference: object=0x{object_id:04X}, index={idx} (name requires system catalog)")
            } else {
                format!("object function reference: object=0x{object_id:04X}, index={idx}")
            }
        },
        0x0010 | 0x000f => sql_info(symbols,operands,show_strings),
        0x0123..=0x012b | 0x012d..=0x013e =>
            "stack-neutral cast (PbdViewer verified)".into(),
        0x0195 => "isnull(value)".into(),
        0x00a8 => "comparison: =".into(),
        0x0025 => "logical OR".into(),
        0x00f8 => "increment variable (++)".into(),
        0x0005 => "SQL CONNECT".into(),
        0x0006 => "SQL COMMIT".into(),
        0x0007 => "SQL ROLLBACK".into(),
        0x0008 => "SQL DISCONNECT".into(),
        _ => opcode_note(opcode, operands),
    }
}


// A deliberately limited, branch-aware stack translator. It produces a
// pseudocode/control-flow outline, NOT guaranteed compilable PowerScript.
// The simulator starts each basic block with an empty stack: any state that
// must flow across blocks remains unresolved rather than being invented.
#[derive(Clone)]
struct FlowInsn {
    position: usize,
    end: usize,
    opcode: u16,
    operands: Vec<u8>,
}

fn flow_instructions(code: &[u8]) -> Result<Vec<FlowInsn>, String> {
    let mut result = Vec::new();
    let mut pos = 0usize;
    while pos < code.len() {
        let raw = code.get(pos..pos + 2)
            .ok_or_else(|| format!("truncated opcode at 0x{pos:X}"))?;
        let opcode = u16::from_le_bytes([raw[0], raw[1]]);
        let words = *OPERAND_WORDS.get(opcode as usize)
            .ok_or_else(|| format!("unknown opcode 0x{opcode:04X}"))? as usize;
        let end = pos + 2 + 2 * words;
        let operands = code.get(pos + 2..end)
            .ok_or_else(|| format!("truncated operands at 0x{pos:X}"))?
            .to_vec();
        result.push(FlowInsn { position: pos, end, opcode, operands });
        pos = end;
    }
    Ok(result)
}

fn flow_pop(stack: &mut Vec<String>) -> String {
    stack.pop().unwrap_or_else(|| "<missing-stack-value>".to_string())
}

fn flow_pop_n(stack: &mut Vec<String>, count: usize) -> Vec<String> {
    if count > stack.len() {
        stack.clear();
        return vec!["<missing-stack-values>".into()];
    }
    stack.split_off(stack.len() - count)
}

fn flow_local(sym: &FunctionSymbols, operand: &[u8]) -> String {
    match operand_u16(operand).map(|x| x as usize) {
        Some(i) => sym.locals.get(i)
            .map(|v| v.name.clone())
            .unwrap_or_else(|| format!("local[{i}]")),
        None => "<invalid-local>".into(),
    }
}

fn flow_global(sym: &FunctionSymbols, operand: &[u8]) -> String {
    match operand_u16(operand) {
        Some(i) => sym.locals.iter().chain(sym.globals.iter())
            .find(|v| v.global_index == Some(i))
            .map(|v| v.name.clone())
            .unwrap_or_else(|| format!("global[{i}]")),
        None => "<invalid-global>".into(),
    }
}

fn flow_operand_word(raw: &[u8], start: usize) -> usize {
    raw.get(start..start + 2).and_then(operand_u16)
        .unwrap_or(0) as usize
}

fn flow_field_name(sym: &FunctionSymbols, operand: &[u8]) -> String {
    let offset = operand_u16(operand).unwrap_or(0) as usize;
    if offset >= 1 && offset <= 7 {
        return "entryobject".into();
    }
    let location = sym.stack_buffer.get(offset..offset + 4);
    if let Some(bytes) = location {
        let name_offset = u32::from_le_bytes([bytes[0],bytes[1],bytes[2],bytes[3]]);
        if let Some(name) = utf16_string(&sym.stack_buffer,name_offset) {
            if !name.is_empty() && name.len() <= 128
                && name.chars().all(|c| !c.is_control())
            {
                return name;
            }
        }
    }
    format!("field[0x{offset:X}]")
}

fn flow_statement(
    ins: &FlowInsn,
    sym: &FunctionSymbols,
    stack: &mut Vec<String>,
    reveal_strings: bool,
) -> Option<String> {
    let op = ins.opcode;
    let p = &ins.operands;
    match op {
        0x001e | 0x0030 | 0x01a9 | 0x0150 | 0x011d | 0x0120 => {
            stack.push(flow_local(sym, p));
            None
        }
        0x002f | 0x011f => {
            stack.push(flow_global(sym, p));
            None
        }
        0x0034 => {
            let value = operand_i32(p).map(|v| v.to_string())
                .unwrap_or_else(|| "<invalid-integer>".into());
            stack.push(value);
            None
        }
        0x003b => {
            let value = p.get(..4)
                .and_then(|b| {
                    let offset = u32::from_le_bytes([b[0],b[1],b[2],b[3]]);
                    utf16_string(&sym.stack_buffer,offset)
                });
            let value = match value {
                Some(s) if reveal_strings => {
                    let mut escaped = s.replace('~',"~~")
                        .replace('"',"~\"")
                        .replace('\n',"~n")
                        .replace('\r',"~r");
                    if escaped.len() > 200 {
                        escaped = escaped.chars().take(200).collect();
                        escaped.push_str("...");
                    }
                    format!("\"{escaped}\"")
                }
                Some(_) => "<string-hidden>".into(),
                None => "<string-offset-unresolved>".into(),
            };
            stack.push(value);
            None
        }
        // PCodeParser110 index 0x01A4 -> 105 case 422 -> 100 case
        // 420 -> PCodeParser90 upper(). A unary builtin string operation.
        0x01a4 => {
            let value=flow_pop(stack);
            stack.push(format!("Upper({value})"));
            None
        }
        // PB333 0x0199 -> PB105 410 -> PB100 409 -> PB90 407.
        // PCodeParser90 defines case 407 as Len(value).
        0x0199 => {
            let value=flow_pop(stack);
            stack.push(format!("Len({value})"));
            None
        }
        // PB333 0x005C -> PB105 92 -> PB100 91 -> PB90 89,
        // defined as a binary subtraction.
        0x005c => {
            let rhs=flow_pop(stack);
            let lhs=flow_pop(stack);
            stack.push(format!("({lhs} - {rhs})"));
            None
        }
        0x0195 => {
            let x = flow_pop(stack);
            stack.push(format!("IsNull({x})"));
            None
        }
        0x00a8 | 0x00ad => {
            let b=flow_pop(stack);
            let a=flow_pop(stack);
            stack.push(format!("({a} = {b})"));
            None
        }
        // PB333 0x0055 -> PB105 85 -> PB100 84 -> PB90 82: addition.
        0x0055 | 0x007d => {
            let b=flow_pop(stack);
            let a=flow_pop(stack);
            stack.push(format!("({a} + {b})"));
            None
        }
        // PB333 0x00C8 -> PB105 200 -> PB100 199 -> PB90 197: >.
        // PB333 0x00EC -> PB105 236 -> PB100 235 -> PB90 233: <=.
        0x00c8 | 0x00ec => {
            let b=flow_pop(stack);
            let a=flow_pop(stack);
            stack.push(format!("({a} {} {b})",
                if op==0x00c8 {">"} else {"<="}));
            None
        }
        0x00b8 => {
            let b=flow_pop(stack);
            let a=flow_pop(stack);
            stack.push(format!("({a} <> {b})"));
            None
        }
        0x0025 => {
            let b=flow_pop(stack);
            let a=flow_pop(stack);
            stack.push(format!("({a} OR {b})"));
            None
        }
        // PB333 0x0123..=0x012B map to PCodeParser90 cases 288..296:
        // Cast(0). 0x012D..=0x013E map to cases 298..315: Cast(type).
        // Both are stack-neutral (PCodeParserBase.Cast does nothing).
        // 0x012C maps to case 297, which has no confirmed handling.
        0x003f | 0x008e | 0x0139 | 0x0123..=0x012b | 0x012d..=0x013e => None,
        0x0080 | 0x0082 | 0x0088 => {
            let value=flow_pop(stack);
            let variable=flow_pop(stack);
            Some(format!("{variable} = {value}"))
        }
        0x00f8 => {
            let variable=flow_pop(stack);
            Some(format!("{variable}++"))
        }
        0x0020 => {
            stack.push(flow_field_name(sym,p));
            None
        }
        0x0027 | 0x0122 => {
            let field=flow_pop(stack);
            let object=flow_pop(stack);
            stack.push(if field=="entryobject" {
                object
            } else {
                format!("{object}.{field}")
            });
            None
        }
        0x0010 => {
            let sqlca=flow_pop(stack);
            let params=flow_pop_n(stack,flow_operand_word(p,4));
            let destinations=flow_pop_n(stack,flow_operand_word(p,6));
            let query=sql_with_into(
                sql_with_bound_params(sym,p,&params,reveal_strings),
                &destinations,reveal_strings
            );
            if reveal_strings {
                Some(format!("{} USING {}",query,sqlca))
            } else {
                Some(format!(
                    "SQL SELECT {} INTO :{} USING {}; INPUTS: {}",
                    query,destinations.join(", :"),sqlca,params.join(", ")
                ))
            }
        }
        0x000f => {
            let sqlca=flow_pop(stack);
            let params=flow_pop_n(stack,flow_operand_word(p,4));
            let query=sql_with_bound_params(sym,p,&params,reveal_strings);
            if reveal_strings {
                Some(format!("{} USING {}",query,sqlca))
            } else {
                Some(format!(
                    "SQL DML {} USING {}; PARAMETERS: {}",
                    query,sqlca,params.join(", ")
                ))
            }
        }
        0x0005 | 0x0006 | 0x0007 | 0x0008 => {
            let transaction=flow_pop(stack);
            let verb=match op {
                0x0005=>"CONNECT",
                0x0006=>"COMMIT",
                0x0007=>"ROLLBACK",
                _=>"DISCONNECT"
            };
            Some(format!("{verb} USING {transaction}"))
        }
        0x01bc => {
            let function_index=flow_operand_word(p,0);
            // For object ref 0x8001, this index is an entry in the
            // referenced-functions table. The actual name is not yet resolved.
            let object_id=flow_operand_word(p,2);
            let name=if object_id & 0x8000 != 0 {
                sym.referenced_functions.get(function_index).cloned()
            } else { None };
            stack.push(match name {
                Some(name) => name,
                None if object_id & 0x4000 != 0 => format!(
                    "<system-function object=0x{object_id:04X} index={function_index}>"
                ),
                None => format!("<unresolved-global-function index={function_index}>"),
            });
            None
        }
        0x01bd => {
            let function=flow_pop(stack);
            let params=flow_pop_n(stack,flow_operand_word(p,2));
            stack.push(format!("{function}({})",params.join(", ")));
            None
        }
        0x0014 => {
            Some(format!("CALL {}",flow_pop(stack)))
        }
        0x0000 => Some("RETURN".into()),
        0x0001 => {
            let flag=flow_operand_word(p,0);
            if flag == 1 {Some(format!("RETURN {}",flow_pop(stack)))}
            else {Some("RETURN".into())}
        }
        0x0002 | 0x0003 => {
            let condition=flow_pop(stack);
            let target=flow_operand_word(p,0);
            if op==0x0003 {
                Some(format!("IF NOT {condition} GOTO L{target:04X}"))
            } else {
                Some(format!("IF {condition} GOTO L{target:04X}"))
            }
        }
        0x0004 => {
            let target=flow_operand_word(p,0);
            Some(format!("GOTO L{target:04X}"))
        }
        _ => {
            // Do not guess its stack effect; warn and abandon any pending
            // expressions in the current basic block.
            stack.clear();
            Some(format!(
                "/* unknown stack semantics: 0x{:04X} {} */",
                op, opcode_action(op)
            ))
        }
    }
}

fn print_flow(sym: &FunctionSymbols, reveal_strings: bool)
    -> Result<(), String>
{
    use std::collections::{BTreeMap, BTreeSet};
    let lines = flow_instructions(&sym.code)?;
    let mut by_offset = BTreeMap::<usize, usize>::new();
    for (i, ins) in lines.iter().enumerate() {
        by_offset.insert(ins.position,i);
    }
    let mut starts = BTreeSet::new();
    starts.insert(0usize);
    for ins in &lines {
        if matches!(ins.opcode,0x0002|0x0003|0x0004) {
            let target=flow_operand_word(&ins.operands,0);
            if !by_offset.contains_key(&target) {
                return Err(format!(
                    "jump at 0x{:04X} targets 0x{:04X}, not an instruction boundary",
                    ins.position,target
                ));
            }
            starts.insert(target);
            if ins.end < sym.code.len() { starts.insert(ins.end); }
        } else if matches!(ins.opcode,0x0000|0x0001) && ins.end < sym.code.len() {
            starts.insert(ins.end);
        }
    }
    let ordered: Vec<usize> = starts.into_iter().collect();
    let mut blocks = Vec::new();
    for (i,start) in ordered.iter().enumerate() {
        let end=ordered.get(i+1).copied().unwrap_or(sym.code.len());
        blocks.push((*start,end));
    }

    let mut reachable = BTreeSet::new();
    let mut todo = vec![0usize];
    while let Some(start) = todo.pop() {
        if !reachable.insert(start) { continue; }
        let Some((_,end))=blocks.iter().copied().find(|(b,_)| *b==start)
        else { return Err(format!("block at 0x{start:X} not found")); };
        let Some(last)=lines.iter().filter(|l| l.position>=start && l.end<=end).last()
        else { continue; };
        match last.opcode {
            0x0000 | 0x0001 => {}
            0x0004 => { todo.push(flow_operand_word(&last.operands,0)); }
            0x0002 | 0x0003 => {
                todo.push(flow_operand_word(&last.operands,0));
                if end < sym.code.len() { todo.push(end); }
            }
            _ => {
                if end < sym.code.len() {todo.push(end);}
            }
        }
    }

    println!("\n=== CONSERVATIVE CONTROL-FLOW OUTLINE ===");
    println!("Blocks: {} | reachable: {} | unreachable: {}",
        blocks.len(), reachable.len(), blocks.len()-reachable.len());
    println!("Warning: per-block stack is reset; system function names need an external catalog.");
    for (start,end) in blocks {
        let annotation=if reachable.contains(&start) {""} else {" [UNREACHABLE]"};
        println!("\nL{start:04X}:{annotation}  // 0x{start:04X}..0x{end:04X}");
        let mut stack=Vec::new();
        for ins in lines.iter().filter(|l| l.position>=start && l.end<=end) {
            if let Some(stmt)=flow_statement(ins,sym,&mut stack,reveal_strings) {
                println!("    {stmt}    // 0x{:04X}",ins.position);
            }
        }
        if !stack.is_empty() {
            println!("    /* {} expression(s) remain on stack */",stack.len());
        }
    }
    Ok(())
}


// Forward-only control-flow structurer. Deliberately refuses to structure
// reachable unconditional jumps or backward branches, rather than emitting
// invalid PowerScript. Expected to work with the simple branch/return CFG
// in w_f_zapis_vykon_neplateny_cash.fun.
#[derive(Clone, Copy, Debug)]
struct StructuredBlock {
    start: usize,
    end: usize,
}

#[derive(Clone, Copy)]
enum BranchScope {
    Join(usize),        // IF...END IF closes at shared successor
    Else(usize),        // IF...ELSE changes arm at second successor
    ElseBody,           // END IF after both branches terminate
}

fn flow_has_path(
    successors: &std::collections::BTreeMap<usize, Vec<usize>>,
    start: usize,
    target: usize,
) -> bool {
    let mut visited=std::collections::BTreeSet::new();
    let mut todo=vec![start];
    while let Some(here)=todo.pop() {
        if here==target {return true;}
        if !visited.insert(here) {continue;}
        if let Some(next)=successors.get(&here) {
            todo.extend(next.iter().copied());
        }
    }
    false
}

// A deliberately narrow loop recognizer: only straight-line loop bodies
// with a conditional guard at their header and a single backward GOTO.
// All non-loop or ambiguous jumps still use the existing refusal path.
fn simple_structured_loop(
    blocks:&[StructuredBlock],
    terminal:&std::collections::BTreeMap<usize,FlowInsn>,
    backedge:StructuredBlock,
) -> Option<(usize,usize)> {
    let jump=terminal.get(&backedge.start)?;
    if jump.opcode!=0x0004 {return None;}
    let header_start=flow_operand_word(&jump.operands,0);
    if header_start>=backedge.start {return None;}
    let header=blocks.iter().find(|block|block.start==header_start)?;
    let guard=terminal.get(&header_start)?;
    if guard.opcode!=0x0003 || header.end>backedge.start ||
        flow_operand_word(&guard.operands,0)!=backedge.end
    {return None;}
    // No branch into, out of, or within the body except its final backedge.
    // Such cases require a more general CFG structurer.
    if blocks.iter().filter(|b|b.start>header_start &&
        b.start<backedge.start).any(|b|matches!(
            terminal.get(&b.start).map(|ins|ins.opcode),
            Some(0x0000..=0x0004)
        ))
    {return None;}
    // Require all intermediate blocks to form an uninterrupted fallthrough.
    let region=blocks.iter().filter(|b|b.start>=header_start &&
        b.start<=backedge.start).collect::<Vec<_>>();
    if region.len()<2 || region.windows(2).any(|pair|pair[0].end!=pair[1].start)
    {return None;}
    Some((header_start,backedge.end))
}

fn print_structured(sym: &FunctionSymbols, reveal_strings: bool)
    -> Result<(), String>
{
    use std::collections::{BTreeMap,BTreeSet};
    let lines=flow_instructions(&sym.code)?;
    if lines.is_empty() {
        println!("\n=== STRUCTURED POWERSCRIPT OUTLINE ===\n/* empty P-code */");
        return Ok(());
    }

    let instructions: BTreeSet<usize>=lines.iter().map(|i|i.position).collect();
    let mut starts=BTreeSet::new();
    starts.insert(0usize);
    for ins in &lines {
        if matches!(ins.opcode,0x0002|0x0003|0x0004) {
            let target=flow_operand_word(&ins.operands,0);
            if !instructions.contains(&target) {
                return Err(format!("invalid branch target 0x{:04X}",target));
            }
            starts.insert(target);
            if ins.end < sym.code.len() {starts.insert(ins.end);}
        } else if matches!(ins.opcode,0x0000|0x0001) && ins.end < sym.code.len() {
            starts.insert(ins.end);
        }
    }
    let labels: Vec<_>=starts.into_iter().collect();
    let blocks: Vec<_>=labels.iter().enumerate().map(|(i,&start)|{
        StructuredBlock {
            start,
            end: labels.get(i+1).copied().unwrap_or(sym.code.len())
        }
    }).collect();

    let mut successors=BTreeMap::<usize,Vec<usize>>::new();
    let mut terminal=BTreeMap::<usize,FlowInsn>::new();
    for block in &blocks {
        let last=lines.iter().filter(|ins|
            ins.position>=block.start && ins.end<=block.end
        ).last().ok_or_else(||format!("empty block at 0x{:X}",block.start))?;
        terminal.insert(block.start,last.clone());
        let next=match last.opcode {
            0x0000 | 0x0001 => Vec::new(),
            0x0004 => vec![flow_operand_word(&last.operands,0)],
            0x0002 | 0x0003 => {
                let mut value=vec![flow_operand_word(&last.operands,0)];
                if block.end<sym.code.len() {value.push(block.end);}
                value
            }
            _ if block.end < sym.code.len()=>vec![block.end],
            _ => Vec::new()
        };
        successors.insert(block.start,next);
    }

    let mut reachable=BTreeSet::new();
    let mut todo=vec![0usize];
    while let Some(start)=todo.pop() {
        if !reachable.insert(start) {continue;}
        let children=successors.get(&start)
            .ok_or_else(||format!("missing basic block at 0x{:X}",start))?;
        todo.extend(children.iter().copied());
    }
    let active: Vec<_>=blocks.into_iter()
        .filter(|b| reachable.contains(&b.start)).collect();
    let mut loop_headers=BTreeMap::<usize,(usize,usize)>::new();
    let mut loop_backedges=BTreeMap::<usize,usize>::new();
    for block in &active {
        let last=terminal.get(&block.start)
            .ok_or("missing block terminator")?;
        if last.opcode==0x0004 {
            let Some((header,exit))=simple_structured_loop(&active,&terminal,*block)
            else {
                return Err(format!(
                    "unstructured unconditional goto at 0x{:04X}; use --flow",
                    last.position
                ));
            };
            if loop_headers.insert(header,(block.start,exit)).is_some() {
                return Err("multiple backedges per loop not supported; use --flow".into());
            }
            loop_backedges.insert(block.start,header);
        }
        if matches!(last.opcode,0x0002|0x0003) {
            let target=flow_operand_word(&last.operands,0);
            if target<=block.end || !reachable.contains(&target) {
                return Err(format!(
                    "backward or unstructured branch at 0x{:04X}; use --flow",
                    last.position
                ));
            }
        }
    }

    println!("\n=== STRUCTURED POWERSCRIPT OUTLINE (EXPERIMENTAL) ===");
    println!("Reachable basic blocks: {} / {}",active.len(),successors.len());
    println!("Pseudocode; unresolved SQL text is hidden and function types are not inferred.");
    let mut scopes=Vec::<BranchScope>::new();
    let mut indent=0usize;
    for block in &active {
        // Closing a nested IF may reveal the start of the ELSE arm
        // of an enclosing IF at the very same block boundary.
        loop {
            let Some(scope)=scopes.last().copied() else {break;};
            match scope {
                BranchScope::Join(target) if target==block.start => {
                    scopes.pop();
                    indent=indent.saturating_sub(1);
                    println!("{}END IF", "    ".repeat(indent));
                }
                BranchScope::Else(target) if target==block.start => {
                    indent=indent.saturating_sub(1);
                    println!("{}ELSE", "    ".repeat(indent));
                    indent+=1;
                    *scopes.last_mut().expect("scope exists")=BranchScope::ElseBody;
                }
                _ => break,
            }
        }

        let mut stack=Vec::new();
        let in_block=lines.iter()
            .filter(|ins|ins.position>=block.start && ins.end<=block.end);
        for ins in in_block {
            if matches!(ins.opcode,0x0002|0x0003) {
                let condition=flow_pop(&mut stack);
                let positive=if ins.opcode==0x0003 {
                    condition
                } else {
                    format!("NOT ({condition})")
                };
                let false_target=flow_operand_word(&ins.operands,0);
                if let Some((_,exit))=loop_headers.get(&block.start) {
                    if *exit!=false_target {
                        return Err("loop exit does not match guard; use --flow".into());
                    }
                    println!("{}DO WHILE {}", "    ".repeat(indent),positive);
                    indent+=1;
                    continue;
                }
                let is_join=flow_has_path(&successors,block.end,false_target);
                // Only one open ELSE branch is supported so far;
                // this avoids silently emitting misnested scopes.
                if !is_join && scopes.iter().any(|s|
                    matches!(s,BranchScope::Else(_)|BranchScope::ElseBody))
                {
                    return Err(format!(
                        "nested else detected at 0x{:04X}; use --flow",
                        ins.position
                    ));
                }
                println!("{}IF {} THEN", "    ".repeat(indent),positive);
                scopes.push(if is_join {
                    BranchScope::Join(false_target)
                } else {
                    BranchScope::Else(false_target)
                });
                indent+=1;
            } else if ins.opcode==0x0004 {
                if loop_backedges.get(&block.start)
                    !=Some(&flow_operand_word(&ins.operands,0)) {
                    return Err("unstructured unconditional jump".into());
                }
                indent=indent.saturating_sub(1);
                println!("{}LOOP", "    ".repeat(indent));
            } else if let Some(stmt)=flow_statement(ins,sym,&mut stack,reveal_strings) {
                println!("{}{}", "    ".repeat(indent),stmt);
            }
        }
        if !stack.is_empty() {
            println!("{}/* {} unresolved stack expression(s) */",
                "    ".repeat(indent),stack.len());
        }
    }

    while let Some(scope)=scopes.pop() {
        match scope {
            BranchScope::ElseBody => {
                indent=indent.saturating_sub(1);
                println!("{}END IF","    ".repeat(indent));
            }
            BranchScope::Join(target) | BranchScope::Else(target) => {
                return Err(format!(
                    "unclosed branch at target L{target:04X}; use --flow"
                ));
            }
        }
    }
    Ok(())
}


// Identify compiled PowerBuilder database connection sites without reading
// credentials or displaying string literals. PB333 0x0005 maps through
// PCodeParser110->105->100->90 to SqlOperateTransaction("connect").
fn scan_db_connections(parser: &PblParser) -> Result<(), Box<dyn std::error::Error>> {
    const SUFFIXES: &[&str] = &[".fun", ".win", ".apl", ".udo"];
    let mut seen=0usize;
    let mut validated=0usize;
    let mut skipped=0usize;
    let mut connect_entries=0usize;
    let mut connection_sites=0usize;
    let mut filename_candidates=Vec::new();
    println!("=== PB333 DATABASE CONNECT LOCATION SCAN ===");
    println!("Target opcode: 0x0005 = CONNECT USING <transaction>");
    println!("No literal strings, parameter values or credentials will be printed.\n");
    for entry in parser.entries().iter()
        .filter(|e| SUFFIXES.iter().any(|ext| e.name.to_ascii_lowercase().ends_with(*ext)))
    {
        seen += 1;
        let name=entry.name.to_ascii_lowercase();
        if ["connect","login","prihlas","overenie_prihlas","post_login"]
            .iter().any(|part|name.contains(part))
        {
            filename_candidates.push(entry.name.clone());
        }
        let data=match parser.parse_chunk_chain(entry.data_offset,entry.size) {
            Ok(bytes)=>bytes,
            Err(_)=>{skipped+=1;continue;}
        };
        let inspection=match inspect(&data) {
            Ok(result) if result.problems.is_empty()
                && result.decoded_bodies==result.bodies => result,
            _=>{skipped+=1;continue;}
        };
        validated+=1;
        let n=inspection.opcode_counts.get(&0x0005).copied().unwrap_or(0);
        if n==0 {continue;}
        connect_entries+=1;
        println!("CONNECT ENTRY: {} ({} opcode occurrence(s), {} bodies)",
                 entry.name,n,inspection.bodies);
        for body_index in 0..inspection.bodies {
            let sym=match find_pcode_symbols(&data,body_index) {
                Ok(sym)=>sym,
                Err(err)=>{
                    println!("  body {}: unable to resolve symbols: {}",body_index,err);
                    continue;
                }
            };
            let lines=flow_instructions(&sym.code)?;
            for (i,ins) in lines.iter().enumerate().filter(|(_,x)|x.opcode==0x0005) {
                connection_sites+=1;
                let target=match i.checked_sub(1).and_then(|j|lines.get(j)) {
                    Some(previous) if previous.opcode==0x002f =>
                        flow_global(&sym,&previous.operands),
                    Some(previous) if matches!(previous.opcode,0x001e|0x0030|0x01a9|0x0150) =>
                        flow_local(&sym,&previous.operands),
                    _=>"<transaction expression not yet reconstructed>".into()
                };
                println!("  body={} P-code offset=0x{:04X} CONNECT USING {}",
                    body_index,ins.position,target);
                let fields: std::collections::BTreeSet<String>=lines.iter()
                    .filter(|x|x.opcode==0x0020)
                    .map(|x|flow_field_name(&sym,&x.operands))
                    .filter(|field| {
                        ["dbms","dbparm","database","servername","userid",
                         "dbpass","logid","logpass","autocommit","sqlcode"]
                        .contains(&field.to_ascii_lowercase().as_str())
                    }).collect();
                if !fields.is_empty() {
                    println!("  referenced transaction properties: {}",
                        fields.into_iter().collect::<Vec<_>>().join(", "));
                }
            }
        }
    }
    println!("\nSCAN SUMMARY: entries considered={}, validated={}, skipped={}",
        seen,validated,skipped);
    println!("Entries with CONNECT={}, located CONNECT sites={}",connect_entries,connection_sites);
    println!("\nFILENAME CANDIDATES (may implement application rather than DB login):");
    filename_candidates.sort();
    filename_candidates.dedup();
    for candidate in filename_candidates.iter().take(50) {
        println!("  {candidate}");
    }
    if connect_entries==0 {
        println!("No direct CONNECT opcode found in parsed entries. It may be located");
        println!("in an unsupported object type or implemented through external code.");
    }
    println!("\nTo inspect a candidate without showing strings:");
    println!("  cargo run --release -p pb-devkit-cli --example probe_pcode -- f_connect.fun 0 120 --flow");
    Ok(())
}

// Trace credential *provenance* at the CONNECT site without outputting
// secrets, user names, connection strings, or literal values. Does not
// perform authentication or modify PowerBuilder/ODBC configuration.
// Report metadata of a PB333 field-name descriptor, never its value.
// PbdViewer PushInstanceVariableName: u32 at offset 0, member index u16
// at offset +4. A low word 0xFFFF means the name is implied by the
// associated object type rather than present as a UTF-16 string.
fn field_descriptor_metadata(sym: &FunctionSymbols, operands: &[u8]) -> String {
    let Some(offset) = operand_u16(operands).map(usize::from) else {
        return "missing field descriptor".into();
    };
    if (1..=7).contains(&offset) { return "entry-object marker".into(); }
    let Some(raw) = sym.stack_buffer.get(offset..offset.saturating_add(6)) else {
        return format!("field descriptor at 0x{offset:04X}: unavailable");
    };
    let word = u32::from_le_bytes([raw[0],raw[1],raw[2],raw[3]]);
    let index = u16::from_le_bytes([raw[4],raw[5]]);
    if word & 0xffff == 0xffff {
        format!("type-indexed member=#{index}, name requires transaction type catalog")
    } else {
        let pointer=word & 0x7fff_ffff;
        format!("inline-name pointer=0x{pointer:08X}, member index=#{index}")
    }
}

fn referenced_function_at(sym: &FunctionSymbols, ins: &FlowInsn) -> Option<String> {
    if ins.opcode != 0x01bc { return None; }
    let index=flow_operand_word(&ins.operands,0);
    let object_id=flow_operand_word(&ins.operands,2);
    if object_id & 0x8000 == 0 { return None; }
    Some(sym.referenced_functions.get(index)
        .cloned().unwrap_or_else(||format!("function-ref#{index}")))
}

// Resolve an index-only member name using the external PB333 system catalog.
// Only trust the catalog when the target object has a local type explicitly
// named "transaction". Do not apply a transaction field index to other types.
fn catalog_transaction_property(
    sym:&FunctionSymbols,
    op:&FlowInsn,
    descriptor:&FlowInsn,
    objects:&[SystemCatalogObject],
) -> Option<String> {
    if !matches!(op.opcode,0x001e|0x0030|0x01a9|0x0150) ||
        descriptor.opcode!=0x0020 {return None;}
    let local=flow_operand_word(&op.operands,0);
    let var=sym.locals.get(local)?;
    let name=if var.type_id&0xf000==0x8000 {
        sym.type_names.get((var.type_id&0x3fff) as usize)?
    } else {return None;};
    if !name.eq_ignore_ascii_case("transaction") {return None;}
    let descriptor_offset=operand_u16(&descriptor.operands)? as usize;
    let record=sym.stack_buffer.get(descriptor_offset..descriptor_offset.checked_add(6)?)?;
    let raw_ptr=u32::from_le_bytes([record[0],record[1],record[2],record[3]]);
    if raw_ptr&0xffff != 0xffff {return None;}
    let member=u16::from_le_bytes([record[4],record[5]]) as usize;
    let transaction=objects.iter().find(|x|x.name.eq_ignore_ascii_case("transaction"))?;
    system_property_name(objects,transaction.type_id,member,0)
}

// List only a controlled vocabulary of DBParm configuration key *names*.
// Never print arbitrary text from the application, even for near matches.
// This is heuristic: keys fragmented across literals may not be detected.
fn known_dbparm_keys(sym:&FunctionSymbols, instructions:&[FlowInsn]) -> Vec<&'static str> {
    const KEYS:&[&str]=&[
        "ConnectString","DSN","UID","PWD","UserID","Password",
        "DatabaseName","DBN","ENG","CommLinks","Host","Port",
        "AutoStop","DelimitIdentifier","IsolationLevel","SQLCA",
    ];
    let mut known=std::collections::BTreeSet::<&str>::new();
    for ins in instructions.iter().filter(|ins|ins.opcode==0x003b) {
        let Some(raw)=ins.operands.get(..4) else {continue;};
        let offset=u32::from_le_bytes([raw[0],raw[1],raw[2],raw[3]]);
        let Some(value)=utf16_string(&sym.stack_buffer,offset) else {continue;};
        let low=value.to_ascii_lowercase();
        for key in KEYS {
            let needle=key.to_ascii_lowercase();
            for (index,_) in low.match_indices(&needle) {
                let prefix=low[..index].chars().last();
                let after=low[index+needle.len()..].trim_start();
                // Require a syntactic assignment, and no preceding name
                // character. No value or substring from the source is emitted.
                if (prefix.is_none() || prefix.is_some_and(|c|
                    !c.is_ascii_alphanumeric() && c!='_')) &&
                    after.starts_with('=')
                {known.insert(*key);}
            }
        }
    }
    known.into_iter().collect()
}

// Determine the immediate SQLCode guard following a PB333 CONNECT.
// Only exact patterns are accepted: a transaction field compared with integer
// zero and then a conditional branch. Never infer success/failure from a jump
// unless the field is confirmed as SQLCode in a version-matched type catalog.
#[derive(Debug)]
struct SqlCodeGuard {
    branch_offset: usize,
    field_name: String,
    zero_dest: usize,
    nonzero_dest: usize,
}

fn connect_sqlcode_guard(
    sym:&FunctionSymbols, lines:&[FlowInsn], connect_idx:usize,
    catalog:Option<&[SystemCatalogObject]>,
) -> Option<SqlCodeGuard> {
    let window=lines.get(connect_idx+1..connect_idx+7)?;
    let [object,field,get_field,zero,compare,branch]=window else {return None;};
    if !matches!(object.opcode,0x001e|0x0030|0x01a9|0x0150) ||
        field.opcode!=0x0020 ||
        !matches!(get_field.opcode,0x0027|0x0122) ||
        zero.opcode!=0x0034 ||
        operand_i32(&zero.operands)!=Some(0) ||
        !matches!(compare.opcode,0x00b8|0x00a8|0x00ad) ||
        !matches!(branch.opcode,0x0002|0x0003) {return None;}
    let field_name=catalog.and_then(|objects|
        catalog_transaction_property(sym,object,field,objects))?;
    if !field_name.eq_ignore_ascii_case("sqlcode") {return None;}
    // 0x0002 = branch when true, 0x0003 = branch when false.
    // 0x00B8 = <>, 0x00A8/0x00AD = =, both against zero.
    let jump_dest=flow_operand_word(&branch.operands,0);
    let fallthrough_dest=branch.end;
    let (true_dest,false_dest)=if branch.opcode==0x0002 {
        (jump_dest,fallthrough_dest)
    } else {(fallthrough_dest,jump_dest)};
    let (nonzero_dest,zero_dest)=if compare.opcode==0x00b8 {
        (true_dest,false_dest)
    } else {(false_dest,true_dest)};
    Some(SqlCodeGuard {
        branch_offset:branch.position,field_name,
        zero_dest,nonzero_dest,
    })
}

// Conservative static reachability from one point until the NEXT database
// CONNECT or RETURN. Conditions not modeled are explored in both directions.
// These endpoints are possible paths, not a claim about runtime execution.
fn next_connect_endpoints(
    lines:&[FlowInsn],start:usize,
) -> Vec<String> {
    use std::collections::{BTreeMap,BTreeSet};
    let by_offset:BTreeMap<usize,&FlowInsn>=lines.iter()
        .map(|instruction|(instruction.position,instruction)).collect();
    let mut visited=BTreeSet::new();
    let mut endpoints=BTreeSet::<String>::new();
    let mut todo=vec![start];
    while let Some(offset)=todo.pop() {
        if !visited.insert(offset) {continue;}
        let Some(ins)=by_offset.get(&offset) else {
            endpoints.insert(format!("end/unresolved at 0x{offset:04X}"));
            continue;
        };
        match ins.opcode {
            0x0005=>{
                endpoints.insert(format!("CONNECT at 0x{:04X}",ins.position));
            },
            0x0000|0x0001=>{
                endpoints.insert(format!("RETURN at 0x{:04X}",ins.position));
            },
            0x0004=>{
                todo.push(flow_operand_word(&ins.operands,0));
            },
            0x0002|0x0003=>{
                todo.push(flow_operand_word(&ins.operands,0));
                todo.push(ins.end);
            },
            _=>todo.push(ins.end),
        }
    }
    endpoints.into_iter().collect()
}

// Reachability before the first CONNECT (or RETURN), starting at function entry.
// This deliberately stops at CONNECT; it never treats a later retry as an
// independently selected entry path. Both unknown outcomes of conditional
// jumps remain possible, and no runtime credential values are inspected.
fn preconnect_branch_gates(lines:&[FlowInsn])
    -> Vec<(usize,usize,Vec<String>,Vec<String>)>
{
    use std::collections::{BTreeMap,BTreeSet};
    let by_offset:BTreeMap<usize,&FlowInsn>=lines.iter()
        .map(|ins|(ins.position,ins)).collect();
    let mut visited=BTreeSet::new();
    let mut gates=Vec::new();
    let mut todo=vec![0usize];
    while let Some(offset)=todo.pop() {
        if !visited.insert(offset) {continue;}
        let Some(ins)=by_offset.get(&offset) else {continue;};
        match ins.opcode {
            0x0000|0x0001|0x0005 => {},
            0x0004=>todo.push(flow_operand_word(&ins.operands,0)),
            0x0002|0x0003=>{
                let jump=flow_operand_word(&ins.operands,0);
                let fall=ins.end;
                let jumps=next_connect_endpoints(lines,jump);
                let falls=next_connect_endpoints(lines,fall);
                if jumps!=falls && (jumps.iter().chain(falls.iter())
                    .any(|x|x.starts_with("CONNECT at ")))
                {
                    gates.push((ins.position,jump,jumps,falls));
                }
                todo.push(jump);
                todo.push(fall);
            }
            _=>todo.push(ins.end),
        }
    }
    gates.sort_by_key(|(offset,_,_,_)|*offset);
    gates
}

// A single-block, redacted symbolic reconstruction of a branch condition.
// Reconstruct only operations from the nearest basic-block boundary, refusing
// to guess a value that arrives from a predecessor block. Numeric and string
// literals are both suppressed. No call is executed; only opcode metadata is
// read. The full condition is shown only when the stack is fully resolved.
// PB333 primary connection selector: a local receives Upper(parameter)
// and is compared with two string literals using equality + false jump.
// The instruction template is checked explicitly; nothing is inferred from
// merely nearby instructions. Constants and their offsets are not displayed.
fn pb333_connection_mode_predicate(
    lines:&[FlowInsn], sym:&FunctionSymbols, branch_position:usize
) -> Option<String> {
    let branch_index=lines.iter().position(|ins|ins.position==branch_position)?;
    if branch_index<3 {return None;}
    let compare=&lines[branch_index-1];
    let value=&lines[branch_index-2];
    let literal=&lines[branch_index-3];
    let branch=&lines[branch_index];
    if branch.opcode!=0x0003 ||
        compare.opcode!=0x00ad ||
        value.opcode!=0x001e ||
        literal.opcode!=0x003b {return None;}
    let selector_idx=flow_operand_word(&value.operands,0);
    let selector=sym.locals.get(selector_idx)?;
    // Verify a direct assignment of Upper(local) to the selector variable.
    // Prefer the closest assignment before this comparison, avoiding a
    // mistaken mapping to an old overwritten value.
    let mut definitions=lines.windows(4)
        .filter(|w|w[0].position<branch_position &&
            w[0].opcode==0x011d &&
            flow_operand_word(&w[0].operands,0)==selector_idx &&
            w[1].opcode==0x001e &&
            w[2].opcode==0x01a4 &&
            matches!(w[3].opcode,0x0080|0x0082|0x0088));
    let def=definitions.next_back()?;
    let source_idx=flow_operand_word(&def[1].operands,0);
    let source=sym.locals.get(source_idx)?;
    if source.type_id!=0x0006 || selector.type_id!=0x0006 {return None;}
    if source.name.is_empty() || source.name.chars().any(|c|
        !c.is_ascii_alphanumeric() && c!='_') {return None;}
    Some(format!("(Upper({}) = <string-hidden>)",source.name))
}

fn redacted_entry_gate_predicate(
    lines:&[FlowInsn], sym:&FunctionSymbols, branch_position:usize
) -> String {
    use std::collections::BTreeSet;
    if let Some(predicate)=pb333_connection_mode_predicate(lines,sym,branch_position) {
        return predicate;
    }
    let mut boundaries=BTreeSet::new();
    boundaries.insert(0usize);
    for ins in lines {
        if matches!(ins.opcode,0x0002|0x0003|0x0004) {
            boundaries.insert(flow_operand_word(&ins.operands,0));
        }
        if matches!(ins.opcode,0x0000..=0x0004) {
            boundaries.insert(ins.end);
        }
    }
    let Some(begin)=boundaries.range(..=branch_position).next_back().copied()
        else {return "<unresolved: no basic block boundary>".into();};
    let mut stack=Vec::<String>::new();
    let mut unsupported=false;
    let mut inspected=false;
    for ins in lines.iter()
        .filter(|ins|ins.position>=begin && ins.position<=branch_position) {
        if ins.position==branch_position {
            if !matches!(ins.opcode,0x0002|0x0003) {
                return "<unresolved: target is not a conditional branch>".into();
            }
            inspected=true;
            break;
        }
        if ins.opcode==0x0034 {
            stack.push("<number-hidden>".into());
            continue;
        }
        if let Some(stmt)=flow_statement(ins,sym,&mut stack,false) {
            if stmt.starts_with("/* unknown stack semantics") {
                unsupported=true;
            }
        }
    }
    if !inspected || unsupported || stack.len()!=1 {
        return format!("<unresolved predicate; block 0x{begin:04X}..0x{branch_position:04X}>");
    }
    let condition=stack.pop().unwrap_or_default();
    if condition.contains("<missing-stack-value>") ||
        condition.contains("<string-offset-unresolved>") ||
        condition.contains("<invalid-") {
        return format!("<partially unresolved; block 0x{begin:04X}..0x{branch_position:04X}>");
    }
    condition
}

// A strictly redacted, instruction-level diagnostic for unresolved login
// routing conditions. Unlike --strings or raw disassembly, this deliberately
// NEVER emits literal text, literal numbers, stack-buffer bytes, or unknown
// instruction operand data. It is safe to share as a metadata report.
fn gate_symbol_name(name:&str) -> &str {
    if name.len()<=64 && !name.is_empty() &&
        name.chars().all(|c|c.is_ascii_alphanumeric() || c=='_')
    {name} else {"<unnamed>"}
}

// Opt-in local inspection of compiled literals; no credential execution.
fn sensitive_literal_at(sym:&FunctionSymbols, ins:&FlowInsn) -> Option<String> {
    match ins.opcode {
        0x003b=>{
            let raw=ins.operands.get(..4)?;
            let offset=u32::from_le_bytes([raw[0],raw[1],raw[2],raw[3]]);
            let value=utf16_string(&sym.stack_buffer,offset)?;
            Some(format!("string literal = \"{}\"",value.escape_debug()))
        },
        0x0034=>operand_i32(&ins.operands)
            .map(|v|format!("integer literal = {v}")),
        _=>None,
    }
}

fn diagnostic_literal_info(
    sym:&FunctionSymbols,ins:&FlowInsn,show_sensitive:bool
) -> String {
    if show_sensitive {
        sensitive_literal_at(sym,ins).unwrap_or_else(||
            "literal (decoder unavailable for this opcode)".into())
    } else {
        "literal <redacted> (content and numeric value suppressed)".into()
    }
}

fn redacted_gate_instruction(sym:&FunctionSymbols, ins:&FlowInsn,
    show_sensitive:bool) -> String {
    let op=ins.opcode;
    match op {
        0x001e|0x0030|0x014f|0x0150|0x0172|0x01a9|0x0200|
        0x011d|0x0120 => {
            let idx=flow_operand_word(&ins.operands,0);
            match sym.locals.get(idx) {
                Some(v)=>format!("local[{idx}] {} : {}",
                    gate_symbol_name(&v.name),
                    describe_type_id(v.type_id,&sym.type_names)),
                None=>format!("local[{idx}] <unresolved>"),
            }
        },
        0x002f|0x011f|0x0153|0x0154|0x0174|0x01a6|0x0201=>{
            let index=flow_operand_word(&ins.operands,0) as u16;
            let var=sym.locals.iter().chain(sym.globals.iter())
                .find(|v|v.global_index==Some(index));
            match var {
                Some(v)=>format!("global[{index}] {} : {}",
                    gate_symbol_name(&v.name),
                    describe_type_id(v.type_id,&sym.type_names)),
                None=>format!("global[{index}] <unresolved>"),
            }
        },
        0x0032..=0x003c | 0x01ff =>
            diagnostic_literal_info(sym,ins,show_sensitive),
        0x0002|0x0003|0x0004=>{
            let dest=flow_operand_word(&ins.operands,0);
            let form=if op==0x0002 {"if true"}
                else if op==0x0003 {"if false"}
                else {"unconditional"};
            format!("jump {form} -> 0x{dest:04X}")
        },
        0x01bc=>{
            let index=flow_operand_word(&ins.operands,0);
            let object_id=flow_operand_word(&ins.operands,2);
            if object_id&0x8000!=0 {
                let name=sym.referenced_functions.get(index)
                    .map(|value|gate_symbol_name(value)).unwrap_or("<unresolved>");
                format!("referenced function {name} (ref #{index})")
            } else {
                format!("system/object function 0x{object_id:04X}:#{index}")
            }
        },
        0x0020|0x01d2=>field_descriptor_metadata(sym,&ins.operands),
        0x01a4=>"builtin Upper(value)".into(),
        0x0199=>"builtin Len(value)".into(),
        0x005c=>"binary subtraction (-)".into(),
        _=>format!("{}",opcode_action(op)),
    }
}

fn gate_trace_block_start(lines:&[FlowInsn], branch_position:usize)
    -> Option<usize>
{
    let mut candidates=vec![0usize];
    for ins in lines {
        if matches!(ins.opcode,0x0002|0x0003|0x0004) {
            candidates.push(flow_operand_word(&ins.operands,0));
        }
        if matches!(ins.opcode,0x0000..=0x0004) {
            candidates.push(ins.end);
        }
    }
    candidates.into_iter()
        .filter(|&offset|offset<=branch_position)
        .max()
}

fn print_redacted_gate_trace(lines:&[FlowInsn],sym:&FunctionSymbols,
    show_sensitive:bool) {
    println!("=== CONNECT GATE INSTRUCTION DIAGNOSTICS ({}) ===",
        if show_sensitive {"SENSITIVE LOCAL OUTPUT"} else {"REDACTED"});
    if show_sensitive {
        println!("SENSITIVE MODE: compiled literal values follow; keep output local.");
    } else {
        println!("No source literals or raw operands. Type IDs, named symbols, opcodes and offsets only.");
    }
    for (gate, limit) in [(0x009cusize,80usize),(0x02d8usize,50usize)] {
        println!("\nGate at 0x{gate:04X}:");
        let Some(begin)=gate_trace_block_start(lines,gate) else {
            println!("  Cannot determine basic block boundary.");
            continue;
        };
        let block:Vec<&FlowInsn>=lines.iter()
            .filter(|ins|ins.position>=begin && ins.position<=gate).collect();
        let omitted=block.len().saturating_sub(limit);
        println!("  Basic block: 0x{begin:04X}..0x{gate:04X} | instructions {}",
            block.len());
        if omitted>0 {
            println!("  First {omitted} instruction(s) omitted from this view.");
        }
        for ins in block.iter().skip(omitted) {
            println!("  0x{:04X}  0x{:04X} {:<27} {}",
                ins.position,ins.opcode,opcode_action(ins.opcode),
                redacted_gate_instruction(sym,ins,show_sensitive));
        }
        // Print only unnamed locals referenced by this basic block:
        // identifies variable#9's declared type without its runtime value.
        let mut anonymous=std::collections::BTreeSet::new();
        for ins in &block {
            if matches!(ins.opcode,0x001e|0x0030|0x014f|0x0150|
                0x0172|0x01a9|0x0200|0x011d|0x0120) {
                let idx=flow_operand_word(&ins.operands,0);
                if let Some(var)=sym.locals.get(idx) {
                    if var.name.starts_with("<variable#") {
                        anonymous.insert(idx);
                    }
                }
            }
        }
        for index in anonymous {
            if let Some(var)=sym.locals.get(index) {
                println!("  Unnamed local #{index}: {}",
                    describe_type_id(var.type_id,&sym.type_names));
            }
        }
    }
}

fn print_preconnect_routing(lines:&[FlowInsn], sym:&FunctionSymbols,
    show_sensitive:bool) {
    println!("\n=== CONNECT ENTRY ROUTING (STATIC, {}) ===",
        if show_sensitive {"SENSITIVE LOCAL OUTPUT"} else {"REDACTED"});
    println!("Explores both outcomes of unknown conditions; stops at first CONNECT or RETURN.");
    let endpoints=next_connect_endpoints(lines,0);
    println!("First reachable operations from function entry: {}",
        if endpoints.is_empty() {"<none>".to_string()} else {endpoints.join(", ")});
    let gates=preconnect_branch_gates(lines);
    if gates.is_empty() {
        println!("No branch distinguishes first-connection destinations.");
        return;
    }
    println!("Branch decisions that distinguish first CONNECT/RETURN destinations:");
    for (offset,jump,jumps,falls) in gates.iter().take(40) {
        println!("  Branch at 0x{offset:04X}:");
        let predicate=redacted_entry_gate_predicate(lines,sym,*offset);
        let op=lines.iter().find(|ins|ins.position==*offset).map(|ins|ins.opcode);
        let condition=if op==Some(0x0003) {"IF NOT"} else {"IF"};
        println!("    {} {} (all constants redacted)",condition,predicate);
        if show_sensitive {
            if let Some(branch_idx)=lines.iter().position(|ins|ins.position==*offset) {
                if let Some(literal)=branch_idx.checked_sub(3).and_then(|idx|lines.get(idx)) {
                    if literal.opcode==0x003b {
                        if let Some(value)=sensitive_literal_at(sym,literal) {
                            println!("    Comparison constant: {value}");
                        }
                    }
                }
            }
        }
        println!("    jump -> 0x{jump:04X}: {}",
            if jumps.is_empty() {"<none>".to_string()} else {jumps.join(", ")});
        println!("    fallthrough -> {}",
            if falls.is_empty() {"<none>".to_string()} else {falls.join(", ")});
    }
    if gates.len()>40 {
        println!("  {} further distinguishing branches omitted",gates.len()-40);
    }
    if !show_sensitive {
        println!("Note: a branch's comparison is not reconstructed here; no literal values are printed.");
    } else {
        println!("Comparison literals are static constants, not runtime values.");
    }
}

fn print_connection_control_flow(
    lines:&[FlowInsn], sym:&FunctionSymbols,
    catalog:Option<&[SystemCatalogObject]>,
    connections:&[usize],
) {
    println!("\n=== CONNECT SQLCODE OUTCOME GRAPH (STATIC) ===");
    println!("Only type-verified SQLCode =/<> 0 guards are classified.");
    println!("Other branch conditions are explored conservatively, not evaluated.");
    if catalog.is_none() {
        println!("No system catalog provided; SQLCode guards cannot be type-verified.");
    }
    for (n,index) in connections.iter().enumerate() {
        let connect=&lines[*index];
        println!("\nCONNECT #{} at 0x{:04X}:",n+1,connect.position);
        match connect_sqlcode_guard(sym,lines,*index,catalog) {
            Some(guard)=>{
                println!("  SQL outcome check at 0x{:04X}: {} compared with 0",
                    guard.branch_offset,guard.field_name);
                let ok=next_connect_endpoints(lines,guard.zero_dest);
                println!("  SQLCode = 0 (success): -> 0x{:04X}; next endpoints: {}",
                    guard.zero_dest,ok.join(", "));
                let failure=next_connect_endpoints(lines,guard.nonzero_dest);
                println!("  SQLCode <> 0 (failure): -> 0x{:04X}; next endpoints: {}",
                    guard.nonzero_dest,failure.join(", "));
            },
            None=>{
                println!("  Immediate type-verified SQLCode branch not identified.");
                let next=next_connect_endpoints(lines,connect.end);
                println!("  Possible following endpoints: {}",next.join(", "));
            }
        }
    }
}

// Entire-function control-flow reachability, unlike next_connect_endpoints,
// which stops at the first CONNECT. Unknown branch conditions explore both
// directions. No database or application code is ever executed.
fn reachable_return_indices(lines:&[FlowInsn]) -> Vec<usize> {
    use std::collections::{BTreeMap,BTreeSet};
    let positions:BTreeMap<usize,usize>=lines.iter().enumerate()
        .map(|(i,ins)|(ins.position,i)).collect();
    let mut visited=BTreeSet::<usize>::new();
    let mut todo=vec![0usize];
    let mut result=Vec::new();
    while let Some(pos)=todo.pop() {
        if !visited.insert(pos) {continue;}
        let Some(&index)=positions.get(&pos) else {continue;};
        let ins=&lines[index];
        match ins.opcode {
            0x0000|0x0001=>result.push(index),
            0x0004=>todo.push(flow_operand_word(&ins.operands,0)),
            0x0002|0x0003=>{
                todo.push(flow_operand_word(&ins.operands,0));
                todo.push(ins.end);
            },
            _=>todo.push(ins.end),
        }
    }
    result.sort_unstable();
    result
}

// Resolve a narrow, exact sequence: typed transaction local -> indexed
// member -> member read -> optional stack-neutral casts -> RETURN(value).
// Unlike a proximity-based heuristic, this supports only a verified
// five-instruction expression and never accesses the field's runtime value.
fn direct_transaction_field_return(
    lines:&[FlowInsn],idx:usize,sym:&FunctionSymbols,
    catalog:Option<&[SystemCatalogObject]>
) -> Option<String> {
    let catalog=catalog?;
    let ret=lines.get(idx)?;
    if ret.opcode!=0x0001 || flow_operand_word(&ret.operands,0)!=1 {
        return None;
    }
    let mut before=idx.checked_sub(1)?;
    while matches!(lines.get(before)?.opcode,
        0x003f|0x008e|0x0139|0x0123..=0x012b|0x012d..=0x013e)
    {
        before=before.checked_sub(1)?;
    }
    let read=lines.get(before)?;
    if read.opcode!=0x0027{return None;}
    let field=lines.get(before.checked_sub(1)?)?;
    let object=lines.get(before.checked_sub(2)?)?;
    let property=catalog_transaction_property(sym,object,field,catalog)?;
    // Verify instruction order is truly contiguous.
    if object.end!=field.position || field.end!=read.position {return None;}
    let local_idx=flow_operand_word(&object.operands,0);
    let name=sym.locals.get(local_idx)?.name.as_str();
    if name.is_empty() || !name.chars().all(|c|c.is_ascii_alphanumeric()||c=='_'){
        return None;
    }
    Some(format!("RETURN {name}.{property} (system catalog verified)"))
}

// Only the conventional status literals -1/0/1 are displayed. If the return
// expression is not a single, immediately preceding literal/push, don't guess
// it: PB's expression stack can be populated across earlier instructions.
fn safe_immediate_return_hint(
    lines:&[FlowInsn],idx:usize,sym:&FunctionSymbols,
    catalog:Option<&[SystemCatalogObject]>
) -> String {
    if let Some(value)=direct_transaction_field_return(lines,idx,sym,catalog) {
        return value;
    }
    let Some(ins)=lines.get(idx) else {return "unresolved return".into();};
    if ins.opcode==0x0000 ||
        (ins.opcode==0x0001 && flow_operand_word(&ins.operands,0)==0) {
        return "RETURN without value".into();
    }
    if ins.opcode!=0x0001 || flow_operand_word(&ins.operands,0)!=1 {
        return "RETURN flag unresolved".into();
    }
    let mut previous=idx.checked_sub(1);
    while let Some(p)=previous {
        // Only verified stack-neutral cast opcode families.
        if matches!(lines[p].opcode,
            0x003f|0x008e|0x0139|0x0123..=0x012b|0x012d..=0x013e) {
            previous=p.checked_sub(1);
        } else {break;}
    }
    let Some(prev)=previous.and_then(|p|lines.get(p)) else {
        return "RETURN of stack value; origin unresolved".into();
    };
    match prev.opcode {
        0x0034=>{
            match operand_i32(&prev.operands) {
                Some(-1)=>"RETURN; preceding status literal -1".into(),
                Some(0)=>"RETURN; preceding status literal 0".into(),
                Some(1)=>"RETURN; preceding status literal 1".into(),
                _=>"RETURN; preceding numeric literal redacted".into(),
            }
        },
        0x003b=>"RETURN; preceding string literal redacted".into(),
        0x001e|0x0030|0x01a9|0x0150=>{
            let index=flow_operand_word(&prev.operands,0);
            let name=sym.locals.get(index)
                .map(|v|gate_symbol_name(&v.name))
                .unwrap_or("<unresolved>");
            format!("RETURN; immediately preceding local[{index}] {name}")
        },
        _=>format!("RETURN of stack value; preceding opcode {} (source not proven)",
            opcode_action(prev.opcode)),
    }
}

// Path-sensitive, direct-assign evidence. Each state is a bitmask indicating
// that this control-flow path has *encountered* a typed transaction member
// BeginAssign opcode. This is NOT data-flow proof of a completed assignment,
// and does not account for indirect/method-based changes to a transaction.
fn transaction_assignment_path_states(
    lines:&[FlowInsn],sym:&FunctionSymbols,objects:&[SystemCatalogObject]
) -> std::collections::BTreeMap<usize,std::collections::BTreeSet<u8>> {
    use std::collections::{BTreeMap,BTreeSet};
    let positions:BTreeMap<usize,usize>=lines.iter().enumerate()
        .map(|(i,ins)|(ins.position,i)).collect();
    let mut begins=BTreeMap::<usize,u8>::new();
    for triple in lines.windows(3) {
        if triple[2].opcode!=0x0122 {continue;}
        let Some(name)=catalog_transaction_property(
            sym,&triple[0],&triple[1],objects) else {continue;};
        let bit=match name.to_ascii_lowercase().as_str() {
            "dbms"=>1u8,
            "autocommit"=>2u8,
            "dbparm"=>4u8,
            _=>continue,
        };
        begins.insert(triple[2].position,bit);
    }
    let mut visited=BTreeSet::<(usize,u8)>::new();
    let mut todo=vec![(0usize,0u8)];
    let mut outcomes=BTreeMap::<usize,BTreeSet<u8>>::new();
    while let Some((offset,initial_state))=todo.pop() {
        if !visited.insert((offset,initial_state)) {continue;}
        let Some(&index)=positions.get(&offset) else {continue;};
        let ins=&lines[index];
        let state=initial_state | begins.get(&offset).copied().unwrap_or(0u8);
        if ins.opcode==0x0005 {
            outcomes.entry(offset).or_default().insert(state);
        }
        match ins.opcode {
            0x0000|0x0001=>{},
            0x0004=>todo.push((flow_operand_word(&ins.operands,0),state)),
            0x0002|0x0003=>{
                todo.push((flow_operand_word(&ins.operands,0),state));
                todo.push((ins.end,state));
            },
            _=>todo.push((ins.end,state)),
        }
    }
    outcomes
}

fn print_transaction_assignment_paths(
    lines:&[FlowInsn],sym:&FunctionSymbols,
    objects:Option<&[SystemCatalogObject]>,
) {
    println!("\n=== DIRECT TRANSACTION ASSIGNMENTS ON CONNECT PATHS ===");
    println!("Evidence: typed BeginAssign opcodes reachable on each static path.");
    println!("This is not proof of completed writes or final DBParm values.");
    let Some(objects)=objects else {
        println!("Skipped: requires external PB333 system catalog.");
        return;
    };
    let outcomes=transaction_assignment_path_states(lines,sym,objects);
    for ins in lines.iter().filter(|ins|ins.opcode==0x0005) {
        let Some(states)=outcomes.get(&ins.position) else {
            println!("CONNECT 0x{:04X}: not reachable in static CFG",ins.position);
            continue;
        };
        println!("CONNECT 0x{:04X}: {} distinct direct-assignment state(s)",
            ins.position,states.len());
        for (name,bit) in [("DBMS",1u8),("AutoCommit",2),("DBParm",4)] {
            let yes=states.iter().filter(|&&state|state&bit!=0).count();
            let summary=if yes==states.len() {"ALL paths"}
                else if yes==0 {"NO paths"} else {"SOME paths"};
            println!("  {name}: direct BeginAssign encountered on {summary}");
        }
    }
    println!("Caveat: object initialization and indirect runtime changes not analyzed.");
}

fn print_connect_return_map(lines:&[FlowInsn], sym:&FunctionSymbols,
    catalog:Option<&[SystemCatalogObject]>, show_sensitive:bool) {
    println!("\n=== CONNECT RETURN CONTRACT ({}) ===",
        if show_sensitive {"SENSITIVE LOCAL OUTPUT"} else {"REDACTED STATIC TRACE"});
    println!("Reachable RETURN sites, nearby opcodes and conservative payload hints.");
    if show_sensitive {
        println!("SENSITIVE MODE: available compiled literals follow; runtime values still unknown.");
    } else {
        println!("No raw operands, passwords, DBParm text, string literals or arbitrary numbers.");
    }
    let reachable=reachable_return_indices(lines);
    println!("Reachable RETURN instructions: {}",reachable.len());
    for index in reachable {
        let ins=&lines[index];
        println!("\nRETURN at 0x{:04X}: {}",ins.position,
            safe_immediate_return_hint(lines,index,sym,catalog));
        let start=gate_trace_block_start(lines,ins.position)
            .unwrap_or(ins.position);
        let block:Vec<&FlowInsn>=lines.iter()
            .filter(|item|item.position>=start && item.position<=ins.position)
            .collect();
        let recent=block.len().saturating_sub(12);
        println!("  Local basic block 0x{start:04X}..0x{:04X}; last {} instruction(s):",
            ins.position,block.len()-recent);
        for item in block.iter().skip(recent) {
            println!("    0x{:04X} 0x{:04X} {:<27} {}",
                item.position,item.opcode,opcode_action(item.opcode),
                redacted_gate_instruction(sym,item,show_sensitive));
        }
    }
    println!("\nNote: RETURN payload hints inspect preceding opcodes only; they do not");
    println!("prove semantic success, failure, or the origin of a stack value.");
}

// Report only statically validated SQL operation inputs and sanitized
// instruction metadata. This is deliberately not a SQL decompiler: dynamic
// concatenation, substitution and control flow can change the final query.
fn is_sql_word_boundary(s:&str, prefix:&str) -> bool {
    s.starts_with(prefix) && s[prefix.len()..].chars().next()
        .map(|c|!c.is_ascii_alphanumeric() && c!='_')
        .unwrap_or(true)
}

fn sql_literal_intent(literal:&str) -> Option<&'static str> {
    let trimmed=literal.trim_start();
    let ascii=trimmed.to_ascii_uppercase();
    let text=ascii.as_str();
    if is_sql_word_boundary(text,"SET") {
        return Some("session-setting candidate (SET)");
    }
    if is_sql_word_boundary(text,"SELECT") || is_sql_word_boundary(text,"WITH") {
        return Some("query-like prefix (SELECT/WITH)");
    }
    if ["INSERT","UPDATE","DELETE","MERGE"].iter()
        .any(|p|is_sql_word_boundary(text,p)) {
        return Some("potential data-changing DML");
    }
    if ["CREATE","ALTER","DROP","TRUNCATE"].iter()
        .any(|p|is_sql_word_boundary(text,p)) {
        return Some("potential schema-changing DDL");
    }
    if ["EXEC","EXECUTE","CALL","GRANT","REVOKE"].iter()
        .any(|p|is_sql_word_boundary(text,p)) {
        return Some("command/call/permissions prefix");
    }
    None
}

// Restrict the scan to the local contiguous block that contains EXECUTE
// IMMEDIATE. The reified SQL is never printed. Even a recognized prefix is
// only evidence from a literal, not a proven classification of the final SQL.
// A precise match for the final ls_pom rewrite observed in PB333 f_connect.
// This only recognizes the structural expression; it does not evaluate any
// string or the dynamic database statement. System function names are
// resolved only from a version-matched system catalog.
// Verify a compiler-emitted concatenation of constant strings that
// *appends* to the previous value of the same local variable.
//
// BeginAssignLocal -> (literal + ... + literal) -> ResetAssign
// -> Add -> Cast -> EndAssign.
//
// PbdViewer's ResetAssign duplicates the pending l-value beneath the
// right-hand expression. The final Add therefore uses the old variable
// value plus the new literal expression; it does not overwrite from scratch.
// Stack-exact, per-assignment reconstruction. No actual SQL is executed.
// Stops at any unsupported operation rather than making up a source
// expression. Each redacted token carries only its P-code offset, never
// literal bytes, credentials or personal information.
fn sql_assignment_expression(
    instructions:&[&FlowInsn],sym:&FunctionSymbols,
    catalog:Option<&[SystemCatalogObject]>,show_sensitive:bool,
) -> Result<String,String> {
    sql_assignment_expression_with_prior(
        instructions,sym,catalog,show_sensitive,None
    )
}

fn sql_assignment_expression_with_prior(
    instructions:&[&FlowInsn],sym:&FunctionSymbols,
    catalog:Option<&[SystemCatalogObject]>,show_sensitive:bool,
    prior_rhs:Option<&str>,
) -> Result<String,String> {
    let first=instructions.first().ok_or("empty assignment")?;
    if !matches!(first.opcode,0x011d|0x0120) {
        return Err("first opcode is not BeginAssignLocalVariable".into());
    }
    let index=flow_operand_word(&first.operands,0);
    let name=sym.locals.get(index)
        .map(|var|gate_symbol_name(&var.name))
        .ok_or("assignment local missing")?;
    let mut stack=vec![name.to_string()];
    let mut ended=false;
    for (i,ins) in instructions.iter().enumerate().skip(1) {
        let offset=ins.position;
        let pop=|stack:&mut Vec<String>|stack.pop()
            .ok_or_else(||format!("stack underflow at 0x{offset:04X}"));
        match ins.opcode {
            0x001e|0x0030|0x01a9|0x0150=>{
                let pos=flow_operand_word(&ins.operands,0);
                let value=if pos==index {
                    prior_rhs.map(|rhs|format!("({rhs})"))
                } else {None}.unwrap_or_else(||sym.locals.get(pos)
                    .map(|v|gate_symbol_name(&v.name).to_string())
                    .unwrap_or_else(||format!("<local#{pos}>")));
                stack.push(value);
            },
            0x003b=>{
                let value=if show_sensitive {
                    let raw=ins.operands.get(..4)
                        .ok_or_else(||format!("invalid string reference at 0x{offset:04X}"))?;
                    let ptr=u32::from_le_bytes([raw[0],raw[1],raw[2],raw[3]]);
                    let actual=utf16_string(&sym.stack_buffer,ptr)
                        .ok_or_else(||format!("unresolved string at 0x{offset:04X}"))?;
                    format!("\"{}\"",actual.escape_debug())
                } else {format!("<string@0x{offset:04X}>")};
                stack.push(value);
            },
            0x0034=>{
                let value=if show_sensitive {
                    operand_i32(&ins.operands)
                        .map(|v|v.to_string())
                        .ok_or_else(||format!("invalid integer at 0x{offset:04X}"))?
                } else {format!("<number@0x{offset:04X}>")};
                stack.push(value);
            },
            0x0032|0x0033|0x0035..=0x003a|0x003c|0x003d|0x01ff=>{
                stack.push(format!("<constant@0x{offset:04X}>"));
            },
            0x003f|0x008e|0x0123..=0x012b|0x012d..=0x013e=>{
                // Version-mapped casts are stack-neutral in PbdViewer.
            },
            0x007d|0x005c|0x00a8|0x00ad|0x00b8=>{
                let rhs=pop(&mut stack)?;
                let lhs=pop(&mut stack)?;
                let op=match ins.opcode {
                    0x007d=>"+",0x005c=>"-",
                    0x00a8|0x00ad=>"=",0x00b8=>"<>",_=>unreachable!(),
                };
                stack.push(format!("({lhs} {op} {rhs})"));
            },
            0x0199|0x01a4|0x0195=>{
                let arg=pop(&mut stack)?;
                let func=match ins.opcode {
                    0x0199=>"Len",0x01a4=>"Upper",_=>"IsNull",
                };
                stack.push(format!("{func}({arg})"));
            },
            0x01bc=>{
                let index=flow_operand_word(&ins.operands,0);
                let owner=flow_operand_word(&ins.operands,2);
                let function=if owner&0x8000!=0 {
                    sym.referenced_functions.get(index)
                        .map(|v|gate_symbol_name(v).to_string())
                        .unwrap_or_else(||format!("function_ref_{index}"))
                } else if owner==0x40d0 {
                    catalog.and_then(|objects|objects.iter().find(|o|usize::from(o.type_id)==owner))
                        .and_then(|obj|obj.function_definitions.get(index))
                        .map(|name|gate_symbol_name(name).to_string())
                        .unwrap_or_else(||format!("system_function_{index}"))
                } else {format!("function_0x{owner:04X}_{index}")};
                stack.push(function);
            },
            0x01bd=>{
                let function=pop(&mut stack)?;
                let count=flow_operand_word(&ins.operands,2);
                if count>16 || count>stack.len() {
                    return Err(format!("invalid call arity at 0x{offset:04X}"));
                }
                let args=stack.split_off(stack.len()-count);
                stack.push(format!("{function}({})",args.join(", ")));
            },
            0x0119=>{
                // PbdViewer ResetAssign: pop RHS, duplicate l-value under
                // RHS. Thus the subsequent add retains the old local value.
                let rhs=pop(&mut stack)?;
                let lvalue=stack.last().cloned()
                    .ok_or_else(||format!("ResetAssign has no target at 0x{offset:04X}"))?;
                stack.push(prior_rhs.map(|value|format!("({value})"))
                    .unwrap_or(lvalue));
                stack.push(rhs);
            },
            0x0080|0x0082|0x0088=>{
                if i+1!=instructions.len() {
                    return Err(format!("intermediate EndAssign at 0x{offset:04X}"));
                }
                let rhs=pop(&mut stack)?;
                let lvalue=pop(&mut stack)?;
                if lvalue!=name || !stack.is_empty() {
                    return Err(format!("unresolved assignment stack at 0x{offset:04X}"));
                }
                stack.push(format!("{lvalue} = {rhs}"));
                ended=true;
            },
            _=>return Err(format!("unsupported opcode 0x{:04X} at 0x{offset:04X}",
                ins.opcode)),
        }
        if stack.len()>64 || stack.iter().any(|value|value.len()>16384) {
            return Err("symbolic expression size limit reached".into());
        }
    }
    if !ended || stack.len()!=1 {
        return Err("assignment ended without verified EndAssign".into());
    }
    Ok(stack.pop().unwrap_or_default())
}

// Compose only successfully decoded writes to the same SQL source in the
// current basic block. Redaction is forced even in a sensitive local run.
// Refuse composition if the initial write depends on an unknown prior value.
fn composed_sql_source_assignments(
    block:&[&FlowInsn],source_idx:usize,sym:&FunctionSymbols,
    catalog:Option<&[SystemCatalogObject]>,
) -> Result<(usize,String),String> {
    let name=sym.locals.get(source_idx)
        .map(|v|gate_symbol_name(&v.name))
        .ok_or("SQL source local missing")?;
    let mut rhs:Option<String>=None;
    let mut count=0usize;
    let mut last_end:Option<usize>=None;
    for (i,ins) in block.iter().enumerate() {
        if !matches!(ins.opcode,0x011d|0x0120) ||
            flow_operand_word(&ins.operands,0)!=source_idx {continue;}
        if last_end.is_some_and(|end|i<=end) {
            return Err("overlapping SQL source assignments".into());
        }
        let end=((i+1)..block.len()).find(|&j|matches!(block[j].opcode,
            0x007f..=0x008c|0x0104..=0x0118|0x0159|0x0205|0x0207..=0x0209))
            .ok_or_else(||format!("missing EndAssign at 0x{:04X}",ins.position))?;
        let segment=&block[i..=end];
        if rhs.is_none() && segment.iter().skip(1).any(|ins|
            (matches!(ins.opcode,0x001e|0x0030|0x01a9|0x0150) &&
             flow_operand_word(&ins.operands,0)==source_idx) ||
            ins.opcode==0x0119
        ) {
            return Err("initial assignment reads unknown previous value".into());
        }
        let formula=sql_assignment_expression_with_prior(
            segment,sym,catalog,false,rhs.as_deref()
        )?;
        let prefix=format!("{name} = ");
        let next=formula.strip_prefix(&prefix)
            .ok_or("assignment expression has unexpected destination")?;
        if next.len()>16384 {
            return Err("composed expression size limit reached".into());
        }
        rhs=Some(next.to_string());
        count+=1;
        last_end=Some(end);
    }
    rhs.map(|value|(count,format!("{name} = {value}")))
        .ok_or_else(||"no SQL source assignments in block".into())
}

fn recognized_literal_append(
    segment:&[&FlowInsn],sym:&FunctionSymbols,show_sensitive:bool,
) -> Option<(usize,Option<String>)> {
    if segment.len()<9 {return None;}
    let begin=segment[0];
    let end=segment[segment.len()-1];
    if begin.opcode!=0x011d || end.opcode!=0x0088 {return None;}
    let tail=&segment[segment.len()-4..];
    if tail[0].opcode!=0x0119 || tail[1].opcode!=0x007d ||
        !matches!(tail[2].opcode,0x0123..=0x012b|0x012d..=0x013e) ||
        tail[3].opcode!=0x0088 {return None;}
    let rhs=&segment[1..segment.len()-4];
    if rhs.len()<4 || (rhs.len()-4)%3!=0 {return None;}
    if rhs[0].opcode!=0x003b || rhs[1].opcode!=0x003b ||
        rhs[2].opcode!=0x007d ||
        !matches!(rhs[3].opcode,0x0123..=0x012b|0x012d..=0x013e)
    {return None;}
    let mut literal_positions=vec![0usize,1usize];
    for (i,group) in rhs[4..].chunks_exact(3).enumerate() {
        if group[0].opcode!=0x003b ||
            group[1].opcode!=0x007d ||
            !matches!(group[2].opcode,0x0123..=0x012b|0x012d..=0x013e)
        {return None;}
        literal_positions.push(4+i*3);
    }
    let count=literal_positions.len();
    let joined=if show_sensitive {
        let mut value=String::new();
        for pos in literal_positions {
            let raw=rhs[pos].operands.get(..4)?;
            let offset=u32::from_le_bytes([raw[0],raw[1],raw[2],raw[3]]);
            let piece=utf16_string(&sym.stack_buffer,offset)?;
            if value.len().saturating_add(piece.len())>16384 {return None;}
            value.push_str(&piece);
        }
        Some(value)
    } else {None};
    Some((count,joined))
}

fn final_sql_source_rewrite(
    block:&[&FlowInsn], sym:&FunctionSymbols,
    system_catalog:Option<&[SystemCatalogObject]>,
) -> Option<String> {
    for window in block.windows(13) {
        let opcodes=[
            0x011d,0x001e,0x001e,0x0199,0x0034,0x005c,
            0x01bc,0x01bd,0x013b,0x003b,0x007d,0x0129,0x0088,
        ];
        if window.iter().map(|ins|ins.opcode).ne(opcodes.into_iter()) {
            continue;
        }
        let dest=flow_operand_word(&window[0].operands,0);
        if flow_operand_word(&window[1].operands,0)!=dest ||
            flow_operand_word(&window[2].operands,0)!=dest {continue;}
        let owner=flow_operand_word(&window[6].operands,2);
        let function_index=flow_operand_word(&window[6].operands,0);
        if owner!=0x40d0 || function_index!=139 ||
            flow_operand_word(&window[7].operands,2)!=2 {continue;}
        let name=sym.locals.get(dest)?.name.as_str();
        if name.is_empty() || !name.chars().all(|c|
            c.is_ascii_alphanumeric() || c=='_') {continue;}
        let function=system_catalog
            .and_then(|objects|objects.iter().find(|x|x.type_id==0x40d0))
            .and_then(|obj|obj.function_definitions.get(function_index))
            .filter(|value|!value.is_empty() &&
                value.chars().all(|c|c.is_ascii_alphanumeric() || c=='_'))
            .map(|s|s.as_str()).unwrap_or("system_function_139");
        return Some(format!(
            "{name} = {function}({name}, Len({name}) - <integer-hidden>) + <string-hidden>"
        ));
    }
    None
}

fn print_post_connect_sql_trace(
    lines:&[FlowInsn],sym:&FunctionSymbols,show_sensitive:bool,
    system_catalog:Option<&[SystemCatalogObject]>,
) {
    println!("=== POST-CONNECT DYNAMIC SQL TRACE ({}) ===",
        if show_sensitive {"SENSITIVE LOCAL OUTPUT"} else {"REDACTED"});
    if show_sensitive {
        println!("SENSITIVE MODE: compiled literals shown where decoded. No SQL execution.");
    } else {
        println!("Static bytecode only: no SQL values, raw operands, secret literals or query execution.");
    }
    let sql_sites:Vec<usize>=lines.iter().enumerate()
        .filter_map(|(idx,ins)|(ins.opcode==0x01c3).then_some(idx))
        .collect();
    println!("EXECUTE IMMEDIATE sites: {}",sql_sites.len());
    for idx in sql_sites {
        let sql=&lines[idx];
        println!("\nSQL operation at 0x{:04X}:",sql.position);
        let operands=idx.checked_sub(2)
            .and_then(|j|lines.get(j..idx));
        let src=match operands {
            Some(pair) if pair.len()==2 &&
                pair[0].opcode==0x001e &&
                matches!(pair[1].opcode,0x001e|0x0030) =>
            {
                let source_idx=flow_operand_word(&pair[0].operands,0);
                let tx_idx=flow_operand_word(&pair[1].operands,0);
                let source=sym.locals.get(source_idx);
                let tx=sym.locals.get(tx_idx);
                match (source,tx) {
                    (Some(v),Some(t))=>{
                        println!("  SQL source: local[{source_idx}] {} ({})",
                            gate_symbol_name(&v.name),
                            describe_type_id(v.type_id,&sym.type_names));
                        println!("  Transaction: local[{tx_idx}] {} ({})",
                            gate_symbol_name(&t.name),
                            describe_type_id(t.type_id,&sym.type_names));
                        Some(source_idx)
                    },
                    _=>{
                        println!("  SQL source or transaction local unresolved.");
                        None
                    }
                }
            },
            _=>{
                println!("  SQL source/transaction pattern not recognized; not guessing.");
                None
            }
        };
        let start=gate_trace_block_start(lines,sql.position).unwrap_or(0);
        let block:Vec<&FlowInsn>=lines.iter()
            .filter(|ins|ins.position>=start && ins.position<=sql.position)
            .collect();
        println!("  Local basic block 0x{start:04X}..0x{:04X}: {} instruction(s)",
            sql.position,block.len());
        if let Some(rewrite)=final_sql_source_rewrite(&block,sym,system_catalog) {
            println!("  Verified final local-string rewrite (values redacted):");
            println!("    {rewrite}");
            println!("    Only the final transformation is modeled; earlier assignments");
            println!("    and the final SQL text still require separate analysis.");
        } else {
            println!("  No fully verified final-string rewrite pattern identified.");
        }

        // BeginAssignLocalVariable identifies a potential definition site.
        // Only consider completed assignment sequences before the SQL opcode.
        if let Some(source_idx)=src {
            let mut begun=Vec::<usize>::new();
            for (i,ins) in block.iter().enumerate() {
                if matches!(ins.opcode,0x011d|0x0120) &&
                    flow_operand_word(&ins.operands,0)==source_idx {
                    begun.push(i);
                }
            }
            println!("  BeginAssign sites for SQL source in this block: {}",begun.len());
            match composed_sql_source_assignments(&block,source_idx,sym,system_catalog) {
                Ok((count,formula))=>{
                    println!("  Composed SQL source ({count} assignment(s), redacted, current block only):");
                    println!("    {formula}");
                    println!("    Symbolic source expression only; runtime SQL is not verified.");
                },
                Err(reason)=>println!("  Composed SQL source unavailable: {reason}"),
            }
            for begin_idx in begun {
                let begin=block[begin_idx];
                // First EndAssign follows the BeginAssign; this is a
                // candidate match, not a complete nested-stack proof.
                let end_idx=((begin_idx+1)..block.len()).find(|&i|
                    matches!(block[i].opcode,
                       0x007f..=0x008c|0x0104..=0x0118|0x0159|0x0205|0x0207..=0x0209));
                match end_idx {
                    Some(end_idx)=>{
                        println!("    Candidate assignment 0x{:04X}..0x{:04X}",
                            begin.position,block[end_idx].position);
                        let complete_assignment=&block[begin_idx..=end_idx];
                        match sql_assignment_expression(
                            complete_assignment,sym,system_catalog,show_sensitive
                        ) {
                            Ok(expression)=>println!("      Symbolic assignment: {expression}"),
                            Err(reason)=>println!("      Symbolic assignment unresolved: {reason}"),
                        }
                        // A direct source read and ResetAssign are stronger
                        // provenance evidence than a literal keyword alone.
                        // This does not model indirect aliases or called code.
                        let reads_previous=complete_assignment.iter().skip(1).any(|ins|
                            matches!(ins.opcode,0x001e|0x0030|0x01a9|0x0150) &&
                            flow_operand_word(&ins.operands,0)==source_idx);
                        let reset_count=complete_assignment.iter()
                            .filter(|ins|ins.opcode==0x0119).count();
                        println!("      Prior local source read: {}",
                            if reads_previous {"yes"} else {"no direct read found"});
                        println!("      ResetAssign instructions in this assignment: {reset_count}");
                        if let Some((count,joined))=recognized_literal_append(
                            complete_assignment,sym,show_sensitive
                        ) {
                            println!("      VERIFIED append using ResetAssign: previous ls_pom + {count} concatenated string literals");
                            if let Some(value)=joined {
                                println!("      Compiled appended literal string (local sensitive mode):");
                                println!("        \"{}\"",value.escape_debug());
                            }
                        }
                        let assignment=&block[begin_idx+1..end_idx];
                        let mut functions=std::collections::BTreeSet::<String>::new();
                        for ins in assignment {
                            if ins.opcode==0x01bc {
                                let index=flow_operand_word(&ins.operands,0);
                                let owner=flow_operand_word(&ins.operands,2);
                                if owner&0x8000!=0 {
                                    if let Some(name)=sym.referenced_functions.get(index) {
                                        functions.insert(gate_symbol_name(name).to_string());
                                    }
                                } else {
                                    functions.insert("system function (name not resolved)".into());
                                }
                            }
                        }
                        if functions.is_empty() {
                            println!("      Called function references: none identified");
                        } else {
                            println!("      Called function references: {}",
                                functions.into_iter().collect::<Vec<_>>().join(", "));
                        }
                        for ins in assignment {
                            if ins.opcode!=0x003b {continue;}
                            let Some(raw)=ins.operands.get(..4) else {continue;};
                            let offset=u32::from_le_bytes([raw[0],raw[1],raw[2],raw[3]]);
                            let Some(literal)=utf16_string(&sym.stack_buffer,offset) else {continue;};
                            if let Some(class)=sql_literal_intent(&literal) {
                                println!("      SQL-looking literal at 0x{:04X}: {class} (not final SQL)",
                                    ins.position);
                            }
                        }
                    },
                    None=>println!("    BeginAssign at 0x{:04X}: no direct EndAssign found in block",
                        begin.position),
                }
            }
        }
        // Look for a SQL-like beginning in string literals in this same
        // basic block; never print their contents, not even partial values.
        let mut candidates=std::collections::BTreeMap::<&str,usize>::new();
        for ins in &block {
            if ins.opcode!=0x003b {continue;}
            let Some(raw)=ins.operands.get(..4) else {continue;};
            let offset=u32::from_le_bytes([raw[0],raw[1],raw[2],raw[3]]);
            let Some(literal)=utf16_string(&sym.stack_buffer,offset) else {continue;};
            if let Some(class)=sql_literal_intent(&literal) {
                *candidates.entry(class).or_insert(0)+=1;
            }
        }
        if candidates.is_empty() {
            println!("  Recognized SQL-looking literal prefixes: none (query remains unknown)");
        } else {
            println!("  SQL-looking literal prefixes in block (not final query classification):");
            for (class,count) in candidates {
                println!("    {class}: {count}");
            }
        }
        println!("  Redacted instruction sequence:");
        for ins in block.iter().rev().take(65).rev() {
            println!("    0x{:04X} 0x{:04X} {:<27} {}",
                ins.position,ins.opcode,opcode_action(ins.opcode),
                redacted_gate_instruction(sym,ins,show_sensitive));
        }
        if block.len()>65 {
            println!("    {} earlier instruction(s) in this block omitted",
                block.len()-65);
        }
        if show_sensitive {
            println!("  Decodable static literals in this SQL basic block:");
            for ins in &block {
                if let Some(literal)=sensitive_literal_at(sym,ins) {
                    println!("    0x{:04X}  {literal}",ins.position);
                }
            }
        }
    }
    println!("\nWarning: a string-literal SQL prefix does not prove the final dynamic statement.");
    println!("Read-only safety must be enforced by separate DB credentials/privileges.");
}

// Read-only diagnostic for the lexical interval containing each CONNECT.
// A completed typed DBParm assignment is located using a catalog-verified
// transaction member. No string, integer value, operand bytes, hash or
// connection-string fragment is printed. Lexical proximity does not prove
// that the write actually reaches the CONNECT on a feasible control path.
fn dbparm_completed_assignments(
    lines:&[FlowInsn],sym:&FunctionSymbols,
    catalog:&[SystemCatalogObject],
) -> Vec<(usize,usize,usize)> {
    let mut found=Vec::new();
    let mut start=0usize;
    for (connect_idx,connect) in lines.iter().enumerate() {
        if connect.opcode!=0x0005 {continue;}
        for begin in (start.saturating_add(2))..connect_idx {
            if lines[begin].opcode!=0x0122 {continue;}
            let Some(field)=catalog_transaction_property(
                sym,&lines[begin-2],&lines[begin-1],catalog
            ) else {continue;};
            if !field.eq_ignore_ascii_case("dbparm") {continue;}
            let Some(end)=((begin+1)..connect_idx).find(|&i|
                matches!(lines[i].opcode,
                    0x007f..=0x008c|0x0104..=0x0118|0x0159|
                    0x0205|0x0207..=0x0209)
            ) else {continue;};
            found.push((connect.position,begin,end));
        }
        start=connect_idx+1;
    }
    found
}

fn print_dbparm_call_context(
    lines:&[FlowInsn],sym:&FunctionSymbols,
    catalog:&[SystemCatalogObject],
) {
    println!("=== DBPARM CALL CONTEXT (REDACTED, STATIC) ===");
    println!("Only typed assignment sites, operation types, local names and call arities.");
    println!("All literal values and raw operand bytes suppressed; no credential evaluation.");
    println!("Lexical segments only; not proof of assignment execution on any branch.");
    let assignments=dbparm_completed_assignments(lines,sym,catalog);
    println!("Completed DBParm assignment candidates: {}",assignments.len());
    for (connect,begin,end) in assignments {
        let region=&lines[begin..=end];
        println!("\nCONNECT 0x{connect:04X} DBParm assignment 0x{:04X}..0x{:04X}:",
            lines[begin].position,lines[end].position);
        println!("  Instructions: {}; string literals: {}; integer constants: {}",
            region.len(),
            region.iter().filter(|x|x.opcode==0x003b).count(),
            region.iter().filter(|x|x.opcode==0x0034).count());
        let mut source_locals=std::collections::BTreeSet::new();
        for ins in region {
            if !matches!(ins.opcode,0x001e|0x0030|0x01a9|0x0150) {continue;}
            let idx=flow_operand_word(&ins.operands,0);
            if let Some(var)=sym.locals.get(idx) {
                source_locals.insert(format!("local[{idx}] {}",gate_symbol_name(&var.name)));
            }
        }
        println!("  Local reads (not values): {}",
            if source_locals.is_empty() {"none".into()}
            else {source_locals.into_iter().collect::<Vec<_>>().join(", ")});
        let mut call_count=0usize;
        for call in (begin+1)..end {
            if lines[call].opcode!=0x01bc {continue;}
            call_count+=1;
            let function=referenced_function_at(sym,&lines[call])
                .unwrap_or_else(||"system/object function".into());
            let next=lines.get(call+1).filter(|v|v.opcode==0x01bd);
            let arity=next.map(|v|flow_operand_word(&v.operands,2));
            println!("  Function at 0x{:04X}: {} | immediate call arguments: {}",
                lines[call].position,gate_symbol_name(&function),
                arity.map(|n|n.to_string()).unwrap_or_else(||"unverified".into()));
            println!("    Surrounding instructions (redacted):");
            for ins in &lines[call.saturating_sub(9).max(begin+1)..=(call+2).min(end)] {
                println!("      0x{:04X} 0x{:04X} {:<27} {}",
                    ins.position,ins.opcode,opcode_action(ins.opcode),
                    redacted_gate_instruction(sym,ins,false));
            }
        }
        if call_count==0 {println!("  No referenced function pushes identified.");}
    }
}

fn inspect_connection_sites(
    parser: &PblParser, name: &str, body_index: usize,
    system_catalog: Option<&[SystemCatalogObject]>, show_sensitive:bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let entry=parser.entries().iter()
        .find(|e|e.name.eq_ignore_ascii_case(name))
        .ok_or_else(||format!("compiled entry not found: {name}"))?;
    let data=parser.parse_chunk_chain(entry.data_offset,entry.size)?;
    let symbols=find_pcode_symbols(&data,body_index)?;
    let lines=flow_instructions(&symbols.code)?;
    let mut connect_indices=Vec::new();
    for (index,ins) in lines.iter().enumerate() {
        if ins.opcode==0x0005 {connect_indices.push(index);}
    }
    println!("=== DB CONNECT PROVENANCE MAP ({}) ===",
        if show_sensitive {"SENSITIVE LOCAL OUTPUT"} else {"REDACTED"});
    println!("Entry: {} | body: {} | CONNECT sites: {}",
        entry.name,body_index,connect_indices.len());
    println!("Only property names, function names and instruction offsets.");
    if show_sensitive {
        println!("SENSITIVE MODE: static literals may contain DSNs, passwords or SQL.");
        println!("Computed DBParm values are NOT reconstructed.\n");
    } else {
        println!("No source string values, passwords, or connection strings are displayed.\n");
    }
    if system_catalog.is_some() {
        println!("External PB333 system catalog: loaded and used for transaction fields.");
    }
    for (i,var) in symbols.locals.iter().enumerate() {
        if var.name.eq_ignore_ascii_case("at_sql") ||
            var.name.to_ascii_lowercase().contains("sql")
        {
            println!("Local variable [{}] {}: {}",
                i,var.name,describe_type_id(var.type_id,&symbols.type_names));
        }
    }


    for (site_no,&connect_index) in connect_indices.iter().enumerate() {
        let connect=&lines[connect_index];
        let transaction=match connect_index.checked_sub(1).and_then(|i|lines.get(i)) {
            Some(prev) if matches!(prev.opcode,0x001e|0x0030|0x01a9|0x0150) =>
                flow_local(&symbols,&prev.operands),
            Some(prev) if prev.opcode==0x002f =>
                flow_global(&symbols,&prev.operands),
            _=>"<transaction expression unresolved>".into()
        };
        let start=if site_no==0 {0} else {connect_indices[site_no-1]+1};
        let segment=&lines[start..connect_index];
        println!("CONNECT #{} at 0x{:04X} USING {}",site_no+1,
            connect.position,transaction);
        println!("  Previous CONNECT: {}", if site_no==0 {"none"} else {"yes"});
        let mut property_assigns=Vec::<(usize,String,String,Vec<String>)>::new();
        let mut calls=Vec::<(usize,String)>::new();
        let mut literal_count=0usize;
        for (i,ins) in segment.iter().enumerate() {
            match ins.opcode {
                0x0122 => {
                    // A BEGIN ASSIGN instance-variable operation consumes a
                    // field name and an object reference. Only report the name,
                    // never the value subsequently assigned.
                    if let Some(prev)=i.checked_sub(1).and_then(|j|segment.get(j)) {
                        if prev.opcode==0x0020 {
                            let field=match (system_catalog,
                                i.checked_sub(2).and_then(|j|segment.get(j))) {
                                (Some(objects),Some(object)) =>
                                    catalog_transaction_property(&symbols,object,prev,objects)
                                        .unwrap_or_else(||flow_field_name(&symbols,&prev.operands)),
                                _=>flow_field_name(&symbols,&prev.operands)
                            };
                            let metadata=field_descriptor_metadata(&symbols,&prev.operands);
                            let mut associated_calls=Vec::new();
                            // Only calls contained in this assignment expression;
                            // do not associate all previous calls with the field.
                            for part in segment.iter().skip(i+1) {
                                if matches!(part.opcode,0x007f..=0x008c|
                                    0x0104..=0x0118|0x0159|0x0205|0x0207..=0x0209)
                                {
                                    break;
                                }
                                if let Some(name)=referenced_function_at(&symbols,part) {
                                    associated_calls.push(name);
                                }
                            }
                            property_assigns.push((ins.position,field,metadata,associated_calls));
                        }
                    }
                }
                0x01bc => {
                    let index=flow_operand_word(&ins.operands,0);
                    let object_id=flow_operand_word(&ins.operands,2);
                    if object_id & 0x8000 != 0 {
                        let name=symbols.referenced_functions.get(index)
                            .cloned().unwrap_or_else(||format!("function-ref#{index}"));
                        calls.push((ins.position,name));
                    }
                }
                0x003b => literal_count+=1,
                _=>{}
            }
        }
        println!("  Transaction/object field assignments in this segment:");
        if property_assigns.is_empty() {
            println!("    none identified using direct BeginAssignInstanceVariable");
        }
        for (offset,property,metadata,associated_calls) in &property_assigns {
            let category=match property.to_ascii_lowercase().as_str() {
                "dbpass"|"logpass" => " [PASSWORD FIELD: value omitted]",
                "userid"|"logid" => " [IDENTITY FIELD: value omitted]",
                "dbms"|"dbparm"|"database"|"servername" => " [CONNECTION FIELD: value omitted]",
                _=>""
            };
            println!("    0x{offset:04X}  {property}{category}");
            println!("      descriptor: {metadata}");
            if !associated_calls.is_empty() {
                println!("      calls inside assignment: {}",associated_calls.join(", "));
            }
        }
        if system_catalog.is_some() {
            let keys=known_dbparm_keys(&symbols,segment);
            if !keys.is_empty() {
                println!("  Recognized configuration keys in this segment (names only): {}",
                    keys.join(", "));
                println!("  (heuristic: presence in literals, not proof of active DBParm assignment)");
            }
        }
        if !calls.is_empty() {
            println!("  Referenced functions before this CONNECT:");
            for (offset,name) in calls {
                println!("    0x{offset:04X}  {name}()");
            }
        }
        println!("  String literals encountered: {}",literal_count);
        if show_sensitive {
            println!("  Decodable compiled literals in this segment:");
            for ins in segment {
                if let Some(value)=sensitive_literal_at(&symbols,ins) {
                    println!("    0x{:04X}  {value}",ins.position);
                }
            }
        }
    }
    println!("\nInterpretation: reported functions are nearby dependencies,");
    println!("not necessarily the direct origin of a specific credential value.");
    println!("For exact data-flow, follow the property-assignment stack backwards.");
    print_connection_control_flow(&lines,&symbols,system_catalog,&connect_indices);
    print_preconnect_routing(&lines,&symbols,show_sensitive);
    print_connect_return_map(&lines,&symbols,system_catalog,show_sensitive);
    print_transaction_assignment_paths(&lines,&symbols,system_catalog);
    Ok(())
}


#[derive(Clone)]
struct SystemCatalogObject {
    name: String,
    type_id: u16,
    parent_type: u16,
    all_member_count: usize,
    instance_fields: Vec<String>,
    // FunctionDefinitions array, indexed by the P-code system function index.
    // Only public names; no body, constants or runtime data.
    function_definitions: Vec<String>,
}

fn system_object_fields(r: &mut Cursor<'_>) -> Result<Vec<String>,String> {
    r.bytes(6)?;
    let pool=pool_buffer(r)?;
    let len=r.u16()? as usize;
    if len%20!=0 {return Err("invalid system object variable records".into());}
    let records=r.bytes(len)?;
    let mut fields=Vec::new();
    for rec in records.chunks_exact(20) {
        if rec[0]&0x0f>1 {continue;}
        let offset=u32::from_le_bytes([rec[8],rec[9],rec[10],rec[11]]);
        let name=utf16_string(&pool,offset)
            .ok_or_else(||"invalid system object property name".to_string())?;
        fields.push(name);
    }
    Ok(fields)
}

fn parse_system_objects(data: &[u8]) -> Result<Vec<SystemCatalogObject>, String> {
    let mut r=Cursor::new(data);
    if r.u16()?!=333 {return Err("not a PB333 system group".into());}
    r.bytes(22)?;
    let descriptors=r.u16()? as usize;
    r.bytes(descriptors.checked_mul(12).ok_or("too many descriptors")?)?;
    r.struct_buffer()?;
    r.records20()?;
    let object_count=r.u16()? as usize;
    let extended_count=r.u16()? as usize;
    // The first buffer contains FunctionDefinition names (+0 of each
    // 48-byte definition); the next stores parameter names.
    let function_name_pool=pool_buffer(&mut r)?;
    r.struct_buffer()?;
    let type_names=named_types(&mut r)?;
    r.records20()?;
    let indices=r.bytes(object_count.checked_mul(16)
        .ok_or("too many object indices")?)?.to_vec();
    let extended=r.bytes(extended_count.checked_mul(32)
        .ok_or("too many extended headers")?)?.to_vec();
    let mut used_extended=0usize;
    let mut objects=Vec::new();
    for item in indices.chunks_exact(16) {
        match (item[0]>>1)&7 {
            0=>{
                let start=used_extended.checked_mul(32).ok_or("bad extended offset")?;
                let header=extended.get(start..start+32)
                    .ok_or("missing system object header")?;
                used_extended+=1;
                let type_id=word(item,2)? as u16;
                let parent_type=word(header,0)? as u16;
                let count=r.u16()? as usize;
                r.bytes(count.checked_mul(4).ok_or("too many functions")?)?;
                for _ in 0..count {
                    let len=r.u16()? as usize;
                    let dbg=r.u16()? as usize;
                    r.u16()?;
                    r.bytes(len)?;
                    r.bytes(dbg.checked_mul(4).ok_or("invalid debug length")?)?;
                    r.records20()?;
                    r.struct_buffer()?;
                }
                r.bytes(word(header,24)?.checked_mul(6).ok_or("invalid events")?)?;
                r.bytes(word(header,22)?.checked_mul(4).ok_or("invalid references")?)?;
                r.records20()?;
                let instance_fields=system_object_fields(&mut r)?;
                let all_member_count=word(header,28)?;
                r.bytes(all_member_count.checked_mul(8).ok_or("invalid all-variable entries")?)?;
                r.bytes(word(header,26)?.checked_mul(16).ok_or("invalid function references")?)?;
                let definition_bytes=r.bytes(word(header,4)?
                    .checked_mul(48).ok_or("invalid definitions")?)?;
                let function_definitions=definition_bytes.chunks_exact(48).enumerate()
                    .map(|(index,def)| {
                        let name_offset=u32::from_le_bytes([def[0],def[1],def[2],def[3]]);
                        utf16_string(&function_name_pool,name_offset)
                            .filter(|name|!name.is_empty() && !name.chars().any(char::is_control))
                            .map(|name|name.trim_start_matches('+').to_string())
                            .unwrap_or_else(||format!("<unresolved function #{index}>"))
                    }).collect();
                let name=type_names.get((type_id&0x3fff) as usize)
                    .cloned().unwrap_or_else(||format!("type 0x{type_id:04X}"));
                objects.push(SystemCatalogObject {
                    name,type_id,parent_type,all_member_count,instance_fields,
                    function_definitions,
                });
            }
            1=>{r.bytes(word(item,4)?.checked_mul(8)
                .ok_or("invalid system nonobject entry")?)?;}
            _=>{}
        }
    }
    if r.pos!=data.len() {
        return Err(format!("system group end mismatch 0x{:X} vs 0x{:X}",r.pos,data.len()));
    }
    Ok(objects)
}

fn system_property_name(
    objects: &[SystemCatalogObject], type_id:u16, member:usize,
    depth:usize,
) -> Option<String> {
    if depth>32 {return None;}
    let obj=objects.iter().find(|x|x.type_id==type_id)?;
    if member>=obj.all_member_count ||
        obj.instance_fields.len()>obj.all_member_count {return None;}
    let start=obj.all_member_count-obj.instance_fields.len();
    if member>=start {
        return obj.instance_fields.get(member-start).cloned();
    }
    if obj.parent_type!=type_id {
        system_property_name(objects,obj.parent_type,member,depth+1)
    } else {None}
}


fn report_system_catalog(objects:&[SystemCatalogObject], source:&str) {
    println!("=== PB333 SYSTEM CATALOG METADATA ===");
    println!("Source: {source}");
    println!("No DB values, embedded literals, executable function bodies or credentials.\n");
    println!("System class objects parsed: {}",objects.len());
    let transaction=objects.iter().find(|x|x.name.eq_ignore_ascii_case("transaction"));
    match transaction {
        Some(trans)=>{
            println!("\nTransaction class: type=0x{:04X}, parent=0x{:04X}, all fields={}",
                trans.type_id,trans.parent_type,trans.all_member_count);
            for idx in [2usize,9,14,15] {
                let field=system_property_name(objects,trans.type_id,idx,0)
                    .unwrap_or_else(||"<unresolved>".into());
                println!("  PB P-code member #{idx}: {field}");
            }
        },
        None=>println!("\nTransaction class not identified in this catalog."),
    };
    let system=objects.iter().find(|obj|obj.type_id==0x40d0)
        .or_else(||objects.iter().find(|obj|
            obj.name.eq_ignore_ascii_case("SystemFunctions")));
    match system {
        Some(obj)=>{
            println!("\nSystem function owner: {} (0x{:04X}, {} definitions)",
                obj.name,obj.type_id,obj.function_definitions.len());
            for idx in [139usize,339usize] {
                match obj.function_definitions.get(idx) {
                    Some(function)=>println!("  P-code system function 0x40D0:{idx} = {function}"),
                    None=>println!("  P-code system function index {idx} not in this definition table"),
                }
            }
        },
        None=>println!("\nSystem function class 0x40D0 not identified."),
    }
    println!("\nNote: both names come from the external PB333 system type group.");
}

fn inspect_external_system_catalog(file:&str)
    -> Result<(),Box<dyn std::error::Error>>
{
    const MAX_CATALOG_BYTES:usize=32*1024*1024;
    let metadata=std::fs::metadata(file)?;
    if metadata.len()>MAX_CATALOG_BYTES as u64 {
        return Err("system catalog exceeds 32 MiB safety limit".into());
    }
    let data=std::fs::read(file)?;
    if data.starts_with(&[0x1f,0x8b]) {
        return Err("the supplied file is GZIP; first run gzip -dc 014d.bin > /tmp/pb333_typedef.grp".into());
    }
    let objects=parse_system_objects(&data)?;
    report_system_catalog(&objects,file);
    Ok(())
}

fn transaction_catalog_scan(parser:&PblParser) -> Result<(),Box<dyn std::error::Error>> {
    println!("=== TRANSACTION TYPE MEMBER CATALOG ===");
    println!("Only system property names and indices are reported; no values.");
    let groups:Vec<_>=parser.entries().iter().filter(|entry|
        entry.name.to_ascii_lowercase().ends_with(".grp")).collect();
    println!("Bundled .grp entries: {}",groups.len());
    if groups.is_empty() {
        println!("No system catalog bundled in this EXE.");
        println!("PB333 instance property indexes #2/#14/#15 cannot yet be named reliably.");
        return Ok(());
    }
    let mut objects=Vec::new();
    for group in groups {
        let data=match parser.parse_chunk_chain(group.data_offset,group.size) {
            Ok(data)=>data,
            Err(err)=>{println!("Could not read group {}: {err}",group.name);continue;}
        };
        match parse_system_objects(&data) {
            Ok(found)=>{
                println!("Parsed system group {} ({} objects)",group.name,found.len());
                objects.extend(found);
            }
            Err(err)=>println!("Could not parse group {}: {err}",group.name)
        }
    }
    let transaction=objects.iter().find(|obj|obj.name.eq_ignore_ascii_case("transaction"));
    if let Some(trans)=transaction {
        println!("Transaction: type=0x{:04X}, inherited type=0x{:04X}, all members={}",
            trans.type_id,trans.parent_type,trans.all_member_count);
        for index in [2usize,9,14,15] {
            let result=system_property_name(&objects,trans.type_id,index,0)
                .unwrap_or_else(||"<unresolved, inherited catalog may be absent>".into());
            println!("  member #{index}: {result}");
        }
    } else {
        println!("Transaction class not located in available system catalog objects.");
    }
    Ok(())
}


// Infer transaction property member names from *explicitly named* field
// descriptors in other compiled code. An index-only descriptor is NOT
// evidence of any name. Only type-checked transaction object assignments
// are considered; all literal values and connection strings are ignored.
fn transaction_field_evidence(parser:&PblParser)
    -> Result<(),Box<dyn std::error::Error>>
{
    use std::collections::{BTreeMap,BTreeSet};
    println!("=== TRANSACTION PROPERTY NAME EVIDENCE (REDACTED) ===");
    println!("Searches only descriptor names, not assigned values or strings.\n");
    let mut examined_entries=0usize;
    let mut examined_bodies=0usize;
    let mut typed_assignments=0usize;
    let mut index_only=BTreeMap::<u16,usize>::new();
    let mut named=BTreeMap::<u16,BTreeSet<String>>::new();
    let mut parse_errors=0usize;
    for entry in parser.entries().iter().filter(|e|{
        let n=e.name.to_ascii_lowercase();
        [".fun",".win",".udo",".apl"].iter().any(|ext|n.ends_with(*ext))
    }) {
        let data=match parser.parse_chunk_chain(entry.data_offset,entry.size) {
            Ok(bytes)=>bytes,
            Err(_)=>{parse_errors+=1;continue;}
        };
        let inspection=match inspect(&data) {
            Ok(inspection) if inspection.problems.is_empty() &&
                inspection.decoded_bodies==inspection.bodies =>inspection,
            _=>{parse_errors+=1;continue;}
        };
        if !inspection.opcode_counts.contains_key(&0x0020) ||
            !inspection.opcode_counts.contains_key(&0x0122) {continue;}
        examined_entries+=1;
        for body in 0..inspection.bodies {
            let symbols=match find_pcode_symbols(&data,body) {
                Ok(value)=>value,
                Err(_)=>{parse_errors+=1;continue;}
            };
            examined_bodies+=1;
            let lines=flow_instructions(&symbols.code)?;
            for triple in lines.windows(3) {
                let object=&triple[0];
                let field=&triple[1];
                let assign=&triple[2];
                if field.opcode!=0x0020 || assign.opcode!=0x0122 {continue;}
                let source=match object.opcode {
                    0x001e|0x0030|0x01a9|0x0150 => {
                        let idx=flow_operand_word(&object.operands,0);
                        symbols.locals.get(idx)
                    }
                    0x002f => {
                        let idx=flow_operand_word(&object.operands,0) as u16;
                        symbols.locals.iter().chain(symbols.globals.iter())
                            .find(|v|v.global_index==Some(idx))
                    }
                    _=>None,
                };
                let Some(var)=source else {continue;};
                // Locally described transaction type, not unrelated objects
                // that reuse the same member number.
                let local_type=var.type_id;
                let type_name=if local_type & 0xF000==0x8000 {
                    symbols.type_names.get((local_type&0x3fff) as usize)
                } else {None};
                if !type_name.map(|n|n.eq_ignore_ascii_case("transaction"))
                    .unwrap_or(false) {continue;}
                let Some(offset)=operand_u16(&field.operands).map(usize::from)
                    else {continue;};
                let Some(raw)=symbols.stack_buffer.get(offset..offset.saturating_add(6))
                    else {continue;};
                typed_assignments+=1;
                let word=u32::from_le_bytes([raw[0],raw[1],raw[2],raw[3]]);
                let index=u16::from_le_bytes([raw[4],raw[5]]);
                if word&0xffff==65535 {
                    *index_only.entry(index).or_insert(0)+=1;
                    continue;
                }
                // This is the safe evidence: the compiler stored both a
                // descriptor index and an inline system property name.
                if let Some(name)=utf16_string(&symbols.stack_buffer,word) {
                    if !name.is_empty() && name.len()<100 &&
                        name.chars().all(|c|
                            c.is_ascii_alphanumeric() || c=='_')
                    {
                        named.entry(index).or_default().insert(name);
                    }
                }
            }
        }
    }
    println!("Entries examined with assignment opcodes: {}",examined_entries);
    println!("Bodies decoded: {}",examined_bodies);
    println!("Typed transaction assignments examined: {}",typed_assignments);
    println!("Parsing errors: {}",parse_errors);
    for index in [2u16,14,15] {
        let known=named.get(&index);
        let samples=index_only.get(&index).copied().unwrap_or(0);
        match known {
            Some(names) if names.len()==1 =>println!(
                "member #{index}: {} [explicit inline-name evidence]; index-only occurrences: {samples}",
                names.iter().next().unwrap()
            ),
            Some(names) =>println!(
                "member #{index}: CONFLICTING explicit names {}; index-only occurrences: {samples}",
                names.iter().cloned().collect::<Vec<_>>().join(", ")
            ),
            None=>println!(
                "member #{index}: no inline-name evidence; index-only occurrences: {samples}"
            ),
        }
    }
    println!("No index-to-property guessing is performed.");
    Ok(())
}

fn disassemble_one(
    parser: &PblParser,
    name: &str,
    body_index: usize,
    limit: usize,
    show_strings: bool,
    show_flow: bool,
    show_structured: bool,
    show_sql_debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let entry = parser.entries().iter()
        .find(|e| e.name.eq_ignore_ascii_case(name)
            && e.name.to_ascii_lowercase().ends_with(".fun"))
        .ok_or_else(|| format!("compiled .fun entry not found: {name}"))?;
    let data = parser.parse_chunk_chain(entry.data_offset, entry.size)?;
    let valid = inspect(&data)?;
    if !valid.problems.is_empty() || valid.decoded_bodies != valid.bodies {
        return Err(format!("entry {} fails opcode validation", entry.name).into());
    }
    let symbols = find_pcode_symbols(&data, body_index)?;
    let code = &symbols.code;
    let (instruction_count, _) = decode_pcode(code)?;

    println!("=== PB333 P-CODE DISASSEMBLY ===");
    println!("Entry: {}", entry.name);
    println!("Body index (zero-based): {}", body_index);
    println!("P-code size: {} bytes", code.len());
    println!("Instructions: {}", instruction_count);
    println!("Showing first {} instructions", limit.min(instruction_count));
    println!();
    println!("{:<8} {:<8} {:<28} {:<22} {}", "OFFSET", "OPCODE", "ACTION", "OPERANDS (u16 LE)", "INTERPRETATION");

    let mut pos = 0usize;
    let mut printed = 0usize;
    while pos < code.len() && printed < limit {
        let opcode = u16::from_le_bytes([code[pos], code[pos + 1]]);
        let words = OPERAND_WORDS[opcode as usize] as usize;
        let operand_bytes = &code[pos + 2..pos + 2 + words * 2];
        let operands = operand_bytes.chunks_exact(2)
            .map(|pair| {
                format!("{:04X}", u16::from_le_bytes([pair[0], pair[1]]))
            })
            .collect::<Vec<_>>()
            .join(" ");
        println!(
            "0x{:04X}   0x{:04X}   {:<28} {:<22} {}",
            pos, opcode, opcode_action(opcode), operands,
            annotated_opcode(opcode, operand_bytes, &symbols, show_strings)
        );
        pos += 2 + words * 2;
        printed += 1;
    }

    if printed < instruction_count {
        println!("... {} additional instructions omitted", instruction_count - printed);
    }
    if let Some(value) = simple_constant_return(&code) {
        println!("\n=== CONSERVATIVE POWERSCRIPT PREVIEW ===");
        println!("return {}", value);
        println!("Matched the entire body, including any unreachable epilogue.");
    } else {
        println!("\nNo source-level preview: this body requires stack/control-flow analysis.");
    }
    println!("Note: most operands remain raw numbers, not resolved variable names or literals.");
    if show_flow { print_flow(&symbols,show_strings)?; }
    if show_structured { print_structured(&symbols,show_strings)?; }
    if show_sql_debug { print_sql_cursor_diagnostics(&symbols)?; }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Explicit opt-in for decoded compiled literals, accepted in any mode.
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let show_sensitive = args.iter().any(|arg|arg=="--show-sensitive");
    args.retain(|arg|arg!="--show-sensitive");
    if show_sensitive {
        eprintln!("WARNING: --show-sensitive prints compiled literals locally. Do not share credentials or unredacted logs.");
    }
    if args.first().map(|arg|arg=="--system-metadata").unwrap_or(false) {
        if args.len()!=2 {
            return Err("usage: probe_pcode --system-metadata /tmp/pb333_typedef.grp".into());
        }
        return inspect_external_system_catalog(&args[1]);
    }
    if args.first().map(|arg|arg=="--transaction-field-evidence").unwrap_or(false) {
        if args.len()!=1 {
            return Err("usage: probe_pcode --transaction-field-evidence".into());
        }
        let parser=PblParser::new(EXE)?;
        return transaction_field_evidence(&parser);
    }
    if args.first().map(|arg|arg=="--transaction-catalog").unwrap_or(false) {
        if args.len()!=1 {return Err("usage: probe_pcode --transaction-catalog".into());}
        let parser=PblParser::new(EXE)?;
        return transaction_catalog_scan(&parser);
    }
    if args.first().map(|arg|arg=="--connect-gate-trace").unwrap_or(false) {
        if args.len()!=3 {
            return Err("usage: probe_pcode --connect-gate-trace <object.fun> <body_index>".into());
        }
        let body_index=args[2].parse::<usize>()?;
        let parser=PblParser::new(EXE)?;
        let entry=parser.entries().iter()
            .find(|e|e.name.eq_ignore_ascii_case(&args[1]))
            .ok_or_else(||format!("compiled entry not found: {}",args[1]))?;
        let data=parser.parse_chunk_chain(entry.data_offset,entry.size)?;
        let symbols=find_pcode_symbols(&data,body_index)?;
        let lines=flow_instructions(&symbols.code)?;
        print_redacted_gate_trace(&lines,&symbols,show_sensitive);
        return Ok(());
    }
    if args.first().map(|arg|arg=="--post-connect-sql").unwrap_or(false) {
        if args.len()!=3 && args.len()!=4 {
            return Err("usage: probe_pcode --post-connect-sql <object.fun> <body_index> [catalog.grp]".into());
        }
        let body_index=args[2].parse::<usize>()?;
        let parser=PblParser::new(EXE)?;
        let entry=parser.entries().iter()
            .find(|e|e.name.eq_ignore_ascii_case(&args[1]))
            .ok_or_else(||format!("compiled entry not found: {}",args[1]))?;
        let data=parser.parse_chunk_chain(entry.data_offset,entry.size)?;
        let symbols=find_pcode_symbols(&data,body_index)?;
        let lines=flow_instructions(&symbols.code)?;
        let catalog=if let Some(path)=args.get(3) {
            Some(parse_system_objects(&std::fs::read(path)?)?)
        } else {None};
        print_post_connect_sql_trace(&lines,&symbols,show_sensitive,catalog.as_deref());
        return Ok(());
    }
    if args.first().map(|arg|arg=="--dbparm-call-context").unwrap_or(false) {
        if args.len()!=4 {
            return Err("usage: probe_pcode --dbparm-call-context <object.fun> <body_index> <catalog.grp>".into());
        }
        let body_index=args[2].parse::<usize>()?;
        let parser=PblParser::new(EXE)?;
        let entry=parser.entries().iter()
            .find(|e|e.name.eq_ignore_ascii_case(&args[1]))
            .ok_or_else(||format!("compiled entry not found: {}",args[1]))?;
        let data=parser.parse_chunk_chain(entry.data_offset,entry.size)?;
        let symbols=find_pcode_symbols(&data,body_index)?;
        let lines=flow_instructions(&symbols.code)?;
        let catalog=parse_system_objects(&std::fs::read(&args[3])?)?;
        print_dbparm_call_context(&lines,&symbols,&catalog);
        return Ok(());
    }
    if args.first().map(|arg|arg=="--connect-map").unwrap_or(false) {
        if args.len()<2 || args.len()>4 {
            return Err("usage: probe_pcode --connect-map <object.fun> [body_index] [catalog.grp]".into());
        }
        let index=args.get(2).map(|s|s.parse::<usize>()).transpose()?.unwrap_or(0);
        let catalog=if let Some(path)=args.get(3) {
            let bytes=std::fs::read(path)?;
            Some(parse_system_objects(&bytes)?)
        } else {None};
        let parser=PblParser::new(EXE)?;
        return inspect_connection_sites(&parser,&args[1],index,catalog.as_deref(),show_sensitive);
    }
    if args.first().map(|arg|arg=="--db-scan").unwrap_or(false) {
        if args.len()!=1 {
            return Err("usage: probe_pcode --db-scan".into());
        }
        let parser=PblParser::new(EXE)?;
        return scan_db_connections(&parser);
    }
    // Optional mode: <name.fun> [body_index=0] [max_instructions=80]
    if !args.is_empty() {
        if args.len() > 7 || args.iter().skip(3).any(|x|
            x != "--flow" && x != "--strings" && x != "--structured" && x != "--sql-debug"
        ) {
            return Err("usage: probe_pcode [name.fun [body_index [max_instructions [--flow] [--structured] [--sql-debug] [--strings]]]]".into());
        }
        let body_index = args.get(1).map(|s| s.parse::<usize>())
            .transpose()?.unwrap_or(0);
        let limit = args.get(2).map(|s| s.parse::<usize>())
            .transpose()?.unwrap_or(80).min(1000);
        let show_strings = show_sensitive || args.iter().skip(3).any(|s| s == "--strings");
        let show_flow = args.iter().skip(3).any(|s| s == "--flow");
        let show_structured = args.iter().skip(3).any(|s| s == "--structured");
        let show_sql_debug = args.iter().skip(3).any(|s| s == "--sql-debug");
        let parser = PblParser::new(EXE)?;
        return disassemble_one(&parser, &args[0], body_index, limit,
                               show_strings, show_flow, show_structured, show_sql_debug);
    }
    let parser = PblParser::new(EXE)?;
    let mut total_objects = 0usize;
    let mut parsed_objects = 0usize;
    let mut bodies = 0usize;
    let mut decoded_bodies = 0usize;
    let mut pcode_bytes = 0usize;
    let mut instructions = 0usize;
    let mut frequencies = BTreeMap::<u16, usize>::new();
    let mut errors = Vec::new();

    for entry in parser.entries().iter()
        .filter(|e| e.name.to_ascii_lowercase().ends_with(".fun"))
    {
        total_objects += 1;
        let data = parser.parse_chunk_chain(entry.data_offset, entry.size)?;

        match inspect(&data) {
            Ok(result) => {
                parsed_objects += 1;
                bodies += result.bodies;
                decoded_bodies += result.decoded_bodies;
                pcode_bytes += result.bytes;
                instructions += result.decoded_instructions;
                for (opcode, count) in result.opcode_counts {
                    *frequencies.entry(opcode).or_insert(0) += count;
                }
                for error in result.problems {
                    if errors.len() < 12 {
                        errors.push(format!("{}: {}", entry.name, error));
                    }
                }
            }
            Err(error) => {
                if errors.len() < 12 {
                    errors.push(format!("{}: STRUCTURAL: {}", entry.name, error));
                }
            }
        }
    }

    println!("=== P-CODE OPCODE BOUNDARY VALIDATION (PB333) ===");
    println!("Objects found: {}", total_objects);
    println!("Structurally parsed: {}", parsed_objects);
    println!("P-code bodies found: {}", bodies);
    println!("Fully decoded bodies: {}", decoded_bodies);
    println!("Failed P-code bodies: {}", bodies - decoded_bodies);
    println!("Total P-code bytes: {}", pcode_bytes);
    println!("Decoded instructions: {}", instructions);
    println!("Distinct opcode values: {}", frequencies.len());

    let mapped_opcodes = frequencies.keys()
        .filter(|&&op| opcode_action(op) != "Unmapped").count();
    let mapped_instructions: usize = frequencies.iter()
        .filter(|(op, _)| opcode_action(**op) != "Unmapped")
        .map(|(_, count)| *count)
        .sum();
    println!("Used opcode IDs with mapped actions: {} / {}", mapped_opcodes, frequencies.len());
    println!("Instructions with mapped actions: {} / {}", mapped_instructions, instructions);


    let mut unknown: Vec<(u16, usize)> = frequencies.iter()
        .filter(|(opcode, _)| opcode_action(**opcode) == "Unmapped")
        .map(|(opcode, count)| (*opcode, *count))
        .collect();
    unknown.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    println!("\nUNMAPPED OPCODES (ID, COUNT):");
    for (opcode, count) in &unknown {
        println!("  0x{:04X}  {}", opcode, count);
    }

    let mut ranked: Vec<_> = frequencies.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    println!("\nTOP 25 OPCODES (ID, COUNT, ACTION):");
    for (opcode, count) in ranked.into_iter().take(25) {
        println!("  0x{:04X}  {:>7}  {}", opcode, count, opcode_action(opcode));
    }

    println!("\nFIRST 12 ERRORS:");
    for error in errors {
        println!("  {}", error);
    }

    Ok(())
}


#[cfg(test)]
mod semantic_probe_tests {
    use super::*;
    #[test]
    fn pb333_string_replace_operators_preserve_boolean_and_arithmetic_stack() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:Vec::new(),globals:Vec::new(),
            referenced_functions:Vec::new(),sql_pool:Vec::new(),
            type_names:Vec::new(),
        };
        for (opcode,expected) in [
            (0x00ecu16,"(ll_pozicia <= 0)"),
            (0x00c8u16,"(ll_pozicia > 0)"),
            (0x0055u16,"(ll_pozicia + ll_dlzka_nove)"),
        ] {
            let mut stack=vec!["ll_pozicia".into(),
                if opcode==0x0055 {"ll_dlzka_nove".into()} else {"0".into()}];
            let op=FlowInsn {position:0,end:2,opcode,operands:Vec::new()};
            assert_eq!(flow_statement(&op,&sym,&mut stack,false),None);
            assert_eq!(stack,vec![expected.to_string()]);
        }
    }

    #[test]
    fn structured_loop_recognizer_accepts_single_guarded_backedge_only() {
        let blocks=vec![
            StructuredBlock {start:0x006a,end:0x007a},
            StructuredBlock {start:0x007a,end:0x00dc},
            StructuredBlock {start:0x00dc,end:0x00e8},
        ];
        let mut terminal=std::collections::BTreeMap::new();
        terminal.insert(0x006a,FlowInsn {
            position:0x0076,end:0x007a,opcode:0x0003,
            operands:vec![0xdc,0x00],
        });
        terminal.insert(0x007a,FlowInsn {
            position:0x00d8,end:0x00dc,opcode:0x0004,
            operands:vec![0x6a,0x00],
        });
        assert_eq!(simple_structured_loop(&blocks,&terminal,blocks[1]),
            Some((0x006a,0x00dc)));
        terminal.get_mut(&0x006a).unwrap().operands=vec![0xe8,0x00];
        assert_eq!(simple_structured_loop(&blocks,&terminal,blocks[1]),None);
    }

    #[test]
    fn completed_dbparm_assignment_is_catalog_verified_and_connect_bounded() {
        let mut stack_buffer=vec![0u8;18];
        stack_buffer[8..12].copy_from_slice(&0xffffu32.to_le_bytes());
        stack_buffer[12..14].copy_from_slice(&15u16.to_le_bytes());
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer,
            locals:vec![VariableSymbol {
                name:"at_sql".into(),global_index:None,type_id:0x8000,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:vec!["transaction".into()],
        };
        let catalog=vec![SystemCatalogObject {
            name:"transaction".into(),type_id:0x4084,parent_type:0xffff,
            all_member_count:16,instance_fields:(0..16).map(|i|
                if i==15 {"dbparm".into()} else {format!("p{i}")}).collect(),
            function_definitions:Vec::new(),
        }];
        let lines=vec![
            FlowInsn {position:0,end:4,opcode:0x0030,operands:vec![0,0]},
            FlowInsn {position:4,end:10,opcode:0x0020,operands:vec![8,0,0,0]},
            FlowInsn {position:10,end:14,opcode:0x0122,operands:vec![0,0]},
            FlowInsn {position:14,end:20,opcode:0x003b,operands:vec![0,0,0,0]},
            FlowInsn {position:20,end:22,opcode:0x0088,operands:vec![]},
            FlowInsn {position:22,end:26,opcode:0x0030,operands:vec![0,0]},
            FlowInsn {position:26,end:28,opcode:0x0005,operands:vec![]},
            FlowInsn {position:28,end:30,opcode:0x0000,operands:vec![]},
        ];
        assert_eq!(dbparm_completed_assignments(&lines,&sym,&catalog),
            vec![(26,2,4)]);
        let mut without_end=lines.clone();
        without_end[4].opcode=0x0139;
        assert!(dbparm_completed_assignments(&without_end,&sym,&catalog).is_empty());
        let mut without_catalog=catalog.clone();
        without_catalog[0].instance_fields[15]="some_other_field".into();
        assert!(dbparm_completed_assignments(&lines,&sym,&without_catalog).is_empty());
    }

    #[test]
    fn sql_composition_carries_prior_literal_through_reset_assign() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![VariableSymbol {
                name:"ls_pom".into(),global_index:None,type_id:0x0006,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        // First assignment initializes the buffer. Second appends two
        // hidden strings via ResetAssign, preserving the first value.
        let codes=[
            0x011d,0x003b,0x0088,
            0x011d,0x003b,0x003b,0x007d,0x0128,
            0x0119,0x007d,0x012a,0x0088,
        ];
        let lines=codes.into_iter().enumerate().map(|(idx,opcode)|FlowInsn {
            position:idx*6,end:(idx+1)*6,opcode,
            operands:vec![0,0,0,0],
        }).collect::<Vec<_>>();
        let refs=lines.iter().collect::<Vec<_>>();
        let (count,formula)=composed_sql_source_assignments(
            &refs,0,&sym,None
        ).unwrap();
        assert_eq!(count,2);
        assert!(formula.contains("<string@0x0006>"));
        assert!(formula.contains("<string@0x0018>"));
        assert!(formula.contains("<string@0x001E>"));
        assert!(!formula.split_once(" = ").unwrap().1.contains("ls_pom"));
    }

    #[test]
    fn sql_composition_refuses_unknown_initial_local_value() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![VariableSymbol {
                name:"ls_pom".into(),global_index:None,type_id:0x0006,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let codes=[0x011d,0x001e,0x0088];
        let lines=codes.into_iter().enumerate().map(|(idx,opcode)|FlowInsn {
            position:idx*6,end:(idx+1)*6,opcode,
            operands:vec![0,0,0,0],
        }).collect::<Vec<_>>();
        let refs=lines.iter().collect::<Vec<_>>();
        assert!(composed_sql_source_assignments(&refs,0,&sym,None).is_err());
    }

    #[test]
    fn sql_symbolic_assignment_shows_order_of_set_and_previous_value() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![VariableSymbol {
                name:"ls_pom".into(),global_index:None,type_id:0x0006,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let opcodes=[
            0x011d,0x003b,0x001e,0x007d,0x0034,0x01bc,
            0x01bd,0x007d,0x0088,
        ];
        let mut instructions=Vec::new();
        for (idx,opcode) in opcodes.into_iter().enumerate() {
            let operands=match opcode {
                0x01bc=>vec![0x53,0x01,0xd0,0x40],
                0x01bd=>vec![0,0,1,0],
                _=>vec![0,0,0,0],
            };
            instructions.push(FlowInsn {
                position:idx*6,end:(idx+1)*6,opcode,operands,
            });
        }
        let refs=instructions.iter().collect::<Vec<_>>();
        let formula=sql_assignment_expression(&refs,&sym,None,false).unwrap();
        assert_eq!(formula,
            "ls_pom = ((<string@0x0006> + ls_pom) + system_function_339(<number@0x0018>))");
        assert!(!formula.contains("Password"));
        instructions[7].opcode=0x020f;
        let broken=instructions.iter().collect::<Vec<_>>();
        assert!(sql_assignment_expression(&broken,&sym,None,false).is_err());
    }

    #[test]
    fn sql_symbolic_reset_assign_preserves_prior_value() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![VariableSymbol {
                name:"ls_pom".into(),global_index:None,type_id:0x0006,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let opcodes=[0x011d,0x003b,0x003b,0x007d,
                     0x0128,0x0119,0x007d,0x012a,0x0088];
        let instructions=opcodes.into_iter().enumerate()
            .map(|(idx,opcode)|FlowInsn {
                position:idx*6,end:(idx+1)*6,
                opcode,operands:vec![0,0,0,0],
            }).collect::<Vec<_>>();
        let refs=instructions.iter().collect::<Vec<_>>();
        assert_eq!(
            sql_assignment_expression(&refs,&sym,None,false).unwrap(),
            "ls_pom = (ls_pom + (<string@0x0006> + <string@0x000C>))");
    }


    #[test]
    fn reset_assign_literal_append_is_not_mistaken_for_replacement() {
        let mut pool=vec![0u8;16];
        let mut literal_offset=Vec::new();
        for value in ["A","B","C"] {
            literal_offset.push(pool.len() as u32);
            for unit in value.encode_utf16().chain(std::iter::once(0)) {
                pool.extend_from_slice(&unit.to_le_bytes());
            }
        }
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:pool,locals:Vec::new(),
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let codes=[
            0x011d,0x003b,0x003b,0x007d,0x0128,
            0x003b,0x007d,0x0129,0x0119,0x007d,0x012a,0x0088,
        ];
        let mut literal_index=0usize;
        let mut ins=Vec::<FlowInsn>::new();
        for (i,opcode) in codes.into_iter().enumerate() {
            let operands=if opcode==0x003b {
                let ret=literal_offset[literal_index].to_le_bytes().to_vec();
                literal_index+=1;
                ret
            } else {vec![0,0,0,0]};
            ins.push(FlowInsn{
                position:i*6,end:(i+1)*6,opcode,operands,
            });
        }
        let refs=ins.iter().collect::<Vec<_>>();
        assert_eq!(recognized_literal_append(&refs,&sym,false),Some((3,None)));
        assert_eq!(recognized_literal_append(&refs,&sym,true),
            Some((3,Some("ABC".into()))));
        ins[8].opcode=0x0139; // No ResetAssign -> not an append.
        let wrong=ins.iter().collect::<Vec<_>>();
        assert!(recognized_literal_append(&wrong,&sym,false).is_none());
    }


    #[test]
    fn final_sql_rewrite_resolves_catalog_function_without_exposing_literals() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![VariableSymbol {
                name:"ls_pom".into(),global_index:None,type_id:0x0006,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let opcodes=[
            0x011d,0x001e,0x001e,0x0199,0x0034,0x005c,
            0x01bc,0x01bd,0x013b,0x003b,0x007d,0x0129,0x0088,
        ];
        let mut instructions=Vec::new();
        for (i,opcode) in opcodes.into_iter().enumerate() {
            let operands=match opcode {
                0x01bc=>vec![139,0,0xd0,0x40],
                0x01bd=>vec![0,0,2,0],
                _=>vec![0,0,0,0],
            };
            instructions.push(FlowInsn{
                position:i*6,end:(i+1)*6,opcode,operands,
            });
        }
        let refs=instructions.iter().collect::<Vec<_>>();
        let mut functions=vec!["unmapped".to_string();140];
        functions[139]="Left".into();
        let objects=vec![SystemCatalogObject {
            name:"systemfunctions".into(),type_id:0x40d0,parent_type:0xffff,
            all_member_count:0,instance_fields:Vec::new(),
            function_definitions:functions,
        }];
        assert_eq!(
            final_sql_source_rewrite(&refs,&sym,Some(&objects)),
            Some("ls_pom = Left(ls_pom, Len(ls_pom) - <integer-hidden>) + <string-hidden>".into())
        );
        let preview=final_sql_source_rewrite(&refs,&sym,None).unwrap();
        assert!(preview.contains("system_function_139"));
        assert!(!preview.contains("Left("));
        instructions[3].opcode=0x0198;
        let wrong=instructions.iter().collect::<Vec<_>>();
        assert!(final_sql_source_rewrite(&wrong,&sym,Some(&objects)).is_none());
    }

    #[test]
    fn builtin_len_and_subtraction_have_stack_effects() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:Vec::new(),globals:Vec::new(),
            referenced_functions:Vec::new(),sql_pool:Vec::new(),
            type_names:Vec::new(),
        };
        let mut stack=vec!["ls_pom".into()];
        let len=FlowInsn {position:0,end:2,opcode:0x0199,operands:vec![]};
        flow_statement(&len,&sym,&mut stack,false);
        assert_eq!(stack,vec!["Len(ls_pom)".to_string()]);
        stack.push("<number-hidden>".into());
        let subtraction=FlowInsn {position:2,end:4,opcode:0x005c,operands:vec![]};
        flow_statement(&subtraction,&sym,&mut stack,false);
        assert_eq!(stack,vec!["(Len(ls_pom) - <number-hidden>)".to_string()]);
    }

    #[test]
    fn explicit_sensitive_literal_mode_is_opt_in() {
        let mut stack=vec![0u8;16];
        let secret="s3cr3t-user:Password=abc";
        for c in secret.encode_utf16().chain(std::iter::once(0)) {
            stack.extend_from_slice(&c.to_le_bytes());
        }
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:stack,locals:Vec::new(),
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let inst=FlowInsn {position:0,end:6,opcode:0x003b,
            operands:(16u32).to_le_bytes().to_vec()};
        assert!(!redacted_gate_instruction(&sym,&inst,false).contains("s3cr3t"));
        assert!(redacted_gate_instruction(&sym,&inst,true).contains(secret));
        let integer=FlowInsn {position:6,end:12,opcode:0x0034,
            operands:12345i32.to_le_bytes().to_vec()};
        assert!(!redacted_gate_instruction(&sym,&integer,false).contains("12345"));
        assert!(redacted_gate_instruction(&sym,&integer,true).contains("12345"));
    }


    #[test]
    fn sql_literal_classifier_only_reports_whitelisted_categories() {
        assert_eq!(sql_literal_intent("  set option foo=bar"),
            Some("session-setting candidate (SET)"));
        assert_eq!(sql_literal_intent("select * from patients"),
            Some("query-like prefix (SELECT/WITH)"));
        assert_eq!(sql_literal_intent("update secret_table set x=1"),
            Some("potential data-changing DML"));
        assert_eq!(sql_literal_intent("CREATE VIEW v AS SELECT 1"),
            Some("potential schema-changing DDL"));
        assert_eq!(sql_literal_intent("set_password=something"),None);
        assert_eq!(sql_literal_intent("password123"),None);
        assert_eq!(sql_literal_intent(""),None);
    }


    #[test]
    fn type_verified_transaction_sqlcode_return_has_no_secret_data() {
        let mut blob=vec![0u8;20];
        blob[8..12].copy_from_slice(&0xffffu32.to_le_bytes());
        blob[12..14].copy_from_slice(&9u16.to_le_bytes());
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:blob,
            locals:vec![VariableSymbol {
                name:"at_sql".into(),global_index:None,type_id:0x8000,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:vec!["transaction".into()],
        };
        let objects=vec![SystemCatalogObject {
            name:"transaction".into(),type_id:0x4084,parent_type:0xffff,
            all_member_count:17,
            instance_fields:(0..17).map(|i|
                if i==9 {"sqlcode".into()} else {format!("member{i}")}).collect(),
            function_definitions:Vec::new(),
        }];
        let lines=vec![
            FlowInsn {position:0x2aa,end:0x2ae,opcode:0x0030,operands:vec![0,0]},
            FlowInsn {position:0x2ae,end:0x2b4,opcode:0x0020,operands:vec![8,0,0,0]},
            FlowInsn {position:0x2b4,end:0x2b8,opcode:0x0027,operands:vec![0,0]},
            FlowInsn {position:0x2b8,end:0x2bc,opcode:0x008e,operands:vec![0,0]},
            FlowInsn {position:0x2bc,end:0x2c0,opcode:0x0001,operands:vec![1,0]},
        ];
        assert_eq!(
            safe_immediate_return_hint(&lines,4,&sym,Some(&objects)),
            "RETURN at_sql.sqlcode (system catalog verified)");
        assert!(safe_immediate_return_hint(&lines,4,&sym,None)
            .contains("source not proven"));
    }

    #[test]
    fn transaction_return_requires_type_matched_field_descriptor() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:vec![0u8;16],
            locals:vec![VariableSymbol {
                name:"at_sql".into(),global_index:None,type_id:0x0006,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let objects=vec![SystemCatalogObject {
            name:"transaction".into(),type_id:0x4084,parent_type:0xffff,
            all_member_count:17,instance_fields:vec!["x".into();17],
            function_definitions:Vec::new(),
        }];
        let lines=vec![
            FlowInsn {position:0,end:4,opcode:0x0030,operands:vec![0,0]},
            FlowInsn {position:4,end:10,opcode:0x0020,operands:vec![8,0,0,0]},
            FlowInsn {position:10,end:14,opcode:0x0027,operands:vec![0,0]},
            FlowInsn {position:14,end:18,opcode:0x0001,operands:vec![1,0]},
        ];
        assert_eq!(direct_transaction_field_return(&lines,3,&sym,Some(&objects)),None);
    }


    #[test]
    fn path_sensitive_transaction_assignment_does_not_inherit_skipped_branch() {
        let mut buffer=vec![0u8;18];
        buffer[8..12].copy_from_slice(&0xffffu32.to_le_bytes());
        buffer[12..14].copy_from_slice(&15u16.to_le_bytes());
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:buffer,
            locals:vec![VariableSymbol {
                name:"at_sql".into(),global_index:None,type_id:0x8000,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:vec!["transaction".into()],
        };
        let objects=vec![SystemCatalogObject {
            name:"transaction".into(),type_id:0x4084,parent_type:0xffff,
            all_member_count:16,instance_fields:(0..16).map(|i|
                if i==15 {"dbparm".into()} else {format!("p{i}")}).collect(),
            function_definitions:Vec::new(),
        }];
        let lines=vec![
            FlowInsn {position:0,end:4,opcode:0x0003,operands:vec![30,0]},
            FlowInsn {position:4,end:8,opcode:0x0030,operands:vec![0,0]},
            FlowInsn {position:8,end:14,opcode:0x0020,operands:vec![8,0,0,0]},
            FlowInsn {position:14,end:18,opcode:0x0122,operands:vec![0,0]},
            FlowInsn {position:18,end:20,opcode:0x0088,operands:vec![]},
            FlowInsn {position:20,end:22,opcode:0x0005,operands:vec![]},
            FlowInsn {position:22,end:24,opcode:0x0000,operands:vec![]},
            FlowInsn {position:30,end:32,opcode:0x0005,operands:vec![]},
            FlowInsn {position:32,end:34,opcode:0x0000,operands:vec![]},
        ];
        let states=transaction_assignment_path_states(&lines,&sym,&objects);
        assert_eq!(states.get(&20).unwrap().iter().copied().collect::<Vec<_>>(),
            vec![4u8]);
        assert_eq!(states.get(&30).unwrap().iter().copied().collect::<Vec<_>>(),
            vec![0u8]);
    }


    #[test]
    fn return_contract_only_shows_whitelisted_status_literals() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:Vec::new(),globals:Vec::new(),
            referenced_functions:Vec::new(),sql_pool:Vec::new(),
            type_names:Vec::new(),
        };
        for (value,hint) in [
            (0i32,"RETURN; preceding status literal 0"),
            (1,"RETURN; preceding status literal 1"),
            (-1,"RETURN; preceding status literal -1"),
            (12345,"RETURN; preceding numeric literal redacted"),
        ] {
            let lines=vec![
                FlowInsn {position:0,end:6,opcode:0x0034,
                    operands:value.to_le_bytes().to_vec()},
                FlowInsn {position:6,end:10,opcode:0x0001,
                    operands:vec![1,0]},
            ];
            assert_eq!(safe_immediate_return_hint(&lines,1,&sym,None),hint);
        }
    }

    #[test]
    fn return_contract_reachability_rejects_dead_epilogue() {
        let lines=vec![
            FlowInsn {position:0,end:4,opcode:0x0003,
                operands:vec![10,0]},
            FlowInsn {position:4,end:8,opcode:0x0001,
                operands:vec![0,0]},
            FlowInsn {position:8,end:10,opcode:0x0000,
                operands:vec![]}, // unreachable
            FlowInsn {position:10,end:14,opcode:0x0001,
                operands:vec![0,0]},
        ];
        assert_eq!(reachable_return_indices(&lines),vec![1,3]);
    }


    #[test]
    fn pb333_connection_selector_is_upper_of_as_typ_without_exposing_modes() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![
                VariableSymbol {name:"at_sql".into(),global_index:None,type_id:0x8000},
                VariableSymbol {name:"as_typ".into(),global_index:None,type_id:0x0006},
                VariableSymbol {name:"<variable#9>".into(),global_index:None,type_id:0x0006},
            ],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let ins=vec![
            FlowInsn {position:0x007c,end:0x0080,opcode:0x011d,
                operands:vec![2,0]},
            FlowInsn {position:0x0080,end:0x0084,opcode:0x001e,
                operands:vec![1,0]},
            FlowInsn {position:0x0084,end:0x0088,opcode:0x01a4,
                operands:vec![0,0]},
            FlowInsn {position:0x0088,end:0x008c,opcode:0x0088,
                operands:vec![0,0]},
            FlowInsn {position:0x008c,end:0x0092,opcode:0x003b,
                operands:vec![0,0,0,0]},
            FlowInsn {position:0x0092,end:0x0096,opcode:0x001e,
                operands:vec![2,0]},
            FlowInsn {position:0x0096,end:0x009c,opcode:0x00ad,
                operands:vec![0,0,0,0]},
            FlowInsn {position:0x009c,end:0x00a0,opcode:0x0003,
                operands:vec![0xc8,0x02]},
            FlowInsn {position:0x02c8,end:0x02ce,opcode:0x003b,
                operands:vec![0,0,0,0]},
            FlowInsn {position:0x02ce,end:0x02d2,opcode:0x001e,
                operands:vec![2,0]},
            FlowInsn {position:0x02d2,end:0x02d8,opcode:0x00ad,
                operands:vec![0,0,0,0]},
            FlowInsn {position:0x02d8,end:0x02dc,opcode:0x0003,
                operands:vec![0xf0,0x04]},
        ];
        for offset in [0x009cusize,0x02d8usize] {
            assert_eq!(
                pb333_connection_mode_predicate(&ins,&sym,offset),
                Some("(Upper(as_typ) = <string-hidden>)".into()));
            assert_eq!(
                redacted_entry_gate_predicate(&ins,&sym,offset),
                "(Upper(as_typ) = <string-hidden>)");
        }
        let mut stack=vec!["as_typ".to_string()];
        let builtin=FlowInsn {position:0,end:4,opcode:0x01a4,
            operands:vec![0,0]};
        assert_eq!(flow_statement(&builtin,&sym,&mut stack,false),None);
        assert_eq!(stack,vec!["Upper(as_typ)".to_string()]);
    }

    #[test]
    fn connection_selector_rejects_non_string_or_non_upper_source() {
        let mut sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![
                VariableSymbol {name:"as_typ".into(),global_index:None,type_id:0x0006},
                VariableSymbol {name:"mode".into(),global_index:None,type_id:0x0006},
            ],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let mut ins=vec![
            FlowInsn {position:0,end:4,opcode:0x011d,operands:vec![1,0]},
            FlowInsn {position:4,end:8,opcode:0x001e,operands:vec![0,0]},
            FlowInsn {position:8,end:12,opcode:0x01a4,operands:vec![0,0]},
            FlowInsn {position:12,end:16,opcode:0x0088,operands:vec![0,0]},
            FlowInsn {position:16,end:22,opcode:0x003b,operands:vec![0,0,0,0]},
            FlowInsn {position:22,end:26,opcode:0x001e,operands:vec![1,0]},
            FlowInsn {position:26,end:32,opcode:0x00ad,operands:vec![0,0,0,0]},
            FlowInsn {position:32,end:36,opcode:0x0003,operands:vec![40,0]},
        ];
        assert!(pb333_connection_mode_predicate(&ins,&sym,32).is_some());
        sym.locals[0].type_id=0x0001;
        assert!(pb333_connection_mode_predicate(&ins,&sym,32).is_none());
        sym.locals[0].type_id=0x0006;
        ins[2].opcode=0x01a3;
        assert!(pb333_connection_mode_predicate(&ins,&sym,32).is_none());
    }

    #[test]
    fn gate_diagnostics_suppress_literal_bytes_and_numeric_values() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![VariableSymbol {
                name:"<variable#9>".into(),global_index:None,type_id:0x0006,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let numeric=FlowInsn {position:0,end:6,opcode:0x0034,
            operands:vec![0x39,0x30,0,0]};
        let literal=redacted_gate_instruction(&sym,&numeric,false);
        assert!(literal.contains("<redacted>"));
        assert!(!literal.contains("12345"));
        let unnamed=FlowInsn {position:6,end:10,opcode:0x0030,
            operands:vec![0x00,0x00]};
        let description=redacted_gate_instruction(&sym,&unnamed,false);
        assert!(description.contains("local[0]"));
        assert!(!description.contains("<variable#9>"));
        assert!(description.contains("primitive type"));
    }

    #[test]
    fn gate_trace_start_uses_jump_target_not_arbitrary_window() {
        let lines=vec![
            FlowInsn {position:0,end:4,opcode:0x0003,operands:vec![20,0]},
            FlowInsn {position:4,end:10,opcode:0x003b,operands:vec![0,0,0,0]},
            FlowInsn {position:20,end:24,opcode:0x0030,operands:vec![0,0]},
            FlowInsn {position:24,end:28,opcode:0x0003,operands:vec![40,0]},
        ];
        assert_eq!(gate_trace_block_start(&lines,24),Some(20));
    }

    #[test]
    fn entry_gate_condition_is_redacted_and_preserves_variable_identity() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![VariableSymbol {
                name:"mode".into(),global_index:None,type_id:0x0001,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let lines=vec![
            FlowInsn {position:0,end:4,opcode:0x0030,operands:vec![0,0]},
            FlowInsn {position:4,end:10,opcode:0x0034,operands:vec![37,0,0,0]},
            FlowInsn {position:10,end:12,opcode:0x00a8,operands:vec![]},
            FlowInsn {position:12,end:16,opcode:0x0003,operands:vec![20,0]},
        ];
        let predicate=redacted_entry_gate_predicate(&lines,&sym,12);
        assert_eq!(predicate,"(mode = <number-hidden>)");
        assert!(!predicate.contains("37"));
    }

    #[test]
    fn entry_gate_never_invents_condition_crossing_basic_block() {
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:Vec::new(),globals:Vec::new(),
            referenced_functions:Vec::new(),sql_pool:Vec::new(),
            type_names:Vec::new(),
        };
        let lines=vec![
            FlowInsn {position:0,end:4,opcode:0x0004,operands:vec![10,0]},
            FlowInsn {position:10,end:12,opcode:0x00a8,operands:vec![]},
            FlowInsn {position:12,end:16,opcode:0x0002,operands:vec![20,0]},
        ];
        assert!(redacted_entry_gate_predicate(&lines,&sym,12)
            .contains("partially unresolved"));
    }

    #[test]
    fn entry_routing_stops_at_first_connection_and_identifies_alternative_gates() {
        let lines=vec![
            FlowInsn {position:0,end:4,opcode:0x0003,operands:vec![20,0]},
            FlowInsn {position:4,end:8,opcode:0x0002,operands:vec![12,0]},
            FlowInsn {position:8,end:10,opcode:0x0005,operands:vec![]},
            FlowInsn {position:10,end:12,opcode:0x0000,operands:vec![]},
            FlowInsn {position:12,end:14,opcode:0x0005,operands:vec![]},
            FlowInsn {position:14,end:16,opcode:0x0000,operands:vec![]},
            FlowInsn {position:20,end:22,opcode:0x0005,operands:vec![]},
            FlowInsn {position:22,end:24,opcode:0x0000,operands:vec![]},
        ];
        let entry=next_connect_endpoints(&lines,0);
        assert_eq!(entry,vec![
            "CONNECT at 0x0008".to_string(),
            "CONNECT at 0x000C".to_string(),
            "CONNECT at 0x0014".to_string(),
        ]);
        let gates=preconnect_branch_gates(&lines);
        assert_eq!(gates.len(),2);
        assert_eq!(gates[0].0,0);
        assert_eq!(gates[1].0,4);
        assert_eq!(gates[0].2,vec!["CONNECT at 0x0014".to_string()]);
        assert_eq!(gates[1].2,vec!["CONNECT at 0x000C".to_string()]);
    }


    #[test]
    fn sqlcode_guard_reconstructs_success_and_failure_for_not_equal() {
        let mut buffer=vec![0u8;16];
        buffer[8..12].copy_from_slice(&0xffffu32.to_le_bytes());
        buffer[12..14].copy_from_slice(&2u16.to_le_bytes());
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:buffer,
            locals:vec![VariableSymbol {
                name:"at_sql".into(),global_index:None,type_id:0x8000,
            }],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:vec!["transaction".into()],
        };
        let objects=vec![SystemCatalogObject {
            name:"transaction".into(),type_id:0x4084,parent_type:0xffff,
            all_member_count:3,
            instance_fields:vec!["dbms".into(),"database".into(),"sqlcode".into()],
            function_definitions:Vec::new(),
        }];
        let instructions=vec![
            FlowInsn {position:0,end:2,opcode:0x0005,operands:vec![]},
            FlowInsn {position:2,end:6,opcode:0x0030,operands:vec![0,0]},
            FlowInsn {position:6,end:12,opcode:0x0020,operands:vec![8,0,0,0]},
            FlowInsn {position:12,end:16,opcode:0x0027,operands:vec![0,0]},
            FlowInsn {position:16,end:22,opcode:0x0034,operands:vec![0,0,0,0]},
            FlowInsn {position:22,end:24,opcode:0x00b8,operands:vec![]},
            FlowInsn {position:24,end:28,opcode:0x0003,operands:vec![40,0]},
            FlowInsn {position:28,end:30,opcode:0x0005,operands:vec![]},
            FlowInsn {position:40,end:42,opcode:0x0000,operands:vec![]},
        ];
        let guard=connect_sqlcode_guard(&sym,&instructions,0,Some(&objects))
            .expect("SQLCode conditional recognized");
        assert_eq!(guard.field_name,"sqlcode");
        assert_eq!(guard.zero_dest,40);
        assert_eq!(guard.nonzero_dest,28);
        assert_eq!(next_connect_endpoints(&instructions,guard.zero_dest),
            vec!["RETURN at 0x0028".to_string()]);
        assert_eq!(next_connect_endpoints(&instructions,guard.nonzero_dest),
            vec!["CONNECT at 0x001C".to_string()]);
        assert!(connect_sqlcode_guard(&sym,&instructions,0,None).is_none());
    }

    #[test]
    fn branch_reachability_reports_both_unknown_outcomes() {
        let instructions=vec![
            FlowInsn {position:0,end:4,opcode:0x0003,operands:vec![10,0]},
            FlowInsn {position:4,end:6,opcode:0x0005,operands:vec![]},
            FlowInsn {position:10,end:12,opcode:0x0001,operands:vec![0,0]},
        ];
        let found=next_connect_endpoints(&instructions,0);
        assert_eq!(found,vec!["CONNECT at 0x0004".to_string(),
                               "RETURN at 0x000A".to_string()]);
    }


    #[test]
    fn dbparm_key_detector_never_returns_secret_literal() {
        let mut stack=vec![0u8;24];
        let secret="ConnectString='DSN=SecretDB;UID=somebody;PWD=NeverPrintMe'";
        for unit in secret.encode_utf16().chain(std::iter::once(0)) {
            stack.extend_from_slice(&unit.to_le_bytes());
        }
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:stack,locals:Vec::new(),
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        let ins=FlowInsn {
            position:0,end:6,opcode:0x003b,
            operands:(24u32).to_le_bytes().to_vec(),
        };
        let keys=known_dbparm_keys(&sym,&[ins]);
        assert!(keys.contains(&"DSN"));
        assert!(keys.contains(&"UID"));
        assert!(keys.contains(&"PWD"));
        assert!(keys.contains(&"ConnectString"));
        assert!(!keys.join(",").contains("NeverPrintMe"));
        assert!(!keys.join(",").contains("somebody"));
    }

    #[test]
    fn catalogs_match_only_a_typed_transaction_object() {
        let var=VariableSymbol {
            name:"at_sql".into(),global_index:None,type_id:0x8000,
        };
        let mut buffer=vec![0u8;24];
        buffer[8..12].copy_from_slice(&0xffffu32.to_le_bytes());
        buffer[12..14].copy_from_slice(&15u16.to_le_bytes());
        let sym=FunctionSymbols {
            code:Vec::new(),stack_buffer:buffer,
            locals:vec![var],globals:Vec::new(),
            referenced_functions:Vec::new(),sql_pool:Vec::new(),
            type_names:vec!["transaction".into()],
        };
        let obj=SystemCatalogObject {
            name:"transaction".into(),type_id:0x4084,parent_type:0xffff,
            all_member_count:16,instance_fields:(0..16).map(|i|
                if i==15 {"dbparm".into()} else {format!("property{i}")}).collect(),
            function_definitions:Vec::new(),
        };
        let ref_local=FlowInsn {
            position:0,end:4,opcode:0x0030,operands:vec![0,0]
        };
        let field=FlowInsn {
            position:4,end:10,opcode:0x0020,operands:vec![8,0,0,0]
        };
        let objects=vec![obj];
        assert_eq!(
            catalog_transaction_property(&sym,&ref_local,&field,&objects),
            Some("dbparm".into()));
    }


    #[test]
    fn pb333_system_function_uses_definition_index_not_global_id() {
        let obj=SystemCatalogObject {
            name:"SystemFunctions".into(),type_id:0x40d0,parent_type:0xFFFF,
            all_member_count:0,instance_fields:Vec::new(),
            function_definitions:(0..340).map(|idx|
                if idx==339 {"documented_sample_function".into()}
                else {format!("other_{idx}")}).collect(),
        };
        assert_eq!(obj.function_definitions.get(339).map(String::as_str),
            Some("documented_sample_function"));
    }


    #[test]
    fn pb333_casts_do_not_clear_expression_stack() {
        let symbols = FunctionSymbols {
            code: Vec::new(), stack_buffer: Vec::new(),
            locals: Vec::new(), globals: Vec::new(),
            referenced_functions: Vec::new(), sql_pool: Vec::new(),
            type_names: Vec::new(),
        };
        let mut stack=vec!["as_pouzivatel".to_string()];
        for &opcode in &[0x0128u16,0x012bu16,0x012au16,0x0133u16] {
            let operands=vec![0u8; OPERAND_WORDS[opcode as usize] as usize*2];
            let ins=FlowInsn {position:0,end:2+operands.len(),opcode,operands};
            assert_eq!(flow_statement(&ins,&symbols,&mut stack,false),None);
            assert_eq!(stack,vec!["as_pouzivatel".to_string()]);
        }
    }

    #[test]
    fn system_function_reference_retains_object_and_index_not_fake_name() {
        let symbols = FunctionSymbols {
            code: Vec::new(), stack_buffer: Vec::new(),
            locals: Vec::new(), globals: Vec::new(),
            referenced_functions: Vec::new(), sql_pool: Vec::new(),
            type_names: Vec::new(),
        };
        let ins=FlowInsn {
            position:0,end:6,opcode:0x01bc,
            operands:vec![0x53,0x01,0x01,0x40],
        };
        let mut stack=Vec::new();
        assert_eq!(flow_statement(&ins,&symbols,&mut stack,false),None);
        assert_eq!(stack,vec!["<system-function object=0x4001 index=339>".to_string()]);
    }



    #[test]
    fn catalog_members_resolve_own_and_inherited_slots() {
        let base=SystemCatalogObject {
            name:"base".into(),type_id:0x4001,parent_type:0xFFFF,
            all_member_count:2,instance_fields:vec!["parent_a".into(),"parent_b".into()],
            function_definitions:Vec::new(),
        };
        let trans=SystemCatalogObject {
            name:"transaction".into(),type_id:0x4013,parent_type:0x4001,
            all_member_count:4,instance_fields:vec!["dbms".into(),"logpass".into()],
            function_definitions:Vec::new(),
        };
        let objects=vec![base,trans];
        assert_eq!(system_property_name(&objects,0x4013,0,0),Some("parent_a".into()));
        assert_eq!(system_property_name(&objects,0x4013,3,0),Some("logpass".into()));
        assert_eq!(system_property_name(&objects,0x4013,9,0),None);
    }
    #[test]
    fn type_metadata_reports_system_index_without_guessing_property() {
        assert!(describe_type_id(0x4013,&[]).contains("system catalog required"));
        assert_eq!(
            describe_type_id(0x8000,&["transaction".to_string()]),
            "local type 0x8000 (transaction)"
        );
    }

    #[test]
    fn index_only_transaction_member_does_not_guess_name() {
        let mut buffer = vec![0u8; 32];
        buffer[8..12].copy_from_slice(&0x0000FFFFu32.to_le_bytes());
        buffer[12..14].copy_from_slice(&123u16.to_le_bytes());
        let symbols=FunctionSymbols {
            code:Vec::new(),stack_buffer:buffer,
            locals:Vec::new(),globals:Vec::new(),
            referenced_functions:Vec::new(),sql_pool:Vec::new(),
            type_names:Vec::new(),
        };
        let metadata=field_descriptor_metadata(&symbols,&8u16.to_le_bytes());
        assert!(metadata.contains("type-indexed member=#123"));
        assert!(!metadata.contains("DBPass"));
    }


    #[test]
    fn reads_local_transaction_operand_for_connect() {
        let symbols = FunctionSymbols {
            code:Vec::new(),stack_buffer:Vec::new(),
            locals:vec![VariableSymbol {name:"atrans".into(),global_index:None,type_id:0x4000}],
            globals:Vec::new(),referenced_functions:Vec::new(),
            sql_pool:Vec::new(),type_names:Vec::new(),
        };
        assert_eq!(flow_local(&symbols,&[0,0]),"atrans");
        let mut stack=Vec::new();
        let push=FlowInsn {position:0,end:4,opcode:0x0030,operands:vec![0,0]};
        assert_eq!(flow_statement(&push,&symbols,&mut stack,false),None);
        let connect=FlowInsn {position:4,end:6,opcode:0x0005,operands:Vec::new()};
        assert_eq!(flow_statement(&connect,&symbols,&mut stack,false),
                   Some("CONNECT USING atrans".into()));
    }


    #[test]
    fn connect_opcode_emits_transaction_operation_without_secrets() {
        let symbol = FunctionSymbols {
            code: Vec::new(), stack_buffer: Vec::new(),
            locals: Vec::new(), globals: Vec::new(),
            referenced_functions: Vec::new(), sql_pool: Vec::new(),
            type_names:Vec::new(),
        };
        let instruction = FlowInsn {
            position: 0, end: 2, opcode: 0x0005,
            operands: Vec::new(),
        };
        let mut stack = vec!["sqlca".to_string()];
        assert_eq!(
            flow_statement(&instruction,&symbol,&mut stack,false),
            Some("CONNECT USING sqlca".to_string())
        );
        assert!(stack.is_empty());
        let (instruction_count, frequency)=decode_pcode(&[0x05,0x00]).unwrap();
        assert_eq!(instruction_count,1);
        assert_eq!(frequency.get(&0x0005),Some(&1));
    }


    #[test]
    fn known_constant_return_pattern() {
        let bytes = [
            0x34, 0x00, 0x00, 0x80, 0x00, 0x00,
            0x01, 0x00, 0x01, 0x00,
            0x04, 0x00, 0x0e, 0x00,
            0x00, 0x00,
        ];
        assert_eq!(simple_constant_return(&bytes), Some(32768));
        assert_eq!(opcode_note(0x0034, &bytes[2..6]),
                   "signed 32-bit integer literal: 32768");
        assert!(opcode_note(0x0004, &[0x0e, 0x00]).contains("0x000E"));
    }

    #[test]
    fn reject_unrecognized_epilogue() {
        let bytes = [
            0x34, 0x00, 0x00, 0x80, 0x00, 0x00,
            0x01, 0x00, 0x01, 0x00,
            0x04, 0x00, 0x0c, 0x00,
            0x00, 0x00,
        ];
        assert_eq!(simple_constant_return(&bytes), None);
    }
}


#[cfg(test)]
mod sql_cursor_interpolation_tests {
    use super::*;

    #[test]
    fn resolves_high_bit_flagged_sql_placeholder_pointer() {
        let template = "select * from T where a=' ' and b=' '";
        let mut pool = vec![0u8; 1024];
        // Terminal SQL cursor record at 0x10.
        pool[0x18..0x1c].copy_from_slice(&65535u32.to_le_bytes());
        pool[0x20..0x24].copy_from_slice(&0x8000_0060u32.to_le_bytes());
        pool[0x28..0x2c].copy_from_slice(&0x8000_0100u32.to_le_bytes());
        for (i, (start, _)) in template.match_indices("' '").enumerate() {
            let start = start as u16; // fixture is ASCII: bytes == UTF-16 units
            let pos = 0x60 + i * 4;
            pool[pos..pos+2].copy_from_slice(&start.to_le_bytes());
            pool[pos+2..pos+4].copy_from_slice(&(start + 3).to_le_bytes());
        }
        for (i, unit) in template.encode_utf16().enumerate() {
            let pos = 0x100 + i * 2;
            pool[pos..pos+2].copy_from_slice(&unit.to_le_bytes());
        }
        let symbols = FunctionSymbols {
            code: Vec::new(), stack_buffer: Vec::new(),
            locals: Vec::new(), globals: Vec::new(),
            referenced_functions: Vec::new(), sql_pool: pool,
            type_names:Vec::new(),
        };
        let sql = sql_with_bound_params(
            &symbols, &0x8000_0010u32.to_le_bytes(),
            &["rod_cis".to_string(), "kod_odd".to_string()], true,
        );
        assert_eq!(sql,
            "select * from T where a=:rod_cis and b=:kod_odd");
    }


    #[test]
    fn inserts_parameters_at_utf16_ranges() {
        // The original SQL contains two placeholder spans (question marks).
        // These positions are UTF-16 units, as in PbdViewer GetCursor().
        let original="select č from T where a=? and b=?";
        let utf16: Vec<u16>=original.encode_utf16().collect();
        let marks: Vec<usize>=utf16.iter().enumerate()
            .filter_map(|(index,&unit)| (unit==b'?' as u16).then_some(index))
            .collect();
        assert_eq!(marks.len(),2);
        let mut pool=Vec::new();
        for m in marks {
            pool.extend_from_slice(&(m as u16).to_le_bytes());
            pool.extend_from_slice(&((m+1) as u16).to_le_bytes());
        }
        let sql=substitute_sql_ranges(
            original,&pool,0,&["gch_rod_cis".into(),"gch_kod_odd".into()]
        ).unwrap();
        assert_eq!(sql,
            "select č from T where a=:gch_rod_cis and b=:gch_kod_odd");
    }

    #[test]
    fn rejects_malformed_parameter_ranges() {
        let params=["x".to_string()];
        assert!(substitute_sql_ranges("SELECT ?", &[0,0,0,0], 0, &params).is_err());
        assert!(substitute_sql_ranges("SELECT ?", &[99,0,100,0], 0, &params).is_err());
    }

    #[test]
    fn inserts_select_into_before_from_case_insensitively() {
        let src="SELECT val FROM tab WHERE x=:x".into();
        let result=sql_with_into(src,&["ll_riadok".into()],true);
        assert_eq!(result,
            "SELECT val INTO :ll_riadok FROM tab WHERE x=:x");
        assert_eq!(sql_with_into("SELECT val FROM tab".into(),&[],false),
            "SELECT val FROM tab");
    }
}
