
use pb_devkit_core::object::parse_object_metadata;
use pb_devkit_core::pbl::PblParser;
use std::collections::BTreeMap;

const MARKER: &[u8] = &[
    0x10, 0x00, 0x32, 0x00,
    0x08, 0x00, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x20, 0x00,
];

#[derive(Default)]
struct Stats {
    total: usize,
    found: usize,
    at_boundary: usize,
    after_boundary: usize,
    before_boundary: usize,
    multiple: usize,
}

fn token_end(data: &[u8], start: usize) -> Option<usize> {
    if data.get(start..start.checked_add(2)?)?
        != [0u8, 0u8]
    {
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

fn hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PblParser::new(
        "/home/juraj/Dev/promis/proamb.exe"
    )?;

    let mut stats = Stats::default();

    let mut prefixes =
        BTreeMap::<[u8; 4], usize>::new();

    let mut deltas =
        BTreeMap::<i64, usize>::new();

    let mut no_marker_samples = Vec::new();
    let mut before_samples = Vec::new();
    let mut after_samples = Vec::new();

    for entry in parser.entries().iter()
        .filter(|e| e.name.to_lowercase().ends_with(".fun"))
    {
        let data = parser.parse_chunk_chain(
            entry.data_offset,
            entry.size,
        )?;

        let meta = parse_object_metadata(
            &entry.name, &data
        )?;

        let table_end =
            0x20 + meta.descriptors.len() * 12;

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

        stats.total += 1;

        if let Some(bytes) =
            data.get(last_text_end..last_text_end + 4)
        {
            let prefix = [
                bytes[0], bytes[1], bytes[2], bytes[3]
            ];

            *prefixes.entry(prefix).or_insert(0) += 1;
        }

        // Search for the exact 16-byte marker throughout
        // the area following the descriptor table.
        let positions: Vec<usize> = data.windows(MARKER.len())
            .enumerate()
            .filter_map(|(pos, bytes)| {
                if pos >= table_end && bytes == MARKER {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect();

        if positions.is_empty() {
            if no_marker_samples.len() < 8 {
                let end = (last_text_end + 24).min(data.len());

                no_marker_samples.push(format!(
                    "{} | tail={} | prefix={}",
                    entry.name,
                    data.len() - last_text_end,
                    hex(&data[last_text_end..end])
                ));
            }

            continue;
        }

        stats.found += 1;

        if positions.len() > 1 {
            stats.multiple += 1;
        }

        // Choose the occurrence nearest to the end
        // of the last successfully decoded token.
        let nearest = positions.iter()
            .copied()
            .min_by_key(|&pos| pos.abs_diff(last_text_end))
            .unwrap();

        let delta =
            nearest as i64 - last_text_end as i64;

        *deltas.entry(delta).or_insert(0) += 1;

        if delta == 0 {
            stats.at_boundary += 1;
        } else if delta > 0 {
            stats.after_boundary += 1;

            if after_samples.len() < 8 {
                after_samples.push(format!(
                    "{} | delta=+{}",
                    entry.name, delta
                ));
            }
        } else {
            stats.before_boundary += 1;

            if before_samples.len() < 8 {
                before_samples.push(format!(
                    "{} | delta={}",
                    entry.name, delta
                ));
            }
        }
    }

    println!("=== FUNCTION SECTION MARKER ===");
    println!("Total functions: {}", stats.total);
    println!("Marker found: {}", stats.found);
    println!("Marker absent: {}", stats.total - stats.found);
    println!("Multiple occurrences: {}", stats.multiple);

    println!("\nMARKER POSITION RELATIVE TO LAST TEXT:");
    println!("Exactly at boundary: {}", stats.at_boundary);
    println!("After boundary: {}", stats.after_boundary);
    println!("Before boundary: {}", stats.before_boundary);

    let mut ranked_deltas: Vec<_> = deltas.into_iter().collect();

    ranked_deltas.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    println!("\nMOST COMMON POSITION DIFFERENCES:");
    for (delta, count) in ranked_deltas.iter().take(15) {
        println!("  delta={:+7} | count={}", delta, count);
    }

    let mut ranked_prefixes: Vec<_> =
        prefixes.into_iter().collect();

    ranked_prefixes.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    println!("\nPREFIXES AT LAST TEXT END:");
    for (prefix, count) in ranked_prefixes.iter().take(15) {
        println!(
            "  {:02X} {:02X} {:02X} {:02X} | {}",
            prefix[0], prefix[1],
            prefix[2], prefix[3], count
        );
    }

    println!("\nSAMPLES WITHOUT MARKER:");
    for sample in no_marker_samples {
        println!("  {}", sample);
    }

    println!("\nSAMPLES WITH MARKER AFTER LAST TEXT:");
    for sample in after_samples {
        println!("  {}", sample);
    }

    println!("\nSAMPLES WITH MARKER BEFORE LAST TEXT:");
    for sample in before_samples {
        println!("  {}", sample);
    }

    Ok(())
}
