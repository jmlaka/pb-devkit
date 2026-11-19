
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::{BTreeMap, BTreeSet, HashSet};

struct Structure {
    own_id: u32,
    target: String,
    reference_id: u32,
    descriptor_values: HashSet<u32>,
}

fn read_token(data: &[u8], offset: usize) -> Option<String> {
    if data.get(offset..offset + 2)? != [0, 0] {
        return None;
    }

    let mut units = Vec::new();

    for pair in data.get(offset + 2..)?
        .chunks_exact(2)
        .take(128)
    {
        let ch = u16::from_le_bytes([pair[0], pair[1]]);

        if ch == 0 {
            return String::from_utf16(&units).ok();
        }

        if !(0x21..=0x7E).contains(&ch) {
            return None;
        }

        units.push(ch);
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    let mut objects = BTreeMap::<String, Structure>::new();

    for entry in parser.entries().iter()
        .filter(|e| e.name.to_lowercase().ends_with(".str"))
    {
        let data = parser.parse_chunk_chain(
            entry.data_offset,
            entry.size,
        )?;

        let meta = parse_object_metadata(&entry.name, &data)?;

        let own = meta.descriptors.iter()
            .find(|d| d.tag_a == 0x4079 && d.tag_b == 0x4079)
            .ok_or("Missing 4079")?;

        let reference = meta.descriptors.iter()
            .find(|d| d.tag_a == 0x40D3 && d.tag_b == 0x40D3)
            .ok_or("Missing 40D3")?;

        let table_end = 0x20 + meta.descriptors.len() * 12;

        let target = read_token(
            &data,
            table_end + reference.field_6 as usize,
        )
        .ok_or("Invalid reference text")?
        .to_lowercase();

        let name = entry.name
            .rsplit_once('.')
            .unwrap().0.to_lowercase();

        objects.insert(name, Structure {
            own_id: own.field_8,
            target,
            reference_id: reference.field_8,
            descriptor_values: meta.descriptors.iter()
                .map(|d| d.field_8)
                .collect(),
        });
    }

    let mut target_to_ids =
        BTreeMap::<String, BTreeSet<u32>>::new();

    let mut id_to_targets =
        BTreeMap::<u32, BTreeSet<String>>::new();

    let mut references = 0usize;
    let mut found_in_target = 0usize;
    let mut missing_targets = 0usize;

    for obj in objects.values() {
        if obj.reference_id == 0 {
            continue;
        }

        references += 1;

        target_to_ids
            .entry(obj.target.clone())
            .or_default()
            .insert(obj.reference_id);

        id_to_targets
            .entry(obj.reference_id)
            .or_default()
            .insert(obj.target.clone());

        match objects.get(&obj.target) {
            Some(target) => {
                if target.descriptor_values
                    .contains(&obj.reference_id)
                {
                    found_in_target += 1;
                }
            }
            None => missing_targets += 1,
        }
    }

    let inconsistent_targets = target_to_ids.values()
        .filter(|ids| ids.len() > 1)
        .count();

    let ambiguous_ids = id_to_targets.values()
        .filter(|targets| targets.len() > 1)
        .count();

    println!("=== REFERENCE ID CONSISTENCY ===");
    println!("Structures: {}", objects.len());
    println!("References: {}", references);
    println!("Distinct targets: {}", target_to_ids.len());
    println!("Distinct reference IDs: {}", id_to_targets.len());
    println!("Targets with multiple IDs: {}", inconsistent_targets);
    println!("IDs referring to multiple targets: {}", ambiguous_ids);
    println!("Reference IDs present in target descriptors: {}",
        found_in_target);
    println!("Missing target structures: {}", missing_targets);

    println!("\nFIRST 15 TARGET MAPPINGS:");
    println!("TARGET                         REF_ID  OWN_ID");

    for (target_name, ids) in target_to_ids.iter().take(15) {
        let own_id = objects.get(target_name)
            .map(|obj| obj.own_id)
            .unwrap_or(0);

        for ref_id in ids {
            println!(
                "{:<30} {:04X}    {:04X}",
                target_name,
                ref_id >> 16,
                own_id >> 16,
            );
        }
    }

    Ok(())
}
