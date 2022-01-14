
use crate::util::*;
use serde::Serialize;
use super::measurement::StandardIlluminant;
use num::FromPrimitive;

#[derive(Debug, Serialize)]
pub struct ViewingConditions {
    pub xyz_illuminant: [f64;3],
    pub xyz_surround: [f64;3],
    pub illuminant: StandardIlluminant,
}

impl ViewingConditions {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        Ok(ViewingConditions{
            xyz_illuminant: read_xyz(buf)?.unwrap_or([0.0, 0.0, 0.0]),
            xyz_surround: read_xyz(buf)?.unwrap_or([0.0, 0.0, 0.0]),
            illuminant: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
        })
    }
}
