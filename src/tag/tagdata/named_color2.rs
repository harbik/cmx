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
    // skip_serializing_if handles zero; just format as "HHHH HHHH"
    let hex = format!("{flags:08X}");
    let formatted = format!("{} {}", &hex[..4], &hex[4..]);
    serializer.serialize_str(&formatted)
}

// Helpers to keep decoding logic readable
#[inline]
fn decode_pcs_lab16(pcs: [U16<BigEndian>; 3]) -> [f64; 3] {
    let mut l = pcs[0].get() as f64;
    let mut a = pcs[1].get() as f64;
    let mut b = pcs[2].get() as f64;

    // ICC Lab16: L* in [0..100], a*/b* in [-128..127]
    l = round_to_precision(l * 100.0 / 65535.0, 2);
    a = round_to_precision(a * 255.0 / 65535.0 - 128.0, 2);
    b = round_to_precision(b * 255.0 / 65535.0 - 128.0, 2);
    [l, a, b]
}

#[inline]
fn decode_pcs_xyz16(pcs: [U16<BigEndian>; 3]) -> [f64; 3] {
    // ICC u1.15 fixed: [0, 1.99997]
    [
        crate::u1_fixed15_number(pcs[0].get()),
        crate::u1_fixed15_number(pcs[1].get()),
        crate::u1_fixed15_number(pcs[2].get()),
    ]
}

#[inline]
fn decode_device_u16(dev: &[U16<BigEndian>]) -> Vec<f64> {
    // Normalize device coords to [0,1]
    dev.iter()
        .map(|d| round_to_precision(d.get() as f64 / 65535.0, 5))
        .collect()
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

        // Bit 16 indicates PCS is Lab in many vendors' ncl2; keep this behavior
        let pcs_is_lab = flags & 0x0001_0000 != 0;

        let prefix = String::from_utf8_lossy(&header.prefix)
            .trim_end_matches('\0')
            .to_string();
        let suffix = String::from_utf8_lossy(&header.suffix)
            .trim_end_matches('\0')
            .to_string();

        let mut colors = BTreeMap::new();
        for _ in 0..n {
            let (entry, rest) = EntryLayout::try_ref_from_prefix_with_elems(data, m)
                .expect("NamedColor2 entry parse error");
            data = rest;

            let root = String::from_utf8_lossy(&entry.root)
                .trim_end_matches('\0')
                .to_string();
            let key = format!("{prefix}{root}{suffix}");

            let pcs = if pcs_is_lab {
                decode_pcs_lab16(entry.pcs)
            } else {
                decode_pcs_xyz16(entry.pcs)
            };

            let device_data = if m == 0 {
                Vec::new()
            } else {
                decode_device_u16(&entry.device)
            };

            colors.insert(key, (pcs, device_data));
        }

        Self { flags, colors }
    }
}
