use crate::util::*;
use serde::Serialize;
use num::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Debug, Serialize)]
pub struct Chromaticity((Primaries, Vec<[f32;2]>));
impl Chromaticity {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let channels = read_be_u16(buf)? as usize;
        let primaries = FromPrimitive::from_u16(read_be_u16(buf)?).unwrap_or_default();
        let mut chromaticities = Vec::with_capacity(channels);
        for _ in 0..channels {
            let x = read_u16fixed16(buf)?;
            let y = read_u16fixed16(buf)?;
            chromaticities.push([x,y]);
        }
        Ok(Chromaticity((primaries, chromaticities)))
    }
}

#[derive(Debug, Serialize, FromPrimitive)]
pub enum Primaries {
    Absolute = 0x0000,
    ITU      = 0x0001,
    SMPTE    = 0x0002,
    EBU      = 0x0003,
    P22      = 0x0004,
}

impl Default for Primaries {
    fn default() -> Self {
        Self::Absolute
    }
}
