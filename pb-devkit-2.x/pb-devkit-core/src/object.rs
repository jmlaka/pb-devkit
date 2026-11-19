
#[derive(Debug, Clone)]
pub struct PbDescriptor {
    pub tag_a: u16,
    pub tag_b: u16,
    pub flags: u16,
    pub field_6: u16,
    pub field_8: u32,
}

#[derive(Debug, Clone)]
pub struct PbObjectMetadata {
    pub name: String,
    pub object_tag: u16,
    pub parent: String,
    pub descriptors: Vec<PbDescriptor>,
}

fn u16_at(data: &[u8], pos: usize) -> u16 {
    u16::from_le_bytes([data[pos], data[pos + 1]])
}

fn u32_at(data: &[u8], pos: usize) -> u32 {
    u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap())
}

pub fn parse_object_metadata(
    name: &str,
    data: &[u8],
) -> Result<PbObjectMetadata, String> {
    if data.len() < 0x20 || data[..4] != [0x4D, 1, 3, 0] {
        return Err(format!("{name}: unsupported header"));
    }

    let count = u16_at(data, 0x18) as usize;
    let name_block_len = u16_at(data, 0x1A) as usize;

    let table_end = 0x20 + count * 12;
    let name_start = table_end + 18;

    let stem = name.rsplit_once('.')
        .map(|(s, _)| s)
        .unwrap_or(name);

    let encoded: Vec<u8> = stem.encode_utf16()
        .flat_map(|c| c.to_le_bytes())
        .collect();

    let name_end = name_start + encoded.len();

    if name_block_len != 18 + encoded.len()
        || name_end + 2 > data.len()
        || data.get(name_start..name_end) != Some(encoded.as_slice())
        || data.get(name_end..name_end + 2) != Some(&[0, 0][..])
    {
        return Err(format!("{name}: invalid name block"));
    }

    let mut descriptors = Vec::with_capacity(count);

    for i in 0..count {
        let pos = 0x20 + i * 12;

        descriptors.push(PbDescriptor {
            tag_a: u16_at(data, pos),
            tag_b: u16_at(data, pos + 2),
            flags: u16_at(data, pos + 4),
            field_6: u16_at(data, pos + 6),
            field_8: u32_at(data, pos + 8),
        });
    }

    let mut parent = String::new();

    for pair in data[name_end + 2..].chunks_exact(2).take(128) {
        let ch = u16::from_le_bytes([pair[0], pair[1]]);

        if !(0x21..=0x7E).contains(&ch) {
            break;
        }

        parent.push(char::from(ch as u8));
    }

    if parent.is_empty() {
        return Err(format!("{name}: missing parent/type"));
    }

    Ok(PbObjectMetadata {
        name: name.to_string(),
        object_tag: u16_at(data, 4),
        parent,
        descriptors,
    })
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeDescriptorProfile {
    Complete,
    Absent,
    Partial(usize),
}

impl PbObjectMetadata {
    /// Classify the ten common descriptors preceding 4077.
    ///
    /// Empirically validated for PB12.5 .win and .udo objects.
    pub fn native_descriptor_profile(
        &self,
    ) -> Result<NativeDescriptorProfile, String> {
        const TAGS: [u16; 10] = [
            0x4079, 0x4083, 0x4086, 0x4087, 0x4088,
            0x4089, 0x409B, 0x40AC, 0x40BF, 0x40D3,
        ];

        let marker_positions: Vec<usize> = self.descriptors
            .iter()
            .enumerate()
            .filter(|(_, d)| {
                d.tag_a == 0x4077 && d.tag_b == 0x4077
            })
            .map(|(index, _)| index)
            .collect();

        if marker_positions.len() != 1 {
            return Err(format!(
                "{}: expected exactly one 4077 marker",
                self.name
            ));
        }

        let before_marker =
            &self.descriptors[..marker_positions[0]];

        let count = TAGS.iter()
            .filter(|&&tag| {
                before_marker.iter().any(|d| {
                    d.tag_a == tag && d.tag_b == tag
                })
            })
            .count();

        Ok(match count {
            10 => NativeDescriptorProfile::Complete,
            0 => NativeDescriptorProfile::Absent,
            n => NativeDescriptorProfile::Partial(n),
        })
    }
}

