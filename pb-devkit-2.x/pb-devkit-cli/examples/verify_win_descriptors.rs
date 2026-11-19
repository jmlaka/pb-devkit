
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::HashSet;

const TAGS: [u16; 10] = [
    0x4079, 0x4083, 0x4086, 0x4087, 0x4088,
    0x4089, 0x409B, 0x40AC, 0x40BF, 0x40D3,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    let known: HashSet<String> = parser.entries()
        .iter()
        .filter(|e| e.name.to_lowercase().ends_with(".win"))
        .filter_map(|e| e.name.rsplit_once('.'))
        .map(|(stem, _)| stem.to_lowercase())
        .collect();

    let mut distribution = [[0usize; 11]; 2];
    let mut invalid = 0usize;
    let mut exceptions = Vec::new();

    for entry in parser.entries().iter()
        .filter(|e| e.name.to_lowercase().ends_with(".win"))
    {
        let data = parser.parse_chunk_chain(
            entry.data_offset,
            entry.size,
        )?;

        let meta = parse_object_metadata(
            &entry.name, &data
        )?;

        let parent = meta.parent.to_lowercase();

        let group = if known.contains(&parent) {
            1usize
        } else {
            0usize
        };

        let markers: Vec<usize> = meta.descriptors
            .iter()
            .enumerate()
            .filter(|(_, d)| {
                d.tag_a == 0x4077 && d.tag_b == 0x4077
            })
            .map(|(i, _)| i)
            .collect();

        if markers.len() != 1 {
            invalid += 1;
            continue;
        }

        let prefix = &meta.descriptors[..markers[0]];

        let found = TAGS.iter().filter(|&&tag| {
            prefix.iter().any(|d| {
                d.tag_a == tag && d.tag_b == tag
            })
        }).count();

        distribution[group][found] += 1;

        let expected = if group == 0 { 10 } else { 0 };

        if found != expected && exceptions.len() < 15 {
            exceptions.push(format!(
                "{} -> {} : {}/10",
                entry.name, parent, found
            ));
        }
    }

    for (group, label) in [
        (0usize, "DIRECT FROM NATIVE WINDOW"),
        (1usize, "INHERITED FROM WINDOW"),
    ] {
        println!("\n=== {} ===", label);

        let total: usize = distribution[group].iter().sum();
        println!("Objects: {}", total);

        for count in 0..=10 {
            let n = distribution[group][count];

            if n > 0 {
                println!("  {:>2}/10 : {}", count, n);
            }
        }
    }

    println!("\nInvalid markers: {}", invalid);

    if !exceptions.is_empty() {
        println!("\nExceptions:");
        for e in exceptions {
            println!("  {}", e);
        }
    }
    
    Ok(())
}
