use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::Serialize;

use crate::types::common::{read_be_u16, read_be_u64, read_f32_from_u16_fixed16};

#[derive(Debug, Serialize, FromPrimitive)]
pub enum Primaries {
    Absolute = 0x0000,
    ITU = 0x0001,
    SMPTE = 0x0002,
    EBU = 0x0003,
    P22 = 0x0004,
}

impl Default for Primaries {
    fn default() -> Self {
        Self::Absolute
    }
}

#[derive(Debug, Serialize)]
pub struct Chromaticity((Primaries, Vec<[f32; 2]>));

impl Chromaticity {
    /// get one or more pairs of [x,y] chromaticity values,
    pub fn new(mut buf: &mut &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        // skip first 8 bytes, already know this is Chromaticity type, and skip the reserverd bytes
        let _ = read_be_u64(&mut buf); 
        let channels = read_be_u16(buf)? as usize;
        let primaries = FromPrimitive::from_u16(read_be_u16(buf)?).unwrap_or_default();
        let mut chromaticities = Vec::with_capacity(channels);
        for _ in 0..channels {
            let x = read_f32_from_u16_fixed16(buf)?;
            let y = read_f32_from_u16_fixed16(buf)?;
            chromaticities.push([x, y]);
        }
        Ok(Chromaticity((primaries, chromaticities)))
    }
}
