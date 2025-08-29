// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, I32};

use crate::{tag::tagdata::XYZArrayData, S15Fixed16};

/// Represents the raw memory layout of an ICC `XYZData` tag.
///
/// It is marked with `#[repr(C)]` to ensure a predictable field order
/// and memory layout, which is required for safe, zero-cost casting.
#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
struct Layout {
    /// TagData signature, must be `b"XYZ "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    /// Array of three CIEXYZ values, stored as s15Fixed16Numbers.
    xyz: [[I32<BigEndian>; 3]],
}

#[derive(IntoBytes, KnownLayout, Unaligned, Immutable)]
#[repr(C)]
struct WriteLayout<const N: usize> {
    /// TagData signature, must be `b"XYZ "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    /// Array of three CIEXYZ values, stored as s15Fixed16Numbers.
    xyz: [[I32<BigEndian>; 3]; N],
}

impl WriteLayout<1> {
    /// Creates a new `WriteLayout` with the signature 'XYZ ' and initializes
    /// the XYZ values.
    pub fn new(xyz: [f64; 3]) -> Self {
        let xyz: [[I32<BigEndian>; 3]; 1] = [[
            S15Fixed16::from(xyz[0]).into(),
            S15Fixed16::from(xyz[1]).into(),
            S15Fixed16::from(xyz[2]).into(),
        ]];

        Self {
            signature: super::DataSignature::XYZArrayData.into(),
            _reserved: [0; 4],
            xyz,
        }
    }
}

// Serializable structs for each tag type
#[derive(Serialize)]
pub struct XYZArrayType {
    xyz: Vec<f64>,
}

impl XYZArrayData {
    /// Creates a new XYZArrayType from a single XYZ value.
    pub fn set(&mut self, xyz: [f64; 3]) {
        self.0 = WriteLayout::new(xyz).as_bytes().to_vec();
    }
}

/// Parses the raw data wrapped in XYZData into a XYZDataToml instance,
/// as used
impl From<&XYZArrayData> for XYZArrayType {
    fn from(xyz: &XYZArrayData) -> Self {
        let layout = Layout::ref_from_bytes(&xyz.0).unwrap();

        // Flatten directly during the conversion
        let xyz_vec: Vec<f64> = layout
            .xyz
            .iter()
            .flat_map(|xyz| {
                [
                    crate::s15fixed16(xyz[0].get()),
                    crate::s15fixed16(xyz[1].get()),
                    crate::s15fixed16(xyz[2].get()),
                ]
            })
            .collect();

        Self { xyz: xyz_vec }
    }
}
