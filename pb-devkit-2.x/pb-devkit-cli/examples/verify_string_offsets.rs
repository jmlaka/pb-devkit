
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;

const TAGS: [u16; 10] = [
    0x4079, 0x4083, 0x4086, 0x4087, 0x4088,
    0x4089, 0x409B, 0x40AC, 0x40BF, 0x40D3,
];

// Decode a UTF-16LE token between two descriptor offsets.
// Each offset is expected to point to a 2-byte separator.
fn decode_token(
    data: &[u8],
    start: usize,
    end: usize,
) -> Option<String> {
    if start + 2 > end || end + 2 > data.len() {
        return None;
    }

    if data.get(start..start + 2)? != [0, 0]
        || data.get(end..end + 2)? != [0, 0]
    {
        return None;
    }

    let raw = data.get(start + 2..end)?;

    if raw.len() % 2 != 0 {
        return None;
    }

    let units: Vec<u16> = raw.chunks_exact(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();

    let text = String::from_utf16(&units).ok()?;

    if !text.chars().all(|c| c.is_ascii_graphic()) {
        return None;
    }

    Some(text)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    for ext in [".fun", ".str", ".win", ".udo"] {
        let mut objects = 0usize;
        let mut checked = 0usize;
        let mut valid = 0usize;
        let mut failures = Vec::new();

        println!("\n====================");
        println!("TYPE: {}", ext);

        for entry in parser.entries().iter()
            .filter(|e| e.name.to_lowercase().ends_with(ext))
        {
            let data = parser.parse_chunk_chain(
                entry.data_offset,
                entry.size,
            )?;

            let meta = parse_object_metadata(
                &entry.name, &data
            )?;

            let marker = meta.descriptors.iter()
                .position(|d|
                    d.tag_a == 0x4077 && d.tag_b == 0x4077
                );

            let Some(marker) = marker else {
                continue;
            };

            let prefix = &meta.descriptors[..marker];

            let found: Vec<_> = TAGS.iter()
                .filter_map(|&tag| {
                    prefix.iter().find(|d|
                        d.tag_a == tag && d.tag_b == tag
                    )
                })
                .collect();

            if found.len() != 10 {
                continue;
            }

            objects += 1;

            let table_end = 0x20 + meta.descriptors.len() * 12;

            // Skip 4079 -> 4083, because the first item
            // may contain the object name and additional data.
            for i in 1..9 {
                let start =
                    table_end + found[i].field_6 as usize;

                let end =
                    table_end + found[i + 1].field_6 as usize;

                checked += 1;

                match decode_token(&data, start, end) {
                    Some(token) => {
                        valid += 1;

                        // Print a sample only for the first object.
                        if objects == 1 {
                            println!(
                                "  {:04X} -> {:04X}: {:?}",
                                TAGS[i], TAGS[i + 1], token
                            );
                        }
                    }
                    None => {
                        if failures.len() < 5 {
                            failures.push(format!(
                                "{}: {:04X}->{:04X}",
                                entry.name,
                                TAGS[i], TAGS[i + 1]
                            ));
                        }
                    }
                }
            }
        }

        println!("Objects tested: {}", objects);
        println!("String intervals checked: {}", checked);
        println!("Valid UTF-16LE intervals: {}", valid);
        println!("Invalid intervals: {}", checked - valid);

        for failure in failures {
            println!("  INVALID: {}", failure);
        }
    }

    Ok(())
}
