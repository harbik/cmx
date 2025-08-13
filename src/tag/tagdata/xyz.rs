// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{BigEndian, FromBytes, Immutable, KnownLayout, Unaligned, I32};

use crate::tag::tagdata::XYZArrayType;

/// The fixed-point denominator for an s15Fixed16Number (2^16).
const S15_FIXED_16_DIVISOR: f64 = 65536.0;

/// Represents the raw memory layout of an ICC `XYZType` tag.
///
/// It is marked with `#[repr(C)]` to ensure a predictable field order
/// and memory layout, which is required for safe, zero-cost casting.
#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
struct XYZTagLayout {
    /// TagData signature, must be `b"XYZ "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    /// Array of three CIEXYZ values, stored as s15Fixed16Numbers.
    xyz: [[I32<BigEndian>; 3]],
}

// Serializable structs for each tag type
#[derive(Serialize)]
pub struct XYZArrayTypeToml {
    xyz: Vec<f64>,
}

/// Parses the raw data wrapped in XYZType into a XYZTypeToml instance,
/// as used
impl From<&XYZArrayType> for XYZArrayTypeToml {
    fn from(xyz: &XYZArrayType) -> Self {
        let layout = XYZTagLayout::ref_from_bytes(&xyz.0).unwrap();

        // Flatten directly during the conversion
        let xyz_vec: Vec<f64> = layout
            .xyz
            .iter()
            .flat_map(|xyz| {
                [
                    xyz[0].get() as f64 / S15_FIXED_16_DIVISOR,
                    xyz[1].get() as f64 / S15_FIXED_16_DIVISOR,
                    xyz[2].get() as f64 / S15_FIXED_16_DIVISOR,
                ]
            })
            .collect();

        Self { xyz: xyz_vec }
    }
}
