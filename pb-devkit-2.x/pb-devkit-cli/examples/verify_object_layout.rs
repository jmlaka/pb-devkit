
use pb_devkit_core::pbl::PblParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser =
        PblParser::new("/home/juraj/Dev/promis/proamb.exe")?;

    println!("Typ    Test  Header  NameLen  NamePos");

    for ext in [".fun", ".str", ".win", ".udo", ".dwo", ".apl"] {
        let mut total = 0;
        let mut headers = 0;
        let mut lengths = 0;
        let mut positions = 0;

        for entry in parser.entries()
            .iter()
            .filter(|e| e.name.to_lowercase().ends_with(ext))
            .take(30)
        {
            total += 1;

            let data = parser.parse_chunk_chain(
                entry.data_offset,
                entry.size,
            )?;

            if data.len() < 0x20
                || data[0..4] != [0x4D, 0x01, 0x03, 0x00]
            {
                continue;
            }

            headers += 1;

            let count = u16::from_le_bytes(
                data[0x18..0x1A].try_into()?
            ) as usize;

            let name_length = u16::from_le_bytes(
                data[0x1A..0x1C].try_into()?
            ) as usize;

            let name_utf16: Vec<u8> = entry.name
                .encode_utf16()
                .flat_map(|c| c.to_le_bytes())
                .collect();

            if name_length == name_utf16.len() + 10 {
                lengths += 1;
            }

            let table_end = 0x20 + count * 12;
            let name_start = table_end + 18;
            let name_end = name_start + name_utf16.len();

            if let Some(bytes) = data.get(name_start..name_end) {
                if bytes == name_utf16.as_slice() {
                    positions += 1;
                }
            }
        }

        println!(
            "{:<6} {:>5} {:>7} {:>8} {:>8}",
            ext, total, headers, lengths, positions
        );
    }

    Ok(())
}
