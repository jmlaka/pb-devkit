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


// Extract a chosen P-code body from an already-validated PB333 .fun entry.
// Body indices are zero-based and count all bodies inside the entry.
fn find_pcode_body(data: &[u8], selected: usize) -> Result<Vec<u8>, String> {
    let mut r = Cursor::new(data);
    if r.u16()? != 333 {
        return Err("not a version-333 compiled entry".into());
    }
    r.bytes(22)?;
    let descriptor_count = r.u16()? as usize;
    r.bytes(descriptor_count * 12)?;
    r.struct_buffer()?;
    r.records20()?;
    let object_count = r.u16()? as usize;
    let extended_count = r.u16()? as usize;
    r.struct_buffer()?;
    r.struct_buffer()?;
    r.records20()?;
    r.records20()?;

    let object_indices = r.bytes(object_count * 16)?;
    let extended_headers = r.bytes(extended_count * 32)?;
    let mut extended_used = 0usize;
    let mut body_index = 0usize;

    for object in object_indices.chunks_exact(16) {
        match (object[0] >> 1) & 7 {
            0 => {
                let header_start = extended_used * 32;
                let header = extended_headers.get(header_start..header_start + 32)
                    .ok_or_else(|| "missing extended object header".to_string())?;
                extended_used += 1;

                let bodies = r.u16()? as usize;
                r.bytes(bodies * 4)?;
                for _ in 0..bodies {
                    let code_length = r.u16()? as usize;
                    let debug_count = r.u16()? as usize;
                    r.u16()?;
                    let code = r.bytes(code_length)?;
                    if body_index == selected {
                        return Ok(code.to_vec());
                    }
                    body_index += 1;
                    r.bytes(debug_count * 4)?;
                    r.records20()?;
                    r.struct_buffer()?;
                }

                r.bytes(word(header, 24)? * 6)?;
                r.bytes(word(header, 22)? * 4)?;
                r.records20()?;
                r.records20()?;
                r.bytes(word(header, 28)? * 8)?;
                r.bytes(word(header, 26)? * 16)?;
                r.bytes(word(header, 4)? * 48)?;
            }
            1 => {
                r.bytes(word(object, 4)? * 8)?;
            }
            _ => {}
        }
    }

    Err(format!(
        "P-code body index {} not found (available: 0..{})",
        selected, body_index
    ))
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
}

struct FunctionSymbols {
    code: Vec<u8>,
    stack_buffer: Vec<u8>,
    locals: Vec<VariableSymbol>,
    globals: Vec<VariableSymbol>,
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
        VariableSymbol { name, global_index }
    }).collect())
}

fn find_pcode_symbols(data: &[u8], selected: usize) -> Result<FunctionSymbols, String> {
    let mut r = Cursor::new(data);
    if r.u16()? != 333 {
        return Err("not a version-333 compiled entry".into());
    }
    r.bytes(22)?;
    let descriptor_count = r.u16()? as usize;
    r.bytes(descriptor_count * 12)?;
    r.struct_buffer()?;
    let globals = named_variables(&mut r)?;
    let object_count = r.u16()? as usize;
    let extended_count = r.u16()? as usize;
    r.struct_buffer()?;
    r.struct_buffer()?;
    r.records20()?;
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
                for _ in 0..bodies {
                    let code_len = r.u16()? as usize;
                    let debug_count = r.u16()? as usize;
                    r.u16()?;
                    let code = r.bytes(code_len)?.to_vec();
                    r.bytes(debug_count * 4)?;

                    if function_index == selected {
                        let locals = named_variables(&mut r)?;
                        let stack_buffer = pool_buffer(&mut r)?;
                        return Ok(FunctionSymbols { code, stack_buffer, locals, globals });
                    }

                    function_index += 1;
                    r.records20()?;
                    r.struct_buffer()?;
                }

                r.bytes(word(header, 24)? * 6)?;
                r.bytes(word(header, 22)? * 4)?;
                r.records20()?;
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
        0x001e | 0x01a9 | 0x0150 | 0x011d => {
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
        0x0195 => "isnull(value)".into(),
        0x00a8 => "comparison: =".into(),
        0x0025 => "logical OR".into(),
        0x00f8 => "increment variable (++)".into(),
        0x0006 => "SQL COMMIT".into(),
        0x0010 => "SQL SELECT".into(),
        0x000f => "SQL DML (insert/update/delete), variant unresolved".into(),
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
        0x001e | 0x01a9 | 0x0150 | 0x011d => {
            stack.push(flow_local(sym, p));
            None
        }
        0x002f => {
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
        0x0025 => {
            let b=flow_pop(stack);
            let a=flow_pop(stack);
            stack.push(format!("({a} OR {b})"));
            None
        }
        0x003f | 0x008e | 0x0133 | 0x013b | 0x0139 | 0x0129 => None, // cast = no-op per PbdViewer
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
        0x0027 => {
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
            Some(format!(
                "SQL SELECT <SQL text hidden> INTO :{} USING {}; INPUTS: {}",
                destinations.join(", :"),sqlca,params.join(", ")
            ))
        }
        0x000f => {
            let sqlca=flow_pop(stack);
            let params=flow_pop_n(stack,flow_operand_word(p,4));
            Some(format!(
                "SQL DML <SQL text hidden> USING {}; PARAMETERS: {}",
                sqlca,params.join(", ")
            ))
        }
        0x0006 => {
            let sqlca=flow_pop(stack);
            Some(format!("COMMIT USING {sqlca}"))
        }
        0x01bc => {
            let function_index=flow_operand_word(p,0);
            // For object ref 0x8001, this index is an entry in the
            // referenced-functions table. The actual name is not yet resolved.
            stack.push(format!("<global-function-ref#{function_index}>"));
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
    println!("Warning: per-block stack is reset; SQL text and function references are unresolved.");
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

fn disassemble_one(
    parser: &PblParser,
    name: &str,
    body_index: usize,
    limit: usize,
    show_strings: bool,
    show_flow: bool,
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
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // Optional mode: <name.fun> [body_index=0] [max_instructions=80]
    if !args.is_empty() {
        if args.len() > 5 || args.iter().skip(3).any(|x| x != "--flow" && x != "--strings") {
            return Err("usage: probe_pcode [name.fun [body_index [max_instructions [--flow] [--strings]]]]".into());
        }
        let body_index = args.get(1).map(|s| s.parse::<usize>())
            .transpose()?.unwrap_or(0);
        let limit = args.get(2).map(|s| s.parse::<usize>())
            .transpose()?.unwrap_or(80).min(1000);
        let show_strings = args.iter().skip(3).any(|s| s == "--strings");
        let show_flow = args.iter().skip(3).any(|s| s == "--flow");
        let parser = PblParser::new(EXE)?;
        return disassemble_one(&parser, &args[0], body_index, limit, show_strings, show_flow);
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
