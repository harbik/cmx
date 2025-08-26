// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, I32};

use crate::{
    tag::{self, tagdata::S15Fixed16ArrayData},
    S15Fixed16,
};

/// Represents the raw memory layout of an ICC `XYZData` tag.
///
/// It is marked with `#[repr(C)]` to ensure a predictable field order
/// and memory layout, which is required for safe, zero-cost casting.
#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
struct Layout {
    signature: [u8; 4],
    /// Reserved, must be 0.
    _1: [u8; 4],
    /// Array of s15Fixed16Numbers.
    values: [I32<BigEndian>],
}

/// Represents the raw memory layout of an ICC `XYZData` tag.
///
/// It is marked with `#[repr(C)]` to ensure a predictable field order
/// and memory layout, which is required for safe, zero-cost casting.
#[repr(C)]
#[derive(IntoBytes, KnownLayout, Unaligned, Immutable)]
struct WriteLayout<const N: usize> {
    signature: [u8; 4],
    /// Reserved, must be 0.
    _1: [u8; 4],
    /// Array of s15Fixed16Numbers.
    values: [I32<BigEndian>; N],
}

impl<const N: usize> WriteLayout<N> {
    /// Creates a new `WriteLayout` with the correct signature and reserved bytes.
    pub fn new(vec: [f64; N]) -> Self {
        let values = vec.map(|v| S15Fixed16::from(v).into());
        Self {
            signature: tag::DataSignature::S15Fixed16ArrayData.into(),
            _1: [0; 4],
            values, // Initialize with zeros
        }
    }
}

// Serializable structs for each tag type
#[derive(Serialize)]
pub struct S15Fixed16ArrayType {
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matrix: Option<[[f64; 3]; 3]>,
}

impl From<&S15Fixed16ArrayData> for S15Fixed16ArrayType {
    fn from(xyz: &S15Fixed16ArrayData) -> Self {
        let layout = Layout::ref_from_bytes(&xyz.0).unwrap();

        let values: Vec<f64> = layout
            .values
            .iter()
            .map(|v| crate::s15fixed16(v.get()))
            .collect();
        if values.len() == 9 {
            // If we have exactly 9 values, treat them as a matrix
            let matrix = [
                [values[0], values[1], values[2]],
                [values[3], values[4], values[5]],
                [values[6], values[7], values[8]],
            ];
            Self {
                values: None,
                matrix: Some(matrix),
            }
        } else {
            // Otherwise, treat them as a simple array of values
            Self {
                values: Some(values),
                matrix: None,
            }
        }
    }
}

impl S15Fixed16ArrayData {
    pub fn set<const N: usize>(&mut self, values: [f64; N]) {
        let layout = WriteLayout::new(values);
        self.0 = layout.as_bytes().to_vec();
    }
}
