// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

// use std::ffi::os_str::Display; // Removed incorrect import

use std::fmt::Display;

use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U32};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    Normal = 0,
    Draft = 1,
    High = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interpolate {
    True = 0,
    False = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamutCheck {
    True = 0,
    False = 1,
}

#[derive(FromBytes, IntoBytes, Unaligned, KnownLayout, Immutable, Debug, Clone, Copy)]
#[repr(C)]
pub struct S15Fixed16(U32<BigEndian>);

/// A 15.16 fixed-point number, where the first 15 bits are the integer part and the last 16 bits are the fractional part.
/// This is used in ICC profiles to represent color values.
/// The value is stored as a 32-bit unsigned integer in big-endian format.
impl From<S15Fixed16> for f64 {
    fn from(value: S15Fixed16) -> Self {
        let s15 = value.0.get() as i32;
        s15 as f64 / 65536.0
    }
}

impl From<f64> for S15Fixed16 {
    fn from(value: f64) -> Self {
        let s15 = (value * 65536.0).round() as i32;
        S15Fixed16(U32::new(s15 as u32))
    }
}

impl Display for S15Fixed16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", f64::from(*self))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(C)]
pub enum RenderingIntent {
    Perceptual = 0,
    RelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,
}

impl RenderingIntent {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => RenderingIntent::Perceptual,
            1 => RenderingIntent::RelativeColorimetric,
            2 => RenderingIntent::Saturation,
            3 => RenderingIntent::AbsoluteColorimetric,
            _ => {
                panic!("Invalid RenderingIntent value: {value}");
            }
        }
    }
}

impl From<u32> for RenderingIntent {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}
