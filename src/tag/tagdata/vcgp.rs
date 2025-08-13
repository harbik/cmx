// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

#![allow(unused)]
use serde::Serialize;

// Typical structure for Vcgp tag
pub struct VcgpData {
    // Header/signature
    signature: [u8; 4], // "vcgp"

    // Parameters that might include:
    gamma: f32,       // Gamma value (e.g., 2.2)
    black_point: f32, // Black level adjustment
    white_point: f32, // White level adjustment

                      // Potentially other parameters for:
                      // - Contrast
                      // - Brightness
                      // - Individual RGB channel controls
}

/*
use crate::tags::common::*;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Vcgp {
    tbd: Vec<u8>, // can not find any information about this tag
}

impl Vcgp {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        Ok(Vcgp {
            tbd: read_vec(buf, buf.len())?,
        })
    }
}

 */
