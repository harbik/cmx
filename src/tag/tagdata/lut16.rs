// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{BigEndian, Immutable, KnownLayout, TryFromBytes, Unaligned, I32, U16};

use crate::tag::tagdata::Lut16Data;

#[derive(TryFromBytes, KnownLayout, Unaligned, Immutable)]
#[repr(C, packed)]
struct Lut16HeaderLayout {
    signature: [u8; 4], // "mft2"
    _reserved: [u8; 4], // reserved, must be 0
    i: u8,              // input channels
    o: u8,              // output channels
    g: u8,
    _padding: u8,               // padding byte, required to be 0
    e_mat: [I32<BigEndian>; 9], // s15Fixed16Number array
    n: U16<BigEndian>,          // number input table entries (n)
    m: U16<BigEndian>,          // number output table entries (m)
}

#[derive(Serialize)]
pub struct Lut16Type {
    g: usize,                   // number of grid points
    e_mat: [f64; 9],            // s15Fixed16Number array
    input_luts: Vec<Vec<u16>>,  // input LUT
    output_luts: Vec<Vec<u16>>, // output LUT
    multi_lut: Vec<Vec<u16>>,   // multi-dimensional LUT
}

impl From<&Lut16Data> for Lut16Type {
    fn from(lut16: &Lut16Data) -> Self {
        let (layout, _) = Lut16HeaderLayout::try_ref_from_prefix(&lut16.0).unwrap();

        // Header validation (debug-only): signature must be "mft2", reserved/padding must be zero.
        debug_assert_eq!(&layout.signature, b"mft2", "lut16Type: invalid signature");
        debug_assert_eq!(layout._reserved, [0; 4], "lut16Type: reserved must be zero");
        debug_assert_eq!(layout._padding, 0, "lut16Type: padding must be zero");

        let i = layout.i as usize;
        let o = layout.o as usize;
        let g = layout.g as usize;
        let n = layout.n.get() as usize;
        let m = layout.m.get() as usize;

        // Convert e_mat from s15Fixed16Number to f64
        let e_mat: [f64; 9] = layout
            .e_mat
            .iter()
            .map(|&v| v.get() as f64 / 65536.0)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();

        // Sizes and offsets (in bytes)
        let header_size = std::mem::size_of::<Lut16HeaderLayout>();
        let input_luts_size = 2usize * i * n; // 2 bytes per entry
        let clut_size = 2usize
            .checked_mul(g.pow(i as u32))
            .and_then(|v| v.checked_mul(o))
            .expect("lut16Type: CLUT size overflow");
        let output_luts_size = 2usize * o * m;

        // Bounds check: ensure the payload is large enough
        let total_size = header_size
            .checked_add(input_luts_size)
            .and_then(|v| v.checked_add(clut_size))
            .and_then(|v| v.checked_add(output_luts_size))
            .expect("lut16Type: total size overflow");
        assert!(
            lut16.0.len() >= total_size,
            "lut16Type: truncated data (have {}, need >= {})",
            lut16.0.len(),
            total_size
        );

        // Offsets
        let input_luts_offset = header_size;
        let clut_offset = input_luts_offset + input_luts_size;
        let output_luts_offset = clut_offset + clut_size;

        // Read input LUTs: i tables of n entries each (u16 BE)
        let input_luts: Vec<Vec<u16>> = lut16.0
            [input_luts_offset..input_luts_offset + input_luts_size]
            .chunks(2 * n)
            .map(|chunk| {
                chunk
                    .chunks(2)
                    .map(|pair| u16::from_be_bytes(pair.try_into().unwrap()))
                    .collect()
            })
            .collect();

        // Sanity: all input LUTs should have same length as n
        debug_assert!(input_luts.iter().all(|t| t.len() == n));

        // Read CLUT: g^i grid points, each with o outputs (u16 BE)
        let multi_lut = lut16.0[clut_offset..clut_offset + clut_size]
            .chunks(2 * o)
            .map(|chunk| {
                chunk
                    .chunks(2)
                    .map(|pair| u16::from_be_bytes(pair.try_into().unwrap()))
                    .collect::<Vec<u16>>()
            })
            .collect::<Vec<Vec<u16>>>();

        // Read output LUTs: o tables of m entries each (u16 BE)
        let output_luts: Vec<Vec<u16>> = lut16.0
            [output_luts_offset..output_luts_offset + output_luts_size]
            .chunks(2 * m)
            .map(|chunk| {
                chunk
                    .chunks(2)
                    .map(|pair| u16::from_be_bytes(pair.try_into().unwrap()))
                    .collect()
            })
            .collect();

        // Sanity: all output LUTs should have same length as m
        debug_assert!(output_luts.iter().all(|t| t.len() == m));

        Lut16Type {
            g,
            e_mat,
            input_luts,
            output_luts,
            multi_lut,
        }
    }
}
