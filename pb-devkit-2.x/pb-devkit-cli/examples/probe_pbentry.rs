//! Read-only probe of PB 12.x compiled .fun entries.
//! Based on the layout followed by Hucxy/PbdViewer, PbEntry.cs.
//! Does not print source strings, credentials or P-code contents.

use pb_devkit_core::pbl::PblParser;

const EXE: &str = "/home/juraj/Dev/promis/proamb.exe";

// Start of ReadVariables(true) + a 16-byte default buffer + empty variable table
// + one object and one extended object, as observed in this application's .fun files.
const MARKER: [u8; 36] = [
    0x10, 0x00, 0x32, 0x00, 0x08, 0x00,
    0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x20, 0x00, 0x20, 0x00,
    0x20, 0x00, 0x20, 0x00, 0x20, 0x00,
    0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x01, 0x00,
];

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self.pos.checked_add(count)
            .ok_or_else(|| format!("offset overflow at 0x{:X}", self.pos))?;
        let result = self.data.get(self.pos..end)
            .ok_or_else(|| format!(
                "out of bounds at 0x{:X}, requested {} bytes (file size 0x{:X})",
                self.pos, count, self.data.len()
            ))?;
        self.pos = end;
        Ok(result)
    }

    fn u16(&mut self) -> Result<u16, String> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    fn u32(&mut self) -> Result<u32, String> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    // PbdViewer ReadStructBuffer(): u32 length + u32 extra_length + both byte arrays.
    fn struct_buffer(&mut self) -> Result<&'a [u8], String> {
        let length = self.u32()? as usize;
        let extra_length = self.u32()? as usize;
        let value = self.take(length)?;
        self.take(extra_length)?;
        Ok(value)
    }

    // PbdViewer ReadVariables() / ReadTypes() / ReadReferencedFunctions():
    // six-byte prefix + struct buffer + u16 byte count + N records of 20 bytes.
    fn records20(&mut self) -> Result<usize, String> {
        self.take(6)?;
        self.struct_buffer()?;
        let bytes = self.u16()? as usize;
        if bytes % 20 != 0 {
            return Err(format!(
                "20-byte record table length {} not divisible by 20 at 0x{:X}",
                bytes, self.pos - 2
            ));
        }
        self.take(bytes)?;
        Ok(bytes / 20)
    }
}

fn word(buf: &[u8], offset: usize) -> Result<u16, String> {
    let b = buf.get(offset..offset + 2)
        .ok_or_else(|| format!("short object header at offset {}", offset))?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}

#[derive(Default)]
struct Summary {
    marker_position: usize,
    marker_exact: bool,
    variable_bytes: usize,
    types: usize,
    objects: usize,
    extra_objects: usize,
    referenced_functions: usize,
    functions: usize,
    pcode_bytes: usize,
    first_pcode_position: Option<usize>,
    end: usize,
}

