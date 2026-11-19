use pb_devkit_core::pbl::PblParser;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new("/home/juraj/Dev/promis");
    let exe = root.join("proamb.exe");
    let output_dir = root.join("analysis");

    fs::create_dir_all(&output_dir)?;

    let parser = PblParser::new(exe.to_str().unwrap())?;

    // Select one non-sensitive PNG resource.
    let entry = parser
        .entries()
        .iter()
        .find(|e| e.name.to_lowercase().ends_with(".png"))
        .ok_or("No PNG object found")?;

    println!("Object: {}", entry.name);
    println!("Expected size: {}", entry.size);
    println!("DAT offset: {}", entry.data_offset);

    let data = parser.parse_chunk_chain(
        entry.data_offset,
        entry.size,
    )?;

    println!("Extracted size: {}", data.len());

    fs::write(output_dir.join("chunk_test_rust.bin"), &data)?;
    fs::write(
        output_dir.join("chunk_test_name.txt"),
        &entry.name,
    )?;

    println!("Rust extraction OK");

    Ok(())
}