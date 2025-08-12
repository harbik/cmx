// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use crate::signatures::Signature;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, serde::Serialize)]
#[repr(u32)]
pub enum DeviceClass {
    Unknown,
    Input = 0x73636E72,      // 'scnr'
    Display = 0x6D6E7472,    // 'mntr'
    Output = 0x70727472,     // 'prtr'
    DeviceLink = 0x6C696E6B, // 'link'
    Abstract = 0x61627374,   // 'abst'
    ColorSpace = 0x73706163, // 'spac'
    NamedColor = 0x6E6D636C, // 'nmcl'
    // ICC.2 (ICCmax) additions:
    Spectral = 0x73706563, // 'spec'
}

impl DeviceClass {
    pub fn new(tag: Signature) -> Self {
        match tag.0 {
            0x73636E72 => DeviceClass::Input,
            0x6D6E7472 => DeviceClass::Display,
            0x70727472 => DeviceClass::Output,
            0x6C696E6B => DeviceClass::DeviceLink,
            0x61627374 => DeviceClass::Abstract,
            0x73706163 => DeviceClass::ColorSpace,
            0x6E6D636C => DeviceClass::NamedColor,
            0x73706563 => DeviceClass::Spectral,
            _ => DeviceClass::Unknown,
        }
    }
}

impl From<DeviceClass> for Signature {
    fn from(device_class: DeviceClass) -> Self {
        Signature(device_class as u32)
    }
}

impl From<DeviceClass> for u32 {
    fn from(device_class: DeviceClass) -> Self {
        device_class as u32
    }
}
