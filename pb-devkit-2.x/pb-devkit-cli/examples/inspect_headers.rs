
use pb_devkit_core::pbl::PblParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/home/juraj/Dev/promis/proamb.exe";
    let parser = PblParser::new(path)?;

    // Compare ordinary compiled functions and structures.
    // Exclude authentication-related object names.
    for extension in [".fun", ".str"] {
        println!("\n=== OBJECT TYPE: {extension} ===");

        let mut count = 0;

        for entry in parser.entries() {
            let name = entry.name.to_lowercase();

            if !name.ends_with(extension)
                || ["connect", "login", "hesl", "password", "auth"]
                    .iter()
                    .any(|term| name.contains(term))
            {
                continue;
            }

            let data = parser.parse_chunk_chain(
                entry.data_offset,
                entry.size,
            )?;

            println!("\n{} ({} bytes)", entry.name, data.len());

            for (i, chunk) in data.iter()
                .take(96)
                .collect::<Vec<_>>()
                .chunks(16)
                .enumerate()
            {
                let hex = chunk.iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                println!("{:04X}: {}", i * 16, hex);
            }

            count += 1;
            if count == 3 {
                break;
            }
        }
    }

    Ok(())
}
