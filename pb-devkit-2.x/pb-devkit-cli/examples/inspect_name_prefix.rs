
use pb_devkit_core::pbl::PblParser;

fn hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser =
        PblParser::new("/home/juraj/Dev/promis/proamb.exe")?;

    for ext in [".fun", ".str", ".win", ".udo", ".apl"] {
        let entry = parser.entries()
            .iter()
            .find(|e| e.name.to_lowercase().ends_with(ext))
            .ok_or("Object not found")?;

        let data = parser.parse_chunk_chain(
            entry.data_offset,
            entry.size,
        )?;

        let count = u16::from_le_bytes(
            data[0x18..0x1A].try_into()?
        ) as usize;

        let field_1a = u16::from_le_bytes(
            data[0x1A..0x1C].try_into()?
        ) as usize;

        let table_end = 0x20 + count * 12;
        let name_start = table_end + 18;

        let base_name = entry.name
            .rsplit_once('.')
            .map(|(name, _)| name)
            .unwrap_or(&entry.name);

        let name_bytes: Vec<u8> = base_name
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect();

        let name_end = name_start + name_bytes.len();

        println!("\n=== {} ===", entry.name);
        println!("Object type: 0x{:04X}",
            u16::from_le_bytes(data[4..6].try_into()?));
        println!("Descriptors: {}", count);
        println!("Name block length: {}", field_1a);
        println!("Expected length: {}", 18 + name_bytes.len());

        println!("Prefix (18 bytes): {}",
            hex(&data[table_end..name_start]));

        println!("Name matches: {}",
            data.get(name_start..name_end)
                == Some(name_bytes.as_slice()));

        println!("Name block ends at: 0x{:04X}", name_end);
        println!("Next 16 bytes: {}",
            hex(&data[name_end..name_end + 16]));
    }

    Ok(())
}
