use crate::common::*;
use num_derive::FromPrimitive;
use num::FromPrimitive;
use serde::{Serialize, Deserialize};

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ColorSpaceSignature {
    NONE = 0,
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
    NC = 0x6e630000, // V5: n channel device data
}

impl ColorSpaceSignature {
    pub fn read(icc_buf: &mut &[u8]) -> Result<(Option<ColorSpaceSignature>, Option<u16>)> {
        let mut sig =read_be_u32(icc_buf)?;
        let n_channels = if (0x6e630001..=0x6e63ffff).contains(&sig) {
            let n = sig - 0x6e630000;
            sig = 0x6e630000;
            Some(n as u16)
        } else {
            None
        };
        match FromPrimitive::from_u32(sig) {
            Some(c) => 
                match c {
                    ColorSpaceSignature::NONE => Ok((None, None)),
                    _ => Ok((Some(c), n_channels)),
                } 
            None => Err("illegal profile color space".into()),
        }
    }

    pub fn to_be_bytes(&self, n_ch: u16) -> [u8;4] {
        if n_ch>0 {
            (Self::NC as u32 + n_ch as u32).to_be_bytes()
        } else {
            (*self as u32).to_be_bytes()
        }
    }
}