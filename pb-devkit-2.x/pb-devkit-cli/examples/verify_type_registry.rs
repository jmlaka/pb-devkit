use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::{BTreeMap, BTreeSet, HashSet};

fn read_name(data: &[u8], pos: usize) -> Option<String> {
    if data.get(pos..pos.checked_add(2)?)? != [0, 0] {
        return None;
    }

    let tail = data.get(pos + 2..)?;
    let mut units = Vec::new();

    for pair in tail.chunks_exact(2).take(128) {
        let ch = u16::from_le_bytes([pair[0], pair[1]]);

        if ch == 0 {
            if units.is_empty() {
                return None;
            }

            return String::from_utf16(&units).ok().map(|s| s.to_lowercase());
        }

        if !(0x21..=0x7E).contains(&ch) {
            return None;
        }

        units.push(ch);
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new("/home/juraj/Dev/promis/proamb.exe")?;

    let known: HashSet<String> = parser
        .entries()
        .iter()
        .filter(|e| e.name.to_lowercase().ends_with(".str"))
        .map(|e| e.name.rsplit_once('.').unwrap().0.to_lowercase())
        .collect();

    // (TAG_A, TAG_B, FLAGS) -> (total, matching catalog)
    let mut classes = BTreeMap::<(u16, u16, u16), (usize, usize)>::new();

    let mut name_to_ids = BTreeMap::<String, BTreeSet<u32>>::new();

    let mut id_to_names = BTreeMap::<u32, BTreeSet<String>>::new();

    let mut checked = 0usize;
    let mut decoded = 0usize;
    let mut catalog_matches = 0usize;
    let mut invalid = 0usize;
    let mut low_word_nonzero = 0usize;
    let mut unknown_examples = BTreeSet::new();

    for entry in parser
        .entries()
        .iter()
        .filter(|e| e.name.to_lowercase().ends_with(".str"))
    {
        let data = parser.parse_chunk_chain(entry.data_offset, entry.size)?;

        let meta = parse_object_metadata(&entry.name, &data)?;

        let table_end = 0x20 + meta.descriptors.len() * 12;

        for d in &meta.descriptors {
            if d.field_8 == 0 {
                continue;
            }

            if (d.tag_a == 0x4079 && d.tag_b == 0x4079) || (d.tag_a == 0x4077 && d.tag_b == 0x4077)
            {
                continue;
            }

            checked += 1;

            if d.field_8 & 0xFFFF != 0 {
                low_word_nonzero += 1;
            }

            let pos = table_end + d.field_6 as usize;

            let Some(name) = read_name(&data, pos) else {
                invalid += 1;
                continue;
            };

            decoded += 1;

            let in_catalog = known.contains(&name);

            let entry = classes.entry((d.tag_a, d.tag_b, d.flags)).or_insert((0, 0));

            entry.0 += 1;

            if in_catalog {
                entry.1 += 1;
                catalog_matches += 1;
            } else if unknown_examples.len() < 15 {
                unknown_examples.insert(name.clone());
            }

            name_to_ids
                .entry(name.clone())
                .or_default()
                .insert(d.field_8);

            id_to_names.entry(d.field_8).or_default().insert(name);
        }
    }

    println!("=== TYPE REFERENCE REGISTRY ===");
    println!("Known .str names: {}", known.len());
    println!("Nonzero references checked: {}", checked);
    println!("Decoded names: {}", decoded);
    println!("Matches in .str catalog: {}", catalog_matches);
    println!("Invalid names: {}", invalid);
    println!("Nonzero low 16 bits: {}", low_word_nonzero);

    println!("\nDESCRIPTOR CLASSES:");
    println!("TAG_A TAG_B FLAGS TOTAL CATALOG");

    for ((a, b, flags), (count, matches)) in &classes {
        println!(
            "{:04X}  {:04X}  {:04X}  {:>5} {:>7}",
            a, b, flags, count, matches
        );
    }

    let inconsistent_names = name_to_ids.iter().filter(|(_, ids)| ids.len() > 1).count();

    let ambiguous_ids = id_to_names
        .iter()
        .filter(|(_, names)| names.len() > 1)
        .count();

    println!("\nCONSISTENCY:");
    println!("Distinct names: {}", name_to_ids.len());
    println!("Distinct IDs: {}", id_to_names.len());
    println!("Names with multiple IDs: {}", inconsistent_names);
    println!("IDs with multiple names: {}", ambiguous_ids);

    println!("\nEXAMPLE MAPPINGS:");
    for (name, ids) in name_to_ids.iter().take(20) {
        let ids: Vec<String> = ids.iter().map(|id| format!("{:08X}", id)).collect();

        println!(
            "{:<32} {}  catalog={}",
            name,
            ids.join(", "),
            known.contains(name),
        );
    }

    if !unknown_examples.is_empty() {
        println!("\nNAMES NOT IN .str CATALOG:");
        for name in unknown_examples {
            println!("  {}", name);
        }
    }

    Ok(())
}
