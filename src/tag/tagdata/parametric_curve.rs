// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use crate::{
    is_zero,
    tag::tagdata::{DataSignature, ParametricCurveData},
    S15Fixed16,
};
use serde::Serialize;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, I32, U16};

/// Represents the raw memory layout of an ICC `ParametricCurveData` tag.
#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
struct ReadLayout {
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    encoded_value: U16<BigEndian>,
    _reserved2: [u8; 2],
    /// Array of three CIEXYZ values, stored as s15Fixed16Numbers.
    parameters: [I32<BigEndian>],
}

#[derive(IntoBytes, KnownLayout, Unaligned, Immutable)]
#[repr(C, packed)]
struct WriteLayoutHeader {
    signature: [u8; 4],
    _reserved: [u8; 4],
    encoded_value: U16<BigEndian>,
    _reserved2: [u8; 2],
}

impl WriteLayoutHeader {
    pub fn new(size: usize) -> Self {
        let encoded_value = match size {
            1 => U16::<BigEndian>::new(0),
            3 => U16::<BigEndian>::new(1),
            4 => U16::<BigEndian>::new(2),
            5 => U16::<BigEndian>::new(3),
            7 => U16::<BigEndian>::new(4),
            _ => panic!("Unsupported number of parameters: {size}"),
        };
        Self {
            signature: DataSignature::ParametricCurveData.into(),
            _reserved: [0; 4],
            encoded_value,
            _reserved2: [0; 2],
        }
    }
}

/// Represents the raw memory layout of an ICC `ParametricCurveData` tag.
#[derive(IntoBytes, KnownLayout, Unaligned, Immutable)]
#[repr(C, packed)]
struct WriteLayout<const N: usize> {
    /// Signature of the tag, must be 'para'.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    encoded_value: U16<BigEndian>,
    _reserved2: [u8; 2],
    /// Array of three CIEXYZ values, stored as s15Fixed16Numbers.
    parameters: [I32<BigEndian>; N],
}

impl<const N: usize> WriteLayout<N> {
    /// Creates a new `WriteLayout` with the signature 'para' and initializes
    pub fn new(params: [f64; N]) -> Self {
        let encoded_value = match N {
            1 => U16::<BigEndian>::new(0),
            3 => U16::<BigEndian>::new(1),
            4 => U16::<BigEndian>::new(2),
            5 => U16::<BigEndian>::new(3),
            7 => U16::<BigEndian>::new(4),
            _ => panic!("Unsupported number of parameters: {N}"),
        };

        let mut parameters: [I32<BigEndian>; N] = [0.into(); N];
        for (i, &value) in params.iter().enumerate() {
            if i < N {
                // Convert f64 to s15Fixed16Number
                let s15fixed16 = crate::S15Fixed16::from(value);
                parameters[i] = s15fixed16.into(); // 65536 is the divisor for s15Fixed16
            }
        }

        Self {
            signature: DataSignature::ParametricCurveData.into(),
            _reserved: [0; 4],
            encoded_value,
            _reserved2: [0; 2],
            parameters,
        }
    }
}

// Serializable structs for each tag type
#[derive(Serialize)]
pub struct ParametricCurveType {
    #[serde(skip_serializing_if = "is_zero")]
    g: f64,
    #[serde(skip_serializing_if = "is_zero")]
    a: f64,
    #[serde(skip_serializing_if = "is_zero")]
    b: f64,
    #[serde(skip_serializing_if = "is_zero")]
    c: f64,
    #[serde(skip_serializing_if = "is_zero")]
    d: f64,
    #[serde(skip_serializing_if = "is_zero")]
    e: f64,
    #[serde(skip_serializing_if = "is_zero")]
    f: f64,
}

impl ParametricCurveType {
    pub fn values(&self) -> [f64; 7] {
        [self.g, self.a, self.b, self.c, self.d, self.e, self.f]
    }
}

impl ParametricCurveData {
    // TODO: rename to set_array, or remove all together in favor or set_slice
    pub fn set_parameters<const N: usize>(&mut self, parameters: [f64; N]) {
        self.0 = WriteLayout::new(parameters).as_bytes().to_vec();
    }

    pub fn set_parameters_slice(&mut self, values: &[f64]) {
        let n = values.len();
        let mut bytes = Vec::with_capacity(
            std::mem::size_of::<WriteLayoutHeader>() + n * std::mem::size_of::<I32<BigEndian>>(),
        );
        bytes.extend_from_slice(WriteLayoutHeader::new(n).as_bytes());
        let params: Vec<I32<BigEndian>> = values
            .iter()
            .map(|&v| crate::S15Fixed16::from(v).into())
            .collect();
        bytes.extend_from_slice(params.as_slice().as_bytes());
        self.0 = bytes;
    }
}

/// Parses the raw data wrapped in XYZData into a XYZDataToml instance,
/// as used
impl From<&ParametricCurveData> for ParametricCurveType {
    fn from(para: &ParametricCurveData) -> Self {
        let layout = ReadLayout::ref_from_bytes(&para.0).unwrap();

        // Flatten directly during the conversion
        let vec: Vec<f64> = layout
            .parameters
            .iter()
            .map(|&i| S15Fixed16::from(i).into())
            .collect();
        // Copy up to 7 values, defaulting the rest to zero
        let mut params = [0.0_f64; 7];
        for (i, v) in vec.iter().take(7).enumerate() {
            params[i] = *v;
        }
        let [g, a, b, c, d, e, f] = params;

        Self {
            g,
            a,
            b,
            c,
            d,
            e,
            f,
        }
    }
}
