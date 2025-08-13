// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::I32;
use zerocopy::{BigEndian, Immutable, KnownLayout, TryFromBytes, Unaligned};

use crate::tag::tagdata::Lut8Type;

#[derive(TryFromBytes, KnownLayout, Unaligned, Immutable)]
#[repr(C, packed)]
struct Lut8HeaderLayout {
    signature: [u8; 4], // "LUT8"
    _reserved: [u8; 4], // reserved, must be 0
    n: u8,              // input channels
    m: u8,              // output channels
    g: u8,
    _padding: u8,               // padding byte, required to be 0
    e_mat: [I32<BigEndian>; 9], // s15Fixed16Number array
}

#[derive(Serialize)]
pub struct Lut8TypeToml {
    g: usize,                  // number of grid points
    e_mat: [f64; 9],           // s15Fixed16Number array
    input_luts: Vec<Vec<u8>>,  // input LUT
    output_luts: Vec<Vec<u8>>, // output LUT
    multi_lut: Vec<u8>,        // multi-dimensional LUT
}

impl From<&Lut8Type> for Lut8TypeToml {
    fn from(lut8: &Lut8Type) -> Self {
        let (layout, _) = Lut8HeaderLayout::try_ref_from_prefix(&lut8.0).unwrap();
        let n = layout.n as usize;
        let m = layout.m as usize;
        let g = layout.g as usize;

        // Convert e_mat from s15Fixed16Number to f64
        let e_mat: [f64; 9] = layout
            .e_mat
            .iter()
            .map(|&v| v.get() as f64 / 65536.0)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();

        // Calculate sizes and offsets
        let header_size = 48; // 8 + 4 + 36
        let input_luts_size = n * 256;
        let clut_size = g.pow(n as u32) * m;
        let output_luts_size = m * 256;

        // Calculate offsets
        let input_luts_offset = header_size;
        let clut_offset = input_luts_offset + input_luts_size;
        let output_luts_offset = clut_offset + clut_size;

        // Read input LUTs
        let input_luts: Vec<Vec<u8>> = lut8.0
            [input_luts_offset..input_luts_offset + input_luts_size]
            .chunks(256)
            .map(|chunk| chunk.to_vec())
            .collect();

        // Read CLUT (multi-dimensional LUT)
        let multi_lut = lut8.0[clut_offset..clut_offset + clut_size].to_vec();

        // Read output LUTs
        let output_luts: Vec<Vec<u8>> = lut8.0
            [output_luts_offset..output_luts_offset + output_luts_size]
            .chunks(256)
            .map(|chunk| chunk.to_vec())
            .collect();

        Lut8TypeToml {
            g,
            e_mat,
            input_luts,
            output_luts,
            multi_lut,
        }
    }
}