fn inspect(data: &[u8]) -> Result<Summary, String> {
    let mut r = Reader::new(data);
    let version = r.u16()?;
    if version != 333 {
        return Err(format!("unexpected compiled object version {}", version));
    }

    r.u16()?; // object flags
    r.u32()?; // object kind
    r.u32()?; // unknown
    r.u32()?; // modification timestamp
    r.u32()?; // compilation timestamp
    r.u32()?; // unknown
    let table_count = r.u16()? as usize;

    // Crucially starts at 0x1A, *not* 0x20.
    r.take(table_count * 12)?;
    let variable_bytes = r.struct_buffer()?.len();

    let marker_position = r.pos;
    let marker_exact = data.get(marker_position..marker_position + MARKER.len())
        == Some(MARKER.as_slice());

    let _global_variables = r.records20()?;
    let object_count = r.u16()? as usize;
    let extended_count = r.u16()? as usize;
    r.struct_buffer()?; // function name buffer
    r.struct_buffer()?; // parameter buffer
    let types = r.records20()?;
    let _enums = r.records20()?;

    // PB 12.x has 16-byte object indices and 32-byte extended object indices.
    let indices = r.take(object_count * 16)?;
    let extended = r.take(extended_count * 32)?;
    let mut extended_used = 0usize;
    let mut functions = 0usize;
    let mut pcode_bytes = 0usize;
    let mut first_pcode_position = None;
    let mut referenced_functions = 0usize;

    for index in indices.chunks_exact(16) {
        let object_kind = (index[0] >> 1) & 7;
        match object_kind {
            0 => {
                let start = extended_used * 32;
                let object_header = extended.get(start..start + 32)
                    .ok_or_else(|| "not enough extended-object headers".to_string())?;
                extended_used += 1;

                // PbdViewer ReadObject(): function references, then bodies.
                let body_count = r.u16()? as usize;
                r.take(body_count * 4)?;
                for _ in 0..body_count {
                    let pcode_length = r.u16()? as usize;
                    let debug_count = r.u16()? as usize;
                    r.u16()?; // additional function metadata
                    if first_pcode_position.is_none() {
                        first_pcode_position = Some(r.pos);
                    }
                    r.take(pcode_length)?;
                    r.take(debug_count * 4)?;
                    r.records20()?; // local variable declarations
                    r.struct_buffer()?; // stack/local data
                    functions += 1;
                    pcode_bytes += pcode_length;
                }

                r.take(word(object_header, 24)? as usize * 6)?;
                r.take(word(object_header, 22)? as usize * 4)?;
                referenced_functions += r.records20()?;
                r.records20()?; // properties/controls
                r.take(word(object_header, 28)? as usize * 8)?;
                r.take(word(object_header, 26)? as usize * 16)?;
                r.take(word(object_header, 4)? as usize * 48)?;
            }
            1 => {
                r.take(word(index, 4)? as usize * 8)?;
            }
            6 => {
                // No extra data read for this object kind in PbdViewer.
            }
            _ => {
                // Other kinds do not consume additional bytes in PbdViewer.
            }
        }
    }

    if r.pos != data.len() {
        return Err(format!(
            "not fully consumed: stopped at 0x{:X}, file ends at 0x{:X}",
            r.pos, data.len()
        ));
    }

    Ok(Summary {
        marker_position,
        marker_exact,
        variable_bytes,
        types,
        objects: object_count,
        extra_objects: extended_count,
        referenced_functions,
        functions,
        pcode_bytes,
        first_pcode_position,
        end: r.pos,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(EXE)?;
    let mut total = 0usize;
    let mut parsed = 0usize;
    let mut matching_markers = 0usize;
    let mut total_functions = 0usize;
    let mut total_pcode_bytes = 0usize;
    let mut failures = Vec::new();
    let mut examples = Vec::new();

    for entry in parser.entries().iter()
        .filter(|e| e.name.to_ascii_lowercase().ends_with(".fun"))
    {
        total += 1;
        let data = parser.parse_chunk_chain(entry.data_offset, entry.size)?;
        match inspect(&data) {
            Ok(s) => {
                parsed += 1;
                matching_markers += usize::from(s.marker_exact);
                total_functions += s.functions;
                total_pcode_bytes += s.pcode_bytes;
                if examples.len() < 5 {
                    examples.push(format!(
                        "{} | variable_buffer={} | marker=0x{:X} | types={} | objects={} (+{} extended) | refs={} | P-code bodies={} | P-code bytes={} | first P-code={:?} | end=0x{:X}",
                        entry.name, s.variable_bytes, s.marker_position,
                        s.types, s.objects, s.extra_objects, s.referenced_functions,
                        s.functions, s.pcode_bytes, s.first_pcode_position, s.end,
                    ));
                }
            }
            Err(err) => {
                if failures.len() < 12 {
                    failures.push(format!("{}: {}", entry.name, err));
                }
            }
        }
    }

    println!("=== PB 12.x ENTRY PARSER ===");
    println!(".fun objects: {}", total);
    println!("Completely parsed: {}", parsed);
    println!("Failed: {}", total - parsed);
    println!("Exact 36-byte marker among parsed: {}", matching_markers);
    println!("Extracted P-code bodies (count only): {}", total_functions);
    println!("Total P-code bytes (count only): {}", total_pcode_bytes);

    println!("\nEXAMPLES:");
    for x in examples { println!("  {}", x); }
    println!("\nFIRST ERRORS:");
    for x in failures { println!("  {}", x); }

    Ok(())
}
