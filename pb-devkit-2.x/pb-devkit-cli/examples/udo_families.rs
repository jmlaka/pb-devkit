use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::{BTreeMap, HashMap, HashSet};

struct Obj {
    parent: String,
    descriptors: Vec<(u16, u16, u16)>,
}

fn root_of(name: &str, objects: &HashMap<String, Obj>) -> Result<String, String> {
    let mut current = name.to_string();
    let mut visited = HashSet::new();

    loop {
        if !visited.insert(current.clone()) {
            return Err(format!("Inheritance cycle: {name}"));
        }

        let obj = objects
            .get(&current)
            .ok_or(format!("Missing object: {current}"))?;

        let parent = obj.parent.to_lowercase();

        if objects.contains_key(&parent) {
            current = parent;
        } else {
            return Ok(parent);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new("/home/juraj/Dev/promis/proamb.exe")?;

    let mut objects: HashMap<String, Obj> = HashMap::new();

    for entry in parser
        .entries()
        .iter()
        .filter(|e| e.name.to_lowercase().ends_with(".udo"))
    {
        let data = parser.parse_chunk_chain(entry.data_offset, entry.size)?;

        let meta = parse_object_metadata(&entry.name, &data)?;

        let stem = entry.name.rsplit_once('.').unwrap().0.to_lowercase();

        let descriptors = meta
            .descriptors
            .iter()
            .map(|d| (d.tag_a, d.tag_b, d.flags))
            .collect();

        objects.insert(
            stem,
            Obj {
                parent: meta.parent,
                descriptors,
            },
        );
    }

    let mut groups: BTreeMap<String, Vec<&Obj>> = BTreeMap::new();

    for (name, obj) in &objects {
        let root = root_of(name, &objects)?;
        groups.entry(root).or_default().push(obj);
    }

    let mut ranked: Vec<_> = groups.iter().collect();
    ranked.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    println!("Total UDO: {}", objects.len());
    println!("Native families: {}", ranked.len());

    for (root, members) in ranked {
        let mut marker_positions = Vec::new();

        for obj in members {
            for (i, &(a, b, _)) in obj.descriptors.iter().enumerate() {
                if a == 0x4077 && b == 0x4077 {
                    marker_positions.push(i);
                }
            }
        }

        let min_pos = marker_positions.iter().min().copied();
        let max_pos = marker_positions.iter().max().copied();

        println!(
            "\n{}: {} objects, 4077 position {:?}..{:?}",
            root,
            members.len(),
            min_pos,
            max_pos
        );

        if members.len() < 5 {
            continue;
        }

        if members.len() >= 5 {
            let mut distribution: BTreeMap<(usize, usize), usize> = BTreeMap::new();

            for obj in members {
                // Count inheritance levels within the UDO library.
                let mut parent = obj.parent.to_lowercase();
                let mut depth = 0usize;
                let mut visited = HashSet::new();

                while let Some(ancestor) = objects.get(&parent) {
                    if !visited.insert(parent.clone()) {
                        return Err("Inheritance cycle detected".into());
                    }

                    depth += 1;
                    parent = ancestor.parent.to_lowercase();
                }

                // Find the unique 4077/4077 descriptor.
                let positions: Vec<usize> = obj
                    .descriptors
                    .iter()
                    .enumerate()
                    .filter(|(_, &(a, b, _))| a == 0x4077 && b == 0x4077)
                    .map(|(i, _)| i)
                    .collect();

                if positions.len() != 1 {
                    return Err("Expected exactly one 4077 descriptor".into());
                }

                *distribution.entry((depth, positions[0])).or_insert(0) += 1;
            }

            println!("  DEPTH  MARKER_POS  COUNT");

            for ((depth, position), count) in distribution {
                println!("  {:>5}  {:>10}  {:>5}", depth, position, count);
            }
        }

        // Descriptor pairs at exactly the same position
        // in every object of this native family.
        let first = &members[0].descriptors;
        let mut stable = Vec::new();

        for (pos, &(a, b, _)) in first.iter().enumerate() {
            if members
                .iter()
                .all(|obj| obj.descriptors.get(pos).map(|&(x, y, _)| (x, y)) == Some((a, b)))
            {
                stable.push(format!("{}:{:04X}/{:04X}", pos, a, b));
            }
        }

        println!(
            "Stable positioned pairs (first 15): {}",
            stable
                .iter()
                .take(15)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    Ok(())
}
