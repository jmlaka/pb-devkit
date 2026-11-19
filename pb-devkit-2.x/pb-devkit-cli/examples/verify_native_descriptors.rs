
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::HashSet;

const COMMON_TAGS: [u16; 10] = [
    0x4079, 0x4083, 0x4086, 0x4087, 0x4088,
    0x4089, 0x409B, 0x40AC, 0x40BF, 0x40D3,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    let known: HashSet<String> = parser.entries()
        .iter()
        .filter(|e| e.name.to_lowercase().ends_with(".udo"))
        .filter_map(|e| e.name.rsplit_once('.'))
        .map(|(stem, _)| stem.to_lowercase())
        .collect();

    // Rows: native parent, user-defined parent.
    // Columns: number of common descriptors found (0..10).
    let mut distribution = [[0usize; 11]; 2];
    let mut invalid = 0usize;

    for entry in parser.entries().iter()
        .filter(|e| e.name.to_lowercase().ends_with(".udo"))
    {
        let data = parser.parse_chunk_chain(
            entry.data_offset,
            entry.size,
        )?;

        let meta = parse_object_metadata(
            &entry.name,
            &data,
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
            .map(|(pos, _)| pos)
            .collect();

        if markers.len() != 1 {
            invalid += 1;
            continue;
        }

        let prefix = &meta.descriptors[..markers[0]];

        let found = COMMON_TAGS.iter()
            .filter(|&&tag| {
                prefix.iter().any(|d| {
                    d.tag_a == tag && d.tag_b == tag
                })
            })
            .count();

        distribution[group][found] += 1;
    }

    for (group, label) in [
        (0usize, "DIRECT FROM NATIVE"),
        (1usize, "INHERITED FROM UDO"),
    ] {
        println!("\n=== {} ===", label);

        let total: usize = distribution[group].iter().sum();
        println!("Objects: {}", total);
        println!("Common tags found / object count:");

        for found in 0..=10 {
            let count = distribution[group][found];

            if count > 0 {
                println!("  {:>2}/10 : {}", found, count);
            }
        }
    }

    println!("\nInvalid marker count: {}", invalid);

    Ok(())
}
