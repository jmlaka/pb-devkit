
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Stats {
    objects: usize,
    occurrences: usize,
    min_pos: usize,
    max_pos: usize,
    flags: HashMap<u16, usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser =
        PblParser::new("/home/juraj/Dev/promis/proamb.exe")?;

    for ext in [".fun", ".str", ".win", ".udo"] {
        let mut total = 0usize;
        let mut stats: HashMap<(u16, u16), Stats> =
            HashMap::new();

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

            total += 1;
            let mut seen = HashSet::new();

            for (pos, d) in meta.descriptors.iter().enumerate() {
                let key = (d.tag_a, d.tag_b);
                let s = stats.entry(key).or_insert_with(|| Stats {
                    min_pos: usize::MAX,
                    ..Default::default()
                });

                s.occurrences += 1;
                s.min_pos = s.min_pos.min(pos);
                s.max_pos = s.max_pos.max(pos);
                *s.flags.entry(d.flags).or_insert(0) += 1;

                if seen.insert(key) {
                    s.objects += 1;
                }
            }
        }

        let mut universal: Vec<_> = stats.iter()
            .filter(|(_, s)| s.objects == total)
            .collect();

        universal.sort_by_key(|(_, s)| s.min_pos);

        println!("\n=== {} ===", ext);
        println!("Objects: {}", total);
        println!("Universal TAG pairs: {}", universal.len());
        println!(
            "TAG_A TAG_B  PRESENT  USES  POSITION  TOP_FLAG"
        );

        for ((a, b), s) in universal {
            let top_flag = s.flags.iter()
                .max_by_key(|(_, count)| *count)
                .map(|(flag, _)| *flag)
                .unwrap_or(0);

            let position = if s.min_pos == s.max_pos {
                format!("{}", s.min_pos)
            } else {
                format!("{}-{}", s.min_pos, s.max_pos)
            };

            println!(
                "{:04X}  {:04X}  {:>7} {:>5} {:>9}  {:04X}",
                a, b, s.objects,
                s.occurrences, position, top_flag
            );
        }
    }

    Ok(())
}
