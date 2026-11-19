
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::HashMap;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    for ext in [".fun", ".str", ".win", ".udo", ".apl"] {
        let mut objects = 0usize;
        let mut descriptors = 0usize;
        let mut matching_tags = 0usize;

        let mut flags: HashMap<u16, usize> = HashMap::new();
        let mut tags: HashMap<(u16, u16), usize> = HashMap::new();

        for entry in parser.entries().iter()
            .filter(|e| e.name.to_lowercase().ends_with(ext))
        {
            let data = parser.parse_chunk_chain(
                entry.data_offset,
                entry.size,
            )?;

            let meta = parse_object_metadata(&entry.name, &data)
                .map_err(|e| io::Error::new(
                    io::ErrorKind::InvalidData, e
                ))?;

            objects += 1;

            for d in &meta.descriptors {
                descriptors += 1;

                if d.tag_a == d.tag_b {
                    matching_tags += 1;
                }

                *flags.entry(d.flags).or_insert(0) += 1;

                *tags.entry((d.tag_a, d.tag_b))
                    .or_insert(0) += 1;
            }
        }

        let mut flag_list: Vec<_> = flags.into_iter().collect();
        flag_list.sort_by(|a, b| b.1.cmp(&a.1));

        let mut tag_list: Vec<_> = tags.into_iter().collect();
        tag_list.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n=== {} ===", ext);
        println!("Objects: {}", objects);
        println!("Descriptors: {}", descriptors);
        println!("TAG_A == TAG_B: {}", matching_tags);

        println!("\nMost common FLAGS:");
        for (flag, count) in flag_list.iter().take(10) {
            println!("  0x{:04X}  {}", flag, count);
        }

        println!("\nMost common TAG pairs:");
        for ((a, b), count) in tag_list.iter().take(15) {
            println!("  0x{:04X} 0x{:04X}  {}", a, b, count);
        }
    }

    Ok(())
}
