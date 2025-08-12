// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{BigEndian, FromBytes, Immutable, KnownLayout, Unaligned, I32};

use crate::tag::tag_value::S15Fixed16ArrayType;

/// Represents the raw memory layout of an ICC `XYZType` tag.
///
/// It is marked with `#[repr(C)]` to ensure a predictable field order
/// and memory layout, which is required for safe, zero-cost casting.
#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
struct Layout {
    /// TagValue signature, must be `b"XYZ "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _1: [u8; 4],
    /// Array of three CIEXYZ values, stored as s15Fixed16Numbers.
    values: [I32<BigEndian>],
}

// Serializable structs for each tag type
#[derive(Serialize)]
pub struct S15Fixed16ArrayTypeToml {
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matrix: Option<[[f64; 3]; 3]>,
}

impl From<&S15Fixed16ArrayType> for S15Fixed16ArrayTypeToml {
    fn from(xyz: &S15Fixed16ArrayType) -> Self {
        let layout = Layout::ref_from_bytes(&xyz.0).unwrap();

        let values: Vec<f64> = layout
            .values
            .iter()
            .map(|v| crate::from_s15fixed16(v.get()))
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
