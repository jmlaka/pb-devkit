
use pb_devkit_core::pbl::PblParser;

fn utf16le(s: &str) -> Vec<u8> {
    s.encode_utf16()
        .flat_map(|c| c.to_le_bytes())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser =
        PblParser::new("/home/juraj/Dev/promis/proamb.exe")?;

    println!(
        "{:<6} {:>6} {:>10} {:>10} {:>10}",
        "Type", "Test", "FullName", "BaseName", "NotFound"
    );

    for ext in [".fun", ".str", ".win", ".udo", ".apl"] {
        let mut total = 0;
        let mut full_matches = 0;
        let mut base_matches = 0;
        let mut not_found = 0;

        for entry in parser.entries()
            .iter()
            .filter(|e| e.name.to_lowercase().ends_with(ext))
            .take(30)
        {
            let data = parser.parse_chunk_chain(
                entry.data_offset,
                entry.size,
            )?;

            if data.len() < 0x20 {
                continue;
            }

            let count = u16::from_le_bytes(
                data[0x18..0x1A].try_into()?
            ) as usize;

            let start = 0x20 + count * 12 + 18;

            let full = utf16le(&entry.name);

            let base_name = entry.name
                .rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(&entry.name);

            let base = utf16le(base_name);

            let full_ok = data.get(start..start + full.len())
                == Some(full.as_slice());

            let base_ok = data.get(start..start + base.len())
                == Some(base.as_slice());

            total += 1;

            if full_ok {
                full_matches += 1;
            }

            if base_ok {
                base_matches += 1;
            }

            if !full_ok && !base_ok {
                not_found += 1;
            }
        }

        println!(
            "{:<6} {:>6} {:>10} {:>10} {:>10}",
            ext, total, full_matches, base_matches, not_found
        );
    }

    Ok(())
}
