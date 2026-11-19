
use pb_devkit_core::pbl::PblParser;
use std::collections::{HashMap, HashSet};

fn read_token(data: &[u8], start: usize) -> String {
    let mut result = String::new();

    if let Some(tail) = data.get(start..) {
        for pair in tail.chunks_exact(2).take(128) {
            let ch = u16::from_le_bytes([pair[0], pair[1]]);

            if !(0x21..=0x7E).contains(&ch) {
                break;
            }

            result.push(char::from_u32(ch as u32).unwrap());
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    let known: HashSet<String> = parser.entries()
        .iter()
        .filter(|e| e.name.to_lowercase().ends_with(".udo"))
        .filter_map(|e| e.name.rsplit_once('.'))
        .map(|(base, _)| base.to_lowercase())
        .collect();

    let mut internal = 0usize;
    let mut native = 0usize;
    let mut invalid = 0usize;
    let mut unresolved: HashMap<String, usize> = HashMap::new();

    for entry in parser.entries()
        .iter()
        .filter(|e| e.name.to_lowercase().ends_with(".udo"))
    {
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

        let name_len = u16::from_le_bytes(
            data[0x1A..0x1C].try_into()?
        ) as usize;

        let name_end = 0x20 + count * 12 + name_len;

        if data.get(name_end..name_end + 2)
            != Some(&[0, 0][..])
        {
            invalid += 1;
            continue;
        }

        let parent = read_token(&data, name_end + 2)
            .to_lowercase();

        if known.contains(&parent) {
            internal += 1;
        } else if parent == "userobject" {
            native += 1;
        } else {
            *unresolved.entry(parent).or_insert(0) += 1;
        }
    }

    println!("Internal inheritance: {}", internal);
    println!("Native userobject: {}", native);
    println!("Invalid: {}", invalid);
    println!("\nUnresolved parent types:");

    let mut types: Vec<_> = unresolved.into_iter().collect();

    types.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    let mut total = 0;

    for (parent, count) in types {
        println!("{:<35} {}", parent, count);
        total += count;
    }

    println!("\nTotal unresolved: {}", total);

    Ok(())
}
