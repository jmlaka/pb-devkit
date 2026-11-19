
use pb_devkit_core::pbl::PblParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser =
        PblParser::new("/home/juraj/Dev/promis/proamb.exe")?;

    let names = [
        "w_f_zapis_vykon_neplat_cash_automat.fun",
        "w_f_zapis_vykon_neplateny_cash.fun",
        "z1_prenos_karta.str",
    ];

    for name in names {
        let entry = parser.entries()
            .iter()
            .find(|e| e.name == name)
            .ok_or("Object not found")?;

        let data = parser.parse_chunk_chain(
            entry.data_offset,
            entry.size,
        )?;

        let count = u16::from_le_bytes(
            data[0x18..0x1A].try_into()?
        ) as usize;

        let table_end = 0x20 + count * 12;

        println!("\n=== {} ===", name);
        println!("Records: {}", count);
        println!("Table end: 0x{:04X}", table_end);

        // Last record and 3 records after the candidate boundary.
        for offset in (table_end - 12..table_end + 36).step_by(12) {
            if offset + 12 > data.len() {
                break;
            }

            let bytes = &data[offset..offset + 12];

            let hex = bytes.iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");

            let marker = if offset < table_end {
                "LAST"
            } else {
                "AFTER"
            };

            println!("{:04X}  {:5}  {}", offset, marker, hex);
        }
    }

    Ok(())
}
