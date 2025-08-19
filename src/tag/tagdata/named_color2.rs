// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::{self, Serialize};
use std::collections::BTreeMap;
use zerocopy::{BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, U16, U32};

use crate::{round_to_precision, tag::tagdata::NamedColor2Data};

fn serialize_flags_as_hex<S>(flags: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Skip serialization if flags is 0
    if *flags == 0 {
        return serializer.serialize_none();
    }

    // Format with spaces every 4 bytes (8 hex digits)
    let hex = format!("{flags:08X}");
    let formatted = if hex.len() > 4 {
        // Insert space after first 4 characters
        format!("{} {}", &hex[..4], &hex[4..])
    } else {
        hex
    };

    serializer.serialize_str(&formatted)
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct NamedColor2Type {
    #[serde(
        serialize_with = "serialize_flags_as_hex",
        skip_serializing_if = "crate::is_zero"
    )]
    pub flags: u32,
    #[serde(flatten)]
    pub colors: BTreeMap<String, ([f64; 3], Vec<f64>)>,
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
pub struct HeaderLayout {
    pub signature: U32<BigEndian>,
    pub reserved: [u8; 4],
    pub flags: U32<BigEndian>,
    pub count: U32<BigEndian>,
    pub dim: U32<BigEndian>, // Number of device coordinates e.g 3 for RGB, 4 for CMYK
    pub prefix: [u8; 32],
    pub suffix: [u8; 32],
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
pub struct EntryLayout {
    pub root: [u8; 32],
    pub pcs: [U16<BigEndian>; 3], // PCS values, 2 bytes each
    pub device: [U16<BigEndian>],
}

impl From<&NamedColor2Data> for NamedColor2Type {
    fn from(ncl2_data: &NamedColor2Data) -> Self {
        let (header, mut data) = HeaderLayout::try_ref_from_prefix(&ncl2_data.0)
            .expect("NamedColor2 header parse error");
        let n = header.count.get() as usize;
        let m = header.dim.get() as usize;
        let flags = header.flags.get();
        let lab = flags & 0x1_0000 != 0; // Check if the PCS Flag is set
        let prefix = String::from_utf8_lossy(&header.prefix)
            .trim_end_matches('\0')
            .to_string();
        let suffix = String::from_utf8_lossy(&header.suffix)
            .trim_end_matches('\0')
            .to_string();
        let mut colors = BTreeMap::new();
        for _ in 0..n {
            // Use correct element count (m) and advance the buffer for each entry
            let (entry, rest) = EntryLayout::try_ref_from_prefix_with_elems(data, m)
                .expect("NamedColor2 entry parse error");
            data = rest;

            let root = String::from_utf8_lossy(&entry.root)
                .trim_end_matches('\0')
                .to_string();
            let key = format!("{prefix}{root}{suffix}");
            let pcs = if lab {
                let mut lab = entry.pcs.map(|c| c.get() as f64);
                lab[0] = round_to_precision(lab[0] * 100.0 / 65535.0, 2); // Scale L
                lab[1] = round_to_precision(lab[1] / 65535.0 * 200.0 - 100.0, 2); // Scale a
                lab[2] = round_to_precision(lab[2] / 65535.0 * 200.0 - 100.0, 2); // Scale a
                lab
            } else {
                entry.pcs.map(|c| crate::u1_fixed15_number(c.get()))
            };
            let device_data: Vec<f64> = if m == 0 {
                Vec::new()
            } else {
                entry
                    .device
                    .iter()
                    .map(|d| round_to_precision(d.get() as f64 / 65536.0, 5))
                    .collect()
            };
            let value = (pcs, device_data);
            colors.insert(key, value);
        }
        Self { flags, colors }
    }
}
