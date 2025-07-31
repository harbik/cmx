use crate::types::common::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Vcgt {
    Table(VcgtTable),
    Formula(VcgtFormula),
}

#[derive(Debug, Serialize)]
pub struct VcgtTable {
    pub channels: u16,
    pub entry_count: u16,
    //  pub entry_size: u16,
    pub data: Lut,
}

impl VcgtTable {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let n_ch = read_be_u16(buf)?;
        let entry_count = read_be_u16(buf)?;
        let entry_size = read_be_u16(buf)?;
        let data = match entry_size {
            1 => Lut::Bit8(read_vec(buf, buf.len())?),
            2 => Lut::Bit16(read_vec_u16(buf, buf.len())?),
            _ => return Err("entry_size error in VcgtTable".into()),
        };
        Ok(VcgtTable {
            channels: n_ch,
            entry_count,
            //         entry_size,
            data,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct VcgtFormula {
    pub red_gamma: f32,
    pub red_min: f32,
    pub red_max: f32,
    pub green_gamma: f32,
    pub green_min: f32,
    pub green_max: f32,
    pub blue_gamma: f32,
    pub blue_min: f32,
    pub blue_max: f32,
}

impl VcgtFormula {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        Ok(Self {
            red_gamma: read_s15fixed16(buf)?,
            red_min: read_s15fixed16(buf)?,
            red_max: read_s15fixed16(buf)?,
            green_gamma: read_s15fixed16(buf)?,
            green_min: read_s15fixed16(buf)?,
            green_max: read_s15fixed16(buf)?,
            blue_gamma: read_s15fixed16(buf)?,
            blue_min: read_s15fixed16(buf)?,
            blue_max: read_s15fixed16(buf)?,
        })
    }
}

impl Vcgt {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let vcgt_type = read_be_u32(buf)?;
        match vcgt_type {
            0 => Ok(Self::Table(VcgtTable::try_new(buf)?)),
            1 => Ok(Self::Formula(VcgtFormula::try_new(buf)?)),
            _ => todo!(),
        }
    }
}
