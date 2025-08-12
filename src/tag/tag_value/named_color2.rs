// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

#![allow(unused)]
use serde::Serialize;
/*
use crate::tags::common::*;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct NamedColor2 {
    pub flags: u32,
    pub prefix: String,
    pub suffix: String,
    pub colors: Vec<(String, Vec<u16>, Vec<u16>)>,
}

impl NamedColor2 {
    pub fn try_new(buf: &mut &[u8], dim_pcs: usize) -> Result<Self> {
        let flags = read_be_u32(buf)?;
        let count = read_be_u32(buf)? as usize;
        let device_coordinates = read_be_u32(buf)? as usize;
        let prefix = read_ascii_string(buf, 32)?;
        let suffix = read_ascii_string(buf, 32)?;
        let mut colors = Vec::with_capacity(count);
        for _ in 0..count {
            let root = read_ascii_string(buf, 32)?;
            let pcs = read_vec_u16(buf, 2 * dim_pcs)?;
            let device = read_vec_u16(buf, 2 * device_coordinates)?;
            colors.push((root, pcs, device));
        }
        Ok(Self {
            flags,
            prefix,
            suffix,
            colors,
        })
    }
}
*/
