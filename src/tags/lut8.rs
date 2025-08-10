/*
use crate::tags::common::*;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Lut8 {
    pub n: usize, // input channels
    pub m: usize, // output channels
    pub k: usize,
    pub e_mat: Vec<f32>,
    pub input_lut: Vec<u8>,
    pub output_lut: Vec<u8>,
    pub multi_lut: Vec<u8>,
}

impl Lut8 {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let n = read_u8(buf)? as usize;
        let m = read_u8(buf)? as usize;
        let k = read_u8(buf)? as usize;
        let _ = read_u8(buf)?; // padding
        let e_mat = read_s15fixed16_array(buf, 36.into())?;
        let input_lut = read_vec(buf, (n * 256).into())?;
        let n_i32 = (n as i32).try_into()?;
        let multi_lut = read_vec(buf, (k.pow(n_i32) * m).into())?;
        let output_lut = read_vec(buf, (m * 256).into())?;
        Ok(Lut8 {
            n,
            m,
            k,
            e_mat,
            input_lut,
            output_lut,
            multi_lut,
        })
    }
}

 */