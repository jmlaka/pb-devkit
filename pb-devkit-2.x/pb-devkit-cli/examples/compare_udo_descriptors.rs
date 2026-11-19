
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::HashMap;

struct Obj {
    parent: String,
    descriptors: Vec<(u16, u16, u16)>,
}

fn print_descriptors(label: &str, name: &str, obj: &Obj) {
    let marker = obj.descriptors.iter().position(|&(a, b, _)| {
        a == 0x4077 && b == 0x4077
    });

    println!("\n{}: {}", label, name);
    println!("Parent: {}", obj.parent);

    let Some(marker) = marker else {
        println!("Marker 4077 not found!");
        return;
    };

    println!("4077 position: {}", marker);
    println!("POS  TAG_A  TAG_B  FLAGS");

    for (i, &(a, b, flags)) in obj.descriptors
        .iter()
        .take(marker + 1)
        .enumerate()
    {
        println!(
            "{:>3}  {:04X}   {:04X}   {:04X}",
            i, a, b, flags
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    let mut objects: HashMap<String, Obj> = HashMap::new();

    // Load all UDO metadata.
    for entry in parser.entries()
        .iter()
        .filter(|e| e.name.to_lowercase().ends_with(".udo"))
    {
        let data = parser.parse_chunk_chain(
            entry.data_offset,
            entry.size,
        )?;

        let meta = parse_object_metadata(&entry.name, &data)?;

        let name = entry.name
            .rsplit_once('.')
            .unwrap().0.to_lowercase();

        let descriptors = meta.descriptors.iter()
            .map(|d| (d.tag_a, d.tag_b, d.flags))
            .collect();

        objects.insert(name, Obj {
            parent: meta.parent.to_lowercase(),
            descriptors,
        });
    }

    println!("Loaded {} UDO objects", objects.len());

    // Compare a native-derived object with its direct child.
    for family in [
        "nonvisualobject",
        "commandbutton",
        "editmask",
    ] {
        println!("\n================================");
        println!("FAMILY: {}", family);
        println!("================================");

        let pair = objects.iter()
            .filter(|(_, obj)| {
                obj.parent.eq_ignore_ascii_case(family)
            })
            .find_map(|(direct_name, direct)| {
                objects.iter()
                    .find(|(_, child)| {
                        child.parent.eq_ignore_ascii_case(direct_name)
                    })
                    .map(|(child_name, child)| {
                        (direct_name, direct, child_name, child)
                    })
            });

        if let Some((direct_name, direct, child_name, child)) = pair {
            print_descriptors(
                "DIRECT FROM NATIVE",
                direct_name,
                direct,
            );

            print_descriptors(
                "INHERITED FROM UDO",
                child_name,
                child,
            );
        } else {
            println!("No direct parent-child pair found.");
        }
    }

    Ok(())
}
