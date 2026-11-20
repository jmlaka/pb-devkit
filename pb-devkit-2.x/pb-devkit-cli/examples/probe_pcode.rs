//! Read-only PowerBuilder 12.x P-code boundary verification.
//! Based on PbEntry.cs and PCodeParser110.cs from Hucxy/PbdViewer.
//! Tests instruction boundaries only; does NOT emit operands, bytecode or strings.
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut ranked: Vec<_> = frequencies.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    println!("\nTOP 25 OPCODES (ID, COUNT):");
    for (opcode, count) in ranked.into_iter().take(25) {
        println!("  0x{:04X}  {}", opcode, count);
    }

    println!("\nFIRST 12 ERRORS:");
    for error in errors {
        println!("  {}", error);
    }

    Ok(())
}
