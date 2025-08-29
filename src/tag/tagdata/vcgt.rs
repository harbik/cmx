// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

#![allow(unused)]

use core::panic;

use serde::Serialize;
use zerocopy::{BigEndian, FromBytes, Immutable, KnownLayout, Ref, I32, U16, U32};

use crate::{
    s15fixed16,
    tag::{tagdata::VcgtData, TagSignature},
};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum VcgtType {
    #[serde(rename = "table")]
    Table(VcgtTable),
    #[serde(rename = "formula")]
    Formula(VcgtFormula),
}

// -------------------------------
// Structs for parsed data
// -------------------------------

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Lut {
    Bit8(Vec<u8>),   // 8-bit LUT
    Bit16(Vec<u16>), // 16-bit LUT
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VcgtTable {
    pub channels: u16,
    pub entry_count: u16,
    pub entry_size: u16,
    pub data: Lut,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VcgtFormula {
    pub red_gamma: f64,
    pub red_min: f64,
    pub red_max: f64,
    pub green_gamma: f64,
    pub green_min: f64,
    pub green_max: f64,
    pub blue_gamma: f64,
    pub blue_min: f64,
    pub blue_max: f64,
}

// -------------------------------
// Fixed header formats
// -------------------------------

#[repr(C)]
#[derive(Debug, FromBytes, Immutable, KnownLayout)]
struct HeaderLayout {
    signature: U32<BigEndian>, // b"vcgt"
    reserved: [u8; 4],
    tag_type: U32<BigEndian>, // 0 = Table, 1 = Formula
}

#[repr(C)]
#[derive(Debug, FromBytes, Immutable, KnownLayout)]
struct TableLayout {
    channels: U16<BigEndian>,    // number of channels (1, 3, or 4)
    entry_count: U16<BigEndian>, // number of entries per channel
    entry_size: U16<BigEndian>,  // bytes per sample: 1 or 2
}

#[derive(Debug, FromBytes, Immutable, KnownLayout)]
#[repr(C, packed)]
struct FormulaLayout {
    red_gamma: I32<BigEndian>,
    red_min: I32<BigEndian>,
    red_max: I32<BigEndian>,
    green_gamma: I32<BigEndian>,
    green_min: I32<BigEndian>,
    green_max: I32<BigEndian>,
    blue_gamma: I32<BigEndian>,
    blue_min: I32<BigEndian>,
    blue_max: I32<BigEndian>,
}

impl From<&VcgtData> for VcgtType {
    fn from(vcgt_data: &VcgtData) -> Self {
        let (header, rest) =
            HeaderLayout::ref_from_prefix(&vcgt_data.0).expect("vcgt: invalid header");

        if header.signature.get() != TagSignature::Vcgt.to_u32() {
            panic!("vcgt: invalid signature");
        }

        match header.tag_type.get() {
            0 => {
                let (table_hdr, data) =
                    TableLayout::ref_from_prefix(rest).expect("vcgt: invalid table data");

                let total_entries =
                    table_hdr.channels.get() as usize * table_hdr.entry_count.get() as usize;

                let lut = match table_hdr.entry_size.get() {
                    1 => {
                        if data.len() < total_entries {
                            panic!("vcgt: not enough 8-bit LUT data");
                        }
                        Lut::Bit8(data[..total_entries].to_vec())
                    }
                    2 => {
                        if data.len() < total_entries * 2 {
                            panic!("vcgt: not enough 16-bit LUT data");
                        }
                        let mut vec = Vec::with_capacity(total_entries);
                        for chunk in data[..total_entries * 2].chunks_exact(2) {
                            vec.push(u16::from_be_bytes([chunk[0], chunk[1]]));
                        }
                        Lut::Bit16(vec)
                    }
                    _ => panic!("vcgt: unsupported entry size"),
                };

                VcgtType::Table(VcgtTable {
                    channels: table_hdr.channels.get(),
                    entry_count: table_hdr.entry_count.get(),
                    entry_size: table_hdr.entry_size.get(),
                    data: lut,
                })
            }
            1 => {
                let (raw, _) =
                    FormulaLayout::ref_from_prefix(rest).expect("vcgt: invalid formula data");

                VcgtType::Formula(VcgtFormula {
                    red_gamma: s15fixed16(raw.red_gamma.get()),
                    red_min: s15fixed16(raw.red_min.get()),
                    red_max: s15fixed16(raw.red_max.get()),
                    green_gamma: s15fixed16(raw.green_gamma.get()),
                    green_min: s15fixed16(raw.green_min.get()),
                    green_max: s15fixed16(raw.green_max.get()),
                    blue_gamma: s15fixed16(raw.blue_gamma.get()),
                    blue_min: s15fixed16(raw.blue_min.get()),
                    blue_max: s15fixed16(raw.blue_max.get()),
                })
            }
            _ => panic!("vcgt: unknown tag type"),
        }
    }
}
