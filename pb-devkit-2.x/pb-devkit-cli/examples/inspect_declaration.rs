
use pb_devkit_core::pbl::PblParser;

fn read_token(data: &[u8], start: usize) -> String {
    let mut result = String::new();

    let Some(tail) = data.get(start..) else {
        return result;
    };

    for pair in tail.chunks_exact(2).take(80) {
        let ch = u16::from_le_bytes([pair[0], pair[1]]);

        if ch == 0 || ch == 32 || ch == 9
            || ch == 10 || ch == 13
        {
            break;
        }

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

    let names = [
        "w_f_zapis_vykon_neplat_cash_automat.fun",
        "z1_prenos_karta.str",
        "w_wizard_vymazanie_pacienta.win",
        "u_wizard_zaklad_cast.udo",
        "ambul010.apl",
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

        let block_length = u16::from_le_bytes(
            data[0x1A..0x1C].try_into()?
        ) as usize;

        let table_end = 0x20 + count * 12;
        let name_end = table_end + block_length;

        let separator = data.get(name_end..name_end + 2);
        let token = read_token(&data, name_end + 2);

        println!("\n=== {} ===", entry.name);
        println!("Object tag: {:04X}",
            u16::from_le_bytes(data[4..6].try_into()?));
        println!("Descriptors: {}", count);
        println!("Name block end: 0x{:X}", name_end);
        println!("Null separator: {}",
            separator == Some(&[0, 0][..]));
        println!("Next token: {:?}", token);
    }

    Ok(())
}
