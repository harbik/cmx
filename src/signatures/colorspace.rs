// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::signatures::Signature;

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize, Deserialize, strum::Display)]
pub enum ColorSpace {
    NONE = 0,
    XYZ = 0x58595A20,
    Lab = 0x4C616220,
    Luv = 0x4C757620,
    YCbr = 0x59436272,
    Yxy = 0x59787920,
    RGB = 0x52474220,
    Gray = 0x47524159,
    HSV = 0x48535620,
    HLS = 0x484C5320,
    CMYK = 0x434D594B,
    CMY = 0x434D5920,
    CLR2 = 0x32434C52,
    CLR3 = 0x33434C52,
    CLR4 = 0x34434C52,
    CLR5 = 0x35434C52,
    CLR6 = 0x36434C52,
    CLR7 = 0x37434C52,
    CLR8 = 0x38434C52,
    CLR9 = 0x39434C52,
    CLRA = 0x41434C52,
    CLRB = 0x42434C52,
    CLRC = 0x43434C52,
    CLRD = 0x44434C52,
    CLRE = 0x45434C52,
    CLRF = 0x46434C52,
    NC = 0x6e630000, // V5: n channel device data
}

impl ColorSpace {
    pub fn new(tag: Signature) -> Self {
        match tag.0 {
            0x58595A20 => ColorSpace::XYZ,
            0x4C616220 => ColorSpace::Lab,
            0x4C757620 => ColorSpace::Luv,
            0x59436272 => ColorSpace::YCbr,
            0x59787920 => ColorSpace::Yxy,
            0x52474220 => ColorSpace::RGB,
            0x47524159 => ColorSpace::Gray,
            0x48535620 => ColorSpace::HSV,
            0x484C5320 => ColorSpace::HLS,
            0x434D594B => ColorSpace::CMYK,
            0x434D5920 => ColorSpace::CMY,
            0x32434C52 => ColorSpace::CLR2,
            0x33434C52 => ColorSpace::CLR3,
            0x34434C52 => ColorSpace::CLR4,
            0x35434C52 => ColorSpace::CLR5,
            0x36434C52 => ColorSpace::CLR6,
            0x37434C52 => ColorSpace::CLR7,
            0x38434C52 => ColorSpace::CLR8,
            0x39434C52 => ColorSpace::CLR9,
            0x41434C52 => ColorSpace::CLRA,
            0x42434C52 => ColorSpace::CLRB,
            0x43434C52 => ColorSpace::CLRC,
            0x44434C52 => ColorSpace::CLRD,
            0x45434C52 => ColorSpace::CLRE,
            0x46434C52 => ColorSpace::CLRF,
            0x6E630000 => ColorSpace::NC, // V5: n channel device data
            _ => ColorSpace::NONE,
        }
    }
}

impl From<ColorSpace> for Signature {
    fn from(color_space: ColorSpace) -> Self {
        Signature(color_space as u32)
    }
}
