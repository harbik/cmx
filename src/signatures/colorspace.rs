// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

#[derive(
    PartialEq,
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    strum::Display,
    FromPrimitive,
    ToPrimitive,
)]
#[repr(u32)]
pub enum ColorSpace {
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
    #[cfg(feature = "v5")]
    NC = 0x6e630000, // V5: n channel device data
}

/*
impl ColorSpace {
    pub fn new(tag: Signature) -> Option<Self> {
        match tag.0 {
                0x58595A20 => Some(Self::XYZ),
                0x4C616220 => Some(Self::Lab),
                0x4C757620 => Some(Self::Luv),
                0x59436272 => Some(Self::YCbr),
                0x59787920 => Some(Self::Yxy),
                0x52474220 => Some(Self::RGB),
                0x47524159 => Some(Self::Gray),
                0x48535620 => Some(Self::HSV),
                0x484C5320 => Some(Self::HLS),
                0x434D594B => Some(Self::CMYK),
                0x434D5920 => Some(Self::CMY),
                0x32434C52 => Some(Self::CLR2),
                0x33434C52 => Some(Self::CLR3),
                0x34434C52 => Some(Self::CLR4),
                0x35434C52 => Some(Self::CLR5),
                0x36434C52 => Some(Self::CLR6),
                0x37434C52 => Some(Self::CLR7),
                0x38434C52 => Some(Self::CLR8),
                0x39434C52 => Some(Self::CLR9),
                0x41434C52 => Some(Self::CLRA),
                0x42434C52 => Some(Self::CLRB),
                0x43434C52 => Some(Self::CLRC),
                0x44434C52 => Some(Self::CLRD),
                0x45434C52 => Some(Self::CLRE),
                0x46434C52 => Some(Self::CLRF),
                #[cfg(feature = "v5")]
                0x6E630000 => Some(Self::NC), // V5: n channel device data
            _ => None
        }
    }
}

impl From<ColorSpace> for Signature {
    fn from(color_space: ColorSpace) -> Self {
        Signature(color_space as u32)
    }
}

 */
