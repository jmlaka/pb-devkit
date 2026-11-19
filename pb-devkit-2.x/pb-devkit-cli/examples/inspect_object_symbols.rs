
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;

const COMMON: [u16; 10] = [
    0x4079, 0x4083, 0x4086, 0x4087, 0x4088,
    0x4089, 0x409B, 0x40AC, 0x40BF, 0x40D3,
];

fn read_token(data: &[u8], pos: usize) -> (String, String) {
    if data.get(pos..pos.saturating_add(2))
        != Some(&[0, 0][..])
    {
        return ("NO_PREFIX".into(), String::new());
    }

    let mut units = Vec::new();

    for pair in data[pos + 2..].chunks_exact(2).take(256) {
        let value = u16::from_le_bytes([pair[0], pair[1]]);

        if value == 0 {
            return match String::from_utf16(&units) {
                Ok(text) => {
                    let kind = if text.is_empty() {
                        "EMPTY"
                    } else if text.chars().all(|c| c.is_ascii_graphic()) {
                        "ASCII"
                    } else if text.chars().all(|c| !c.is_control()) {
                        "UNICODE"
                    } else {
                        "CONTROL"
                    };

                    let preview: String =
                        text.chars().take(65).collect();

                    (kind.into(), format!("{:?}", preview))
                }
                Err(_) => ("INVALID_UTF16".into(), String::new()),
            };
        }

        units.push(value);
    }

    ("NO_TERMINATOR".into(), String::new())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    let targets = [
        "z1_prenos_karta.str",
        "eh_abuzy.str",
        "w_f_zapis_vykon_neplat_cash_automat.fun",
    ];

    for name in targets {
        let entry = parser.entries()
            .iter()
            .find(|e| e.name.eq_ignore_ascii_case(name))
            .ok_or(format!("Object not found: {}", name))?;

        let data = parser.parse_chunk_chain(
            entry.data_offset,
            entry.size,
        )?;

        let meta = parse_object_metadata(&entry.name, &data)?;

        let table_end = 0x20 + meta.descriptors.len() * 12;

        println!("\n======================================");
        println!("OBJECT: {}", entry.name);
        println!("PARENT: {}", meta.parent);
        println!("SIZE: {} bytes", data.len());
        println!("DESCRIPTORS: {}", meta.descriptors.len());
        println!("======================================");

        println!(
            "{:>3} {:>4} {:>4} {:>4} {:>6} {:>8} {:<15} {}",
            "POS", "TAGA", "TAGB", "FLAGS",
            "FIELD6", "FIELD8", "KIND", "VALUE"
        );

        for (index, d) in meta.descriptors.iter().enumerate() {
            // Skip the common built-in type names.
            if d.tag_a == d.tag_b
                && COMMON.contains(&d.tag_a)
            {
                continue;
            }

            // Skip the 4077 delimiter.
            if d.tag_a == 0x4077 && d.tag_b == 0x4077 {
                continue;
            }

            let offset = table_end + d.field_6 as usize;
            let (kind, token) = read_token(&data, offset);

            println!(
                "{:>3} {:04X} {:04X} {:04X} {:04X} {:08X} {:<15} {}",
                index,
                d.tag_a,
                d.tag_b,
                d.flags,
                d.field_6,
                d.field_8,
                kind,
                token
            );
        }
    }

    Ok(())
}
