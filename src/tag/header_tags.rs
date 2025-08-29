// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

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
