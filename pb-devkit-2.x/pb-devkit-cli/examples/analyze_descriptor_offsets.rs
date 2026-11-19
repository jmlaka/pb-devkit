
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::BTreeMap;

const TAGS: [u16; 10] = [
    0x4079, 0x4083, 0x4086, 0x4087, 0x4088,
    0x4089, 0x409B, 0x40AC, 0x40BF, 0x40D3,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    for ext in [".fun", ".str", ".win", ".udo"] {
        let mut total = 0usize;

        let mut deltas: Vec<BTreeMap<i32, usize>> =
            vec![BTreeMap::new(); 9];

        let mut nonzero_field8 = [0usize; 10];

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

            let marker = meta.descriptors.iter()
                .position(|d| {
                    d.tag_a == 0x4077 && d.tag_b == 0x4077
                });

            let Some(marker) = marker else {
                continue;
            };

            let prefix = &meta.descriptors[..marker];

            let found: Vec<_> = TAGS.iter()
                .filter_map(|&tag| {
                    prefix.iter().find(|d| {
                        d.tag_a == tag && d.tag_b == tag
                    })
                })
                .collect();

            if found.len() != 10 {
                continue;
            }

            total += 1;

            for i in 0..10 {
                if found[i].field_8 != 0 {
                    nonzero_field8[i] += 1;
                }
            }

            for i in 1..10 {
                let delta =
                    found[i].field_6 as i32
                    - found[i - 1].field_6 as i32;

                *deltas[i - 1].entry(delta).or_insert(0) += 1;
            }
        }

        println!("\n==============================");
        println!("TYPE: {}", ext);
        println!("Complete profiles: {}", total);
        println!("==============================");

        println!("\nDELTA DISTRIBUTION:");
        println!("FROM   TO     MOST_COMMON  COUNT  DISTINCT");

        for i in 0..9 {
            let distribution = &deltas[i];

            let most_common = distribution.iter()
                .max_by_key(|(_, count)| *count);

            if let Some((delta, count)) = most_common {
                println!(
                    "{:04X}   {:04X}   {:>7}   {:>5}   {:>4}",
                    TAGS[i],
                    TAGS[i + 1],
                    delta,
                    count,
                    distribution.len()
                );

                if distribution.len() > 1 {
                    print!("       Other values:");

                    for (value, frequency) in distribution {
                        if value != delta {
                            print!(" {}({})", value, frequency);
                        }
                    }

                    println!();
                }
            }
        }

        println!("\nNONZERO FIELD_8:");

        for (i, &count) in nonzero_field8.iter().enumerate() {
            if count > 0 {
                println!(
                    "TAG {:04X}: {} / {}",
                    TAGS[i], count, total
                );
            }
        }

        if nonzero_field8.iter().all(|&n| n == 0) {
            println!("All FIELD_8 values are zero.");
        }
    }

    Ok(())
}
