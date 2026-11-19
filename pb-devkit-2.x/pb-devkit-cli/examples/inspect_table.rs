
use pb_devkit_core::pbl::PblParser;

fn u16_at(data: &[u8], pos: usize) -> u16 {
    u16::from_le_bytes(data[pos..pos + 2].try_into().unwrap())
}

fn u32_at(data: &[u8], pos: usize) -> u32 {
    u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap())
}

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

        let count = u16_at(&data, 0x18) as usize;
        let field_1a = u16_at(&data, 0x1A);
        let table_start = 0x20usize;
        let table_end = table_start + count * 12;

        println!("\n=== {} ===", name);
        println!("Object size: {}", data.len());
        println!("Record count candidate: {}", count);
        println!("Field 0x1A: {}", field_1a);
        println!("Calculated table end: 0x{:04X}", table_end);

        if table_end > data.len() {
            println!("ERROR: Table exceeds object size");
            continue;
        }

        println!("INDEX  POS   TAG_A TAG_B FLAGS VALUE EXTRA");

        for i in 0..count {
            let pos = table_start + i * 12;

            println!(
                "{:>3}   {:04X}  {:04X}  {:04X}  {:04X}  {:04X}  {:08X}",
                i,
                pos,
                u16_at(&data, pos),
                u16_at(&data, pos + 2),
                u16_at(&data, pos + 4),
                u16_at(&data, pos + 6),
                u32_at(&data, pos + 8),
            );
        }
    }

    Ok(())
}
