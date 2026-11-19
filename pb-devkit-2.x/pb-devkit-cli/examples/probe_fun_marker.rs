
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::{BTreeMap, BTreeSet};

const MARKER: &[u8] = &[
    0x10, 0x00, 0x32, 0x00,
    0x08, 0x00, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x20, 0x00,
];

fn hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn token_end(data: &[u8], start: usize) -> Option<usize> {
    if data.get(start..start.checked_add(2)?)? != [0, 0] {
        return None;
    }

    let mut pos = start + 2;
    let mut units = Vec::new();

    for _ in 0..256 {
        let pair = data.get(pos..pos.checked_add(2)?)?;
        let ch = u16::from_le_bytes([pair[0], pair[1]]);
        pos += 2;

        if ch == 0 {
            if units.is_empty() {
                return None;
            }

            let text = String::from_utf16(&units).ok()?;

            if text.chars().all(|c| !c.is_control()) {
                return Some(pos);
            }

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

    let mut objects = 0usize;
    let mut marker_with_separator = 0usize;
    let mut descriptor_at_marker_minus_2 = 0usize;
    let mut descriptor_at_marker = 0usize;

    let mut descriptor_classes =
        BTreeMap::<(u16, u16, u16), usize>::new();

    let mut gap_prefixes =
        BTreeMap::<[u8; 4], usize>::new();

    let mut gap_lengths =
        BTreeMap::<usize, usize>::new();

    let mut gap_combinations =
        BTreeMap::<([u8; 4], usize), usize>::new();

    let mut common_prefix: Option<Vec<u8>> = None;
    let mut shown_gaps = BTreeSet::<usize>::new();

    println!("=== REPRESENTATIVE GAPS ===");

    for entry in parser.entries().iter()
        .filter(|e| e.name.to_lowercase().ends_with(".fun"))
    {
        let data = parser.parse_chunk_chain(
            entry.data_offset,
            entry.size,
        )?;

        let meta = parse_object_metadata(&entry.name, &data)?;
        let table_end = 0x20 + meta.descriptors.len() * 12;

        let mut last_text_end = table_end;

        for d in &meta.descriptors {
            let Some(pos) =
                table_end.checked_add(d.field_6 as usize)
            else {
                continue;
            };

            if let Some(end) = token_end(&data, pos) {
                last_text_end = last_text_end.max(end);
            }
        }

        let positions: Vec<usize> = data
            .windows(MARKER.len())
            .enumerate()
            .filter_map(|(pos, bytes)| {
                (pos >= table_end && bytes == MARKER)
                    .then_some(pos)
            })
            .collect();

        if positions.len() != 1 {
            println!(
                "UNEXPECTED: {} has {} markers",
                entry.name,
                positions.len()
            );
            continue;
        }

        objects += 1;

        let marker = positions[0];

        if data.get(marker.saturating_sub(2)..marker)
            == Some(&[0, 0][..])
        {
            marker_with_separator += 1;
        }

        // Is a descriptor pointing directly to this marker,
        // or to the two bytes immediately preceding it?
        for d in &meta.descriptors {
            let Some(pos) =
                table_end.checked_add(d.field_6 as usize)
            else {
                continue;
            };

            if pos == marker {
                descriptor_at_marker += 1;
            }

            if pos.checked_add(2) == Some(marker) {
                descriptor_at_marker_minus_2 += 1;

                *descriptor_classes
                    .entry((d.tag_a, d.tag_b, d.flags))
                    .or_insert(0) += 1;
            }
        }

        // Compare the bytes following the common marker
        // across all functions.
        let suffix = &data[marker..];
        let sample_len = suffix.len().min(96);

        if let Some(prefix) = common_prefix.as_mut() {
            let equal = prefix.iter()
                .zip(suffix.iter())
                .take_while(|(a, b)| a == b)
                .count();

            prefix.truncate(equal);
        } else {
            common_prefix = Some(
                suffix[..sample_len].to_vec()
            );
        }

        if marker < last_text_end {
            println!("UNEXPECTED OVERLAP: {}", entry.name);
            continue;
        }

        let gap = marker - last_text_end;

        *gap_lengths.entry(gap).or_insert(0) += 1;

        if gap >= 4 {
            let raw = &data[last_text_end..marker];

            let prefix = [
                raw[0], raw[1], raw[2], raw[3]
            ];

            *gap_prefixes.entry(prefix).or_insert(0) += 1;
            *gap_combinations.entry((prefix, gap))
                .or_insert(0) += 1;

            // Print one example for the main gap classes.
            if [12, 20, 28, 44, 60, 76, 92, 124]
                .contains(&gap)
                && shown_gaps.insert(gap)
            {
                println!("\n{} | GAP {}", entry.name, gap);

                if gap <= 64 {
                    println!("  {}", hex(raw));
                } else {
                    println!(
                        "  FIRST 32: {}",
                        hex(&raw[..32])
                    );
                    println!(
                        "  LAST  32: {}",
                        hex(&raw[gap - 32..])
                    );
                }
            }
        }
    }

    println!("\n=== MARKER ANALYSIS ===");
    println!("Objects: {}", objects);
    println!(
        "Marker preceded by 00 00: {}",
        marker_with_separator
    );
    println!(
        "Descriptors pointing to marker-2: {}",
        descriptor_at_marker_minus_2
    );
    println!(
        "Descriptors pointing to marker: {}",
        descriptor_at_marker
    );

    println!("\nDESCRIPTOR CLASSES AT MARKER-2:");
    for ((a, b, flags), count) in &descriptor_classes {
        println!(
            "  {:04X}/{:04X} flags={:04X}: {}",
            a, b, flags, count
        );
    }

    println!("\nCOMMON PREFIX FROM MARKER:");
    if let Some(prefix) = common_prefix {
        println!("Identical bytes: {}", prefix.len());
        println!("{}", hex(&prefix));
    }

    println!("\nGAP LENGTH DISTRIBUTION:");
    let mut gaps: Vec<_> = gap_lengths.into_iter().collect();
    gaps.sort_by(|a, b| b.1.cmp(&a.1));

    for (gap, count) in gaps.iter().take(20) {
        println!("  {:>5} bytes: {}", gap, count);
    }

    println!("\nGAP FIRST FOUR BYTES:");
    let mut prefixes: Vec<_> =
        gap_prefixes.into_iter().collect();

    prefixes.sort_by(|a, b| b.1.cmp(&a.1));

    for (p, count) in prefixes.iter().take(12) {
        println!(
            "  {:02X} {:02X} {:02X} {:02X}: {}",
            p[0], p[1], p[2], p[3], count
        );
    }

    println!("\nMOST COMMON (PREFIX, GAP) COMBINATIONS:");
    let mut combinations: Vec<_> =
        gap_combinations.into_iter().collect();

    combinations.sort_by(|a, b| b.1.cmp(&a.1));

    for ((p, gap), count) in combinations.iter().take(20) {
        println!(
            "  {:02X} {:02X} {:02X} {:02X}, gap={:>4}: {}",
            p[0], p[1], p[2], p[3], gap, count
        );
    }

    Ok(())
}
