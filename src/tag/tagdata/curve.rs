// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{BigEndian, Immutable, IntoBytes, Unaligned, U32, U16};

use super::CurveData;

#[derive(Serialize)]
pub struct CurveType {
    #[serde(skip_serializing_if = "Option::is_none")]
    points: Option<Vec<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gamma: Option<f64>,
}

impl From<&CurveData> for CurveType {
    fn from(curve: &CurveData) -> Self {
        let data: Vec<u16> = {
            //let count = u32::from_be_bytes(self.0[8..=11].try_into().unwrap());
            curve.0[12..]
                .chunks_exact(2)
                .map(|chunk| u16::from_be_bytes(chunk.try_into().unwrap()))
                .collect()
        };
        if data.len() == 1 {
            let value = data[0] as f64 / 256.0;
            CurveType {
                points: None,
                gamma: Some(crate::round_to_precision(value, 4)),
            }
        } else {
            CurveType {
                points: Some(data),
                gamma: None,
            }
        }
    }
}

#[derive(IntoBytes, Unaligned, Immutable)]
#[repr(C, packed)]
struct WriteLayout<const N: usize> {
    signature: [u8; 4],
    reserved: [u8; 4],
    count: U32<BigEndian>,
    data: [U16<BigEndian>; N],
}

impl<const N:usize>  WriteLayout<N> {
    /// Creates a new `WriteLayout` with the signature 'curv' and initializes
    /// the count and data fields.
    pub fn new(data: [u16; N]) -> Self {
        let data: [U16<BigEndian>; N] = data.map(|x| U16::<BigEndian>::new(x));
        Self {
            signature: super::DataSignature::CurveData.into(),
            reserved: [0; 4],
            count: U32::<BigEndian>::new(data.len() as u32),
            data
        }
    }
}

impl CurveData {
    /// Parses the raw big-endian bytes into a `Vec<u16>`.
    /*
    pub fn data(&self) -> Vec<u16> {
        //let count = u32::from_be_bytes(self.0[8..=11].try_into().unwrap());
        self.0[12..]
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes(chunk.try_into().unwrap()))
            .collect()
    }
     */

    /// Converts a `Vec<u16>` into raw big-endian bytes and sets it as the tag's data.
    pub fn set_data<const N: usize>(&mut self, data: [u16; N]) {
        let data_bytes= WriteLayout::new(data);
        self.0 = data_bytes.as_bytes().to_vec();
    }

    /// Sets the gamma value for the curve.
    /// This will convert the gamma value to a single point curve.
    /// If the curve already has points, it will replace them with a single point.
    /// 
    pub fn set_gamma(&mut self, gamma: f64) {
        let value = (gamma * 256.0).round() as u16;
        self.set_data([value]);
    }
}
