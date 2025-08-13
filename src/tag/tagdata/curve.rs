// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

#[derive(Serialize)]
pub struct CurveTypeToml {
    #[serde(skip_serializing_if = "Option::is_none")]
    points: Option<Vec<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gamma: Option<f64>,
}

impl From<&super::CurveType> for CurveTypeToml {
    fn from(curve: &super::CurveType) -> Self {
        let data = curve.data();
        if data.len() == 1 {
            let value = data[0] as f64 / 256.0;
            CurveTypeToml {
                points: None,
                gamma: Some(crate::round_to_precision(value, 4)),
            }
        } else {
            CurveTypeToml {
                points: Some(curve.data()),
                gamma: None,
            }
        }
    }
}

impl super::CurveType {
    /// Parses the raw big-endian bytes into a `Vec<u16>`.
    pub fn data(&self) -> Vec<u16> {
        //let count = u32::from_be_bytes(self.0[8..=11].try_into().unwrap());
        self.0[12..]
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes(chunk.try_into().unwrap()))
            .collect()
    }

    /// Converts a `Vec<u16>` into raw big-endian bytes and sets it as the tag's data.
    pub fn set_data(&mut self, data: &[u16]) {
        self.0[8..=12].copy_from_slice(&data.len().to_be_bytes());
        let values: Vec<u8> = data.iter().flat_map(|&value| value.to_be_bytes()).collect();
        // Resize the vector to fit the new data
        self.0.resize(12 + values.len(), 0);
        self.0[12..12 + values.len()].copy_from_slice(&values);
    }
}
