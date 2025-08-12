// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

#![allow(unused)]
use serde::Serialize;
/*
use crate::tags::common::*;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Measurement {
    pub standard_observer: StandardObserver,
    pub xyz: [f64; 3],
    pub geometry: Geometry,
    pub flare: Flare,
    pub illuminant: StandardIlluminant,
}

impl Measurement {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        Ok(Measurement {
            standard_observer: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
            xyz: read_xyz(buf)?.unwrap_or_default(),
            geometry: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
            flare: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
            illuminant: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
        })
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum StandardIlluminant {
    Unknown = 0x00000000,
    D50 = 0x00000001,
    D65 = 0x00000002,
    D93 = 0x00000003,
    F2 = 0x00000004,
    D55 = 0x00000005,
    A = 0x00000006,
    EquiPowerE = 0x00000007, /* Equi-Power (E) */
    F8 = 0x00000008,

    /* The following illuminants are defined for V5 */
    BlackBody = 0x00000009, /* defined by CCT */
    Daylight = 0x0000000A,  /* defiend by CCT */
    B = 0x0000000B,
    C = 0x0000000C,
    F1 = 0x0000000D,
    F3 = 0x0000000E,
    F4 = 0x0000000F,
    F5 = 0x00000010,
    F6 = 0x00000011,
    F7 = 0x00000012,
    F9 = 0x00000013,
    F10 = 0x00000014,
    F11 = 0x00000015,
    F12 = 0x00000016,
}

impl Default for StandardIlluminant {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum StandardObserver {
    Unknown = 0x00000000,           /* Unknown observer */
    Cie1931TwoDegrees = 0x00000001, /* 1931 two degrees */
    Cie1964TenDegrees = 0x00000002, /* 1961 ten degrees */
}

impl Default for StandardObserver {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum Geometry {
    Unknown = 0x00000000,       /* Unknown geometry */
    Normal45 = 0x00000001,      /* 0/45, 45/0 */
    NormalDiffuse = 0x00000002, /* 0/d or d/0 */
}

impl Default for Geometry {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum Flare {
    Flare0 = 0x00000000,   /* 0% flare */
    Flare100 = 0x00000001, /* 100% flare */
}

impl Default for Flare {
    fn default() -> Self {
        Self::Flare0
    }
}
*/
