// SPDX-License-Identifier: Apache-2.0 OR MIT
// This module parses the ICC namedColor2Type (ncl2) tag into a TOML-friendly structure.
// The tag contains a header (with a device coordinate dimension, name prefix/suffix) followed
// by N entries, each with:
// - a 32-byte root name (ASCII, NUL-terminated)
// - a 3x u16 PCS (Lab16 or XYZ16 depending on flags)
// - M x u16 device coordinates where M == header.dim.
// The final name key is "{prefix}{root}{suffix}".
use serde::{self, Serialize};
use std::collections::BTreeMap;
use zerocopy::{BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, U16, U32};

use crate::{round_to_precision, tag::tagdata::NamedColor2Data};

fn serialize_flags_as_hex<S>(flags: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // skip_serializing_if handles zero upstream; format 32-bit flags as "HHHH HHHH"
    // to match other hex formatting in this crate.
    let hex = format!("{flags:08X}");
    let formatted = format!("{} {}", &hex[..4], &hex[4..]);
    serializer.serialize_str(&formatted)
}

// Helpers to keep decoding logic readable

/// Decode ICC Lab16 PCS to Lab with standard ranges:
/// - L* in [0..100]
/// - a*, b* in approximately [-128..127]
///   ICC encodes a* and b* as unsigned 16-bit with 0..65535 covering -128..127.
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

/// Decode ICC XYZ16 PCS from u1.15 fixed to floating XYZ where 1.0 ~= 32768/32768.
/// The range is approximately [0, 1.99997].
#[inline]
fn decode_pcs_xyz16(pcs: [U16<BigEndian>; 3]) -> [f64; 3] {
    // ICC u1.15 fixed: [0, 1.99997]
    [
        crate::u1_fixed15_number(pcs[0].get()),
        crate::u1_fixed15_number(pcs[1].get()),
        crate::u1_fixed15_number(pcs[2].get()),
    ]
}

/// Decode device values as normalized [0, 1] floats from u16.
/// Many producers store device values as 0..65535 where 65535 maps to 1.0.
/// We round to 5 decimals for compact TOML output.
#[inline]
fn decode_device_u16(dev: &[U16<BigEndian>]) -> Vec<f64> {
    // Normalize device coords to [0,1]
    dev.iter()
        .map(|d| round_to_precision(d.get() as f64 / 65535.0, 5))
        .collect()
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct NamedColor2Type {
    /// Original flags from the tag, emitted as a spaced hex string (e.g., "0001 0000").
    /// Bit 16 is commonly used to indicate that the PCS is Lab (vendor convention).
    #[serde(
        serialize_with = "serialize_flags_as_hex",
        skip_serializing_if = "crate::is_zero"
    )]
    pub flags: u32,
    /// Map of full color names -> (PCS triplet, device coordinates).
    /// - PCS is Lab or XYZ depending on flags
    /// - Device coordinates length equals `dim` in the header.
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
        // Parse header, then iterate entries using zerocopy
        let (header, mut data) = HeaderLayout::try_ref_from_prefix(&ncl2_data.0)
            .expect("NamedColor2 header parse error");
        let n = header.count.get() as usize; // number of entries
        let m = header.dim.get() as usize; // device coordinate dimension
        let flags = header.flags.get();

        // Bit 16 is used by many implementations to mark Lab PCS.
        let pcs_is_lab = flags & 0x0001_0000 != 0;

        // Extract NUL-terminated ASCII prefix/suffix
        let prefix = String::from_utf8_lossy(&header.prefix)
            .trim_end_matches('\0')
            .to_string();
        let suffix = String::from_utf8_lossy(&header.suffix)
            .trim_end_matches('\0')
            .to_string();

        let mut colors = BTreeMap::new();
        for _ in 0..n {
            // Each entry has M device U16s; pass M to zerocopy so it can split correctly.
            let (entry, rest) = EntryLayout::try_ref_from_prefix_with_elems(data, m)
                .expect("NamedColor2 entry parse error");
            data = rest; // advance slice for the next entry

            // Build final key: prefix + root + suffix
            let root = String::from_utf8_lossy(&entry.root)
                .trim_end_matches('\0')
                .to_string();
            let key = format!("{prefix}{root}{suffix}");

            // Decode PCS depending on flag
            let pcs = if pcs_is_lab {
                decode_pcs_lab16(entry.pcs)
            } else {
                decode_pcs_xyz16(entry.pcs)
            };

            // Decode device coordinates if present
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
