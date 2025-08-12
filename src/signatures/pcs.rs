// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use crate::{signatures::Signature, Error};

/// Represents the Profile Connection Space (PCS) of an ICC profile.
// The PCS defines the color space used for the profile connection, which is essential for color management.
// The PCS can be XYZ (CIE1931 XYZ), Lab (CIELAB), or Spectral.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
pub enum Pcs {
    /// Profile Connection Space (PCS) for XYZ color space.
    XYZ = 0x58595A20, // 'XYZ '
    /// Profile Connection Space (PCS) for Lab color space.
    Lab = 0x4C616220, // 'Lab ',
    Spectral = 0x73706563, // 'spec' - Spectral PCS
}

impl Pcs {
    pub fn new(tag: Signature) -> Result<Self, Error> {
        match tag.0 {
            0x58595A20 => Ok(Self::XYZ),
            0x4C616220 => Ok(Self::Lab),
            0x73706563 => Ok(Self::Spectral),
            _ => Err(Error::InvalidPcsTag(tag)),
        }
    }
}

impl From<Pcs> for Signature {
    fn from(pcs: Pcs) -> Self {
        Signature(pcs as u32)
    }
}
