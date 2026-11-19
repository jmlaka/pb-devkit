
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::HashMap;

const COMMON: [u16; 10] = [
    0x4079, 0x4083, 0x4086, 0x4087, 0x4088,
    0x4089, 0x409B, 0x40AC, 0x40BF, 0x40D3,
];

// Categories:
// 0 = no UTF-16 separator
// 1 = empty string
// 2 = printable ASCII without spaces
// 3 = other printable Unicode text
// 4 = invalid UTF-16 or control characters
// 5 = no terminator within 256 UTF-16 units
fn classify(data: &[u8], pos: usize) -> usize {
    if data.get(pos..pos.saturating_add(2))
        != Some(&[0, 0][..])
    {
        return 0;
    }

    let mut units = Vec::new();

    for pair in data[pos + 2..].chunks_exact(2).take(256) {
        let ch = u16::from_le_bytes([pair[0], pair[1]]);

        if ch == 0 {
            if units.is_empty() {
                return 1;
            }

            return match String::from_utf16(&units) {
                Ok(text) => {
                    if text.chars().all(|c| c.is_ascii_graphic()) {
                        2
                    } else if text.chars().all(|c| !c.is_control()) {
                        3
                    } else {
                        4
                    }
                }
                Err(_) => 4,
            };
        }

        units.push(ch);
    }

    5
}

#[derive(Default)]
struct Stats {
    total: usize,
    categories: [usize; 6],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    for ext in [".fun", ".str"] {
        let mut stats: HashMap<(u16, u16, u16), Stats> =
            HashMap::new();

        let mut objects = 0usize;
        let mut total = [0usize; 6];

        for entry in parser.entries().iter()
            .filter(|e| e.name.to_lowercase().ends_with(ext))
        {
            let data = parser.parse_chunk_chain(
                entry.data_offset,
                entry.size,
            )?;

            let meta = parse_object_metadata(
                &entry.name, &data
            )?;

            objects += 1;

            let table_end =
                0x20 + meta.descriptors.len() * 12;

            for d in &meta.descriptors {
                if d.tag_a == d.tag_b
                    && (COMMON.contains(&d.tag_a)
                        || d.tag_a == 0x4077)
                {
                    continue;
                }

                let pos = table_end + d.field_6 as usize;
                let category = classify(&data, pos);

                let key = (d.tag_a, d.tag_b, d.flags);
                let s = stats.entry(key).or_default();

                s.total += 1;
                s.categories[category] += 1;
                total[category] += 1;
            }
        }

        let mut ranked: Vec<_> = stats.into_iter().collect();

        ranked.sort_by(|a, b| {
            b.1.total.cmp(&a.1.total)
                .then_with(|| a.0.cmp(&b.0))
        });

        println!("\n=== {} ===", ext);
        println!("Objects: {}", objects);

        println!("\nOVERALL:");
        let labels = [
            "No separator",
            "Empty string",
            "ASCII text",
            "Other Unicode text",
            "Invalid/control",
            "Unterminated/long",
        ];

        for (label, count) in labels.iter().zip(total.iter()) {
            println!("  {:<22} {}", label, count);
        }

        println!("\nTOP DESCRIPTOR CLASSES:");
        println!(
            "TAG_A TAG_B FLAGS  TOTAL  ASCII UNICODE EMPTY INVALID LONG NOSEP"
        );

        for ((a, b, flags), s) in ranked.iter().take(20) {
            println!(
                "{:04X}  {:04X}  {:04X}  {:>5} {:>6} {:>7} {:>5} {:>7} {:>4} {:>5}",
                a,
                b,
                flags,
                s.total,
                s.categories[2],
                s.categories[3],
                s.categories[1],
                s.categories[4],
                s.categories[5],
                s.categories[0],
            );
        }
    }

    Ok(())
}
