
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::{HashMap, HashSet};

const NATIVE: &[&str] = &[
    "window", "userobject", "nonvisualobject",
    "datawindow", "editmask", "datastore", "timing",
    "checkbox", "statictext", "commandbutton",
    "dropdownlistbox", "dropdownpicturelistbox",
    "multilineedit", "oleobject", "picturebutton",
    "listbox", "olecustomcontrol", "radiobutton",
    "singlelineedit", "tab", "transaction",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    for ext in [".win", ".udo"] {
        let mut nodes: HashMap<String, String> = HashMap::new();
        let mut invalid = 0usize;

        for entry in parser.entries().iter()
            .filter(|e| e.name.to_lowercase().ends_with(ext))
        {
            let data = parser.parse_chunk_chain(
                entry.data_offset,
                entry.size,
            )?;

            match parse_object_metadata(&entry.name, &data) {
                Ok(meta) => {
                    let stem = meta.name
                        .rsplit_once('.')
                        .unwrap().0.to_lowercase();

                    nodes.insert(stem, meta.parent.to_lowercase());
                }
                Err(e) => {
                    invalid += 1;
                    eprintln!("{e}");
                }
            }
        }

        let mut internal = 0;
        let mut native = 0;
        let mut unknown = 0;
        let mut cycles = 0;
        let mut max_depth = 0usize;
        let mut deepest = String::new();

        for (name, parent) in &nodes {
            if nodes.contains_key(parent) {
                internal += 1;
            } else if NATIVE.contains(&parent.as_str()) {
                native += 1;
            } else {
                unknown += 1;
                println!("Unknown: {name} -> {parent}");
            }

            let mut visited = HashSet::new();
            let mut current = name.clone();
            let mut depth = 0usize;

            loop {
                if !visited.insert(current.clone()) {
                    cycles += 1;
                    println!("Cycle involving: {name}");
                    break;
                }

                let Some(next) = nodes.get(&current) else {
                    break;
                };

                if !nodes.contains_key(next) {
                    if depth > max_depth {
                        max_depth = depth;
                        deepest = name.clone();
                    }
                    break;
                }

                depth += 1;
                current = next.clone();
            }
        }

        println!("\n=== {ext} ===");
        println!("Parsed: {}", nodes.len());
        println!("Invalid: {invalid}");
        println!("Internal inheritance: {internal}");
        println!("Native roots: {native}");
        println!("Unknown: {unknown}");
        println!("Cycles: {cycles}");
        println!("Maximum inheritance depth: {max_depth}");
        println!("Deepest object: {deepest}");
    }

    Ok(())
}
