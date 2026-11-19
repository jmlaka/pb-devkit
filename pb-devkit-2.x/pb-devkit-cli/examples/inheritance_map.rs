
use pb_devkit_core::pbl::PblParser;
use std::collections::HashSet;

fn read_token(data: &[u8], start: usize) -> String {
    let mut result = String::new();

    let Some(tail) = data.get(start..) else {
        return result;
    };

    for pair in tail.chunks_exact(2).take(100) {
        let ch = u16::from_le_bytes([pair[0], pair[1]]);

        if !(0x21..=0x7E).contains(&ch) {
            break;
        }

        result.push(char::from_u32(ch as u32).unwrap());
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser =
        PblParser::new("/home/juraj/Dev/promis/proamb.exe")?;

    for ext in [".win", ".udo"] {
        let known: HashSet<String> = parser.entries()
            .iter()
            .filter(|e| e.name.to_lowercase().ends_with(ext))
            .filter_map(|e| e.name.rsplit_once('.'))
            .map(|(base, _)| base.to_lowercase())
            .collect();

        let mut total = 0;
        let mut inherited = 0;
        let mut native = 0;
        let mut unknown = 0;
        let mut invalid = 0;
        let mut examples = Vec::new();

        for entry in parser.entries()
            .iter()
            .filter(|e| e.name.to_lowercase().ends_with(ext))
        {
            total += 1;

            let data = parser.parse_chunk_chain(
                entry.data_offset,
                entry.size,
            )?;

            if data.len() < 0x20 {
                invalid += 1;
                continue;
            }

            let count = u16::from_le_bytes(
                data[0x18..0x1A].try_into()?
            ) as usize;

            let name_block_len = u16::from_le_bytes(
                data[0x1A..0x1C].try_into()?
            ) as usize;

            let name_end = 0x20 + count * 12 + name_block_len;

            if name_end + 2 > data.len() {
                invalid += 1;
                continue;
            }

            let token = read_token(&data, name_end + 2);
            let parent = token.to_lowercase();

            if known.contains(&parent) {
                inherited += 1;

                if examples.len() < 15
                    && !["login", "hesl", "connect", "auth"]
                        .iter()
                        .any(|s| entry.name.contains(s)
                            || parent.contains(s))
                {
                    examples.push((
                        entry.name.clone(),
                        parent,
                    ));
                }
            } else if parent == "window"
                || parent == "userobject"
            {
                native += 1;
            } else {
                unknown += 1;
            }
        }

        println!("\n=== {} ===", ext);
        println!("Total objects: {}", total);
        println!("Known parent candidates: {}", inherited);
        println!("Native base type: {}", native);
        println!("Unknown parent/type: {}", unknown);
        println!("Invalid layout: {}", invalid);

        println!("\nSample inheritance candidates:");
        for (child, parent) in examples {
            println!("  {} -> {}", child, parent);
        }
    }

    Ok(())
}
