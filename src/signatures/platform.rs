// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use num_derive::{FromPrimitive, ToPrimitive};

/// Represents the Profile Connection Space (PCS) of an ICC profile.
// The PCS defines the color space used for the profile connection, which is essential for color management.
// The PCS can be XYZ (CIE1931 XYZ), Lab (CIELAB), or Spectral.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum Platform {
    Apple = 0x4150504C,           // 'APPL' - Apple Computer Inc.
    Microsoft = 0x4D534654,       // 'MSFT' - Microsoft Corporation',
    SiliconGraphics = 0x53474920, // 'SGI ' - Silicon Graphics Inc.
    SunMicrosystems = 0x53554E57, // 'SUN ' - Sun Microsystems Inc.
}

/*
impl Platform {
    pub fn new(tag: Signature) -> Option<Platform> {
        match tag.0 {
            0x4150504C => Some(Self::Apple),           // 'APPL'
            0x4D534654 => Some(Self::Microsoft),       // 'MSFT'
            0x53474920 => Some(Self::SiliconGraphics), // 'SGI '
            0x53554E57 => Some(Self::SunMicrosystems), // 'SUN '
            _ => None
        }
    }
}

impl From<Platform> for Signature {
    fn from(platform: Platform) -> Self {
        Signature(platform as u32)
    }
}

 */
