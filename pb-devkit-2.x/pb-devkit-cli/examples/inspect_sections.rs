
use pb_devkit_core::pbl::PblParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser =
        PblParser::new("/home/juraj/Dev/promis/proamb.exe")?;

    for extension in [".fun", ".str"] {
        let mut count = 0;

        for entry in parser.entries() {
            let name = entry.name.to_lowercase();

            if !name.ends_with(extension) {
                continue;
            }

            if ["connect", "login", "hesl", "password", "auth"]
                .iter()
                .any(|s| name.contains(s))
            {
                continue;
            }

            let data = parser.parse_chunk_chain(
                entry.data_offset,
                entry.size,
            )?;

            if data.len() < 0x98 {
                continue;
            }

            let u16_at = |pos: usize| -> u16 {
                u16::from_le_bytes(
                    data[pos..pos + 2].try_into().unwrap()
                )
            };

            let u32_at = |pos: usize| -> u32 {
                u32::from_le_bytes(
                    data[pos..pos + 4].try_into().unwrap()
                )
            };

            println!("\n=== {} ===", entry.name);
            println!("Size: {}", data.len());
            println!("Object tag: 0x{:04X}", u16_at(4));
            println!("Field 0x18: {}", u16_at(0x18));
            println!("Field 0x1A: {}", u16_at(0x1A));

            println!("POS    TAG_A  TAG_B  FLAGS  VALUE  RESERVED");

            for i in 0..8 {
                let pos = 0x38 + i * 12;

                println!(
                    "{:04X}   {:04X}   {:04X}   {:04X}   {:04X}   {:08X}",
                    pos,
                    u16_at(pos),
                    u16_at(pos + 2),
                    u16_at(pos + 4),
                    u16_at(pos + 6),
                    u32_at(pos + 8),
                );
            }

            count += 1;

            if count >= 2 {
                break;
            }
        }
    }

    Ok(())
}
