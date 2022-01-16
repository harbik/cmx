use crate::common::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NativeDisplayInfo{
    pub red_phosphor: [f32;2],
    pub green_phosphor: [f32;2],
    pub blue_phosphor: [f32;2],
    pub white_point: [f32;2],
    pub red_gamma_value: f32,
    pub green_gamma_value: f32,
    pub blue_gamma_value: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma_channels: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma_data: Option<Lut>,

}

impl NativeDisplayInfo{
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let _size = read_be_u32(buf)?;
        let red_phosphor = [read_s15fixed16(buf)?, read_s15fixed16(buf)?];
        let green_phosphor = [read_s15fixed16(buf)?, read_s15fixed16(buf)?];
        let blue_phosphor = [read_s15fixed16(buf)?, read_s15fixed16(buf)?];
        let white_point = [read_s15fixed16(buf)?, read_s15fixed16(buf)?];
        let red_gamma_value = read_s15fixed16(buf)?;
        let green_gamma_value = read_s15fixed16(buf)?;
        let blue_gamma_value = read_s15fixed16(buf)?;
        let gamma_channels = zero_as_none(read_be_u16(buf)?);
        let _gamma_entry_count = read_be_u16(buf)?;
        let gamma_entry_size = read_be_u16(buf)?;
        let gamma_data = match gamma_entry_size {
            0 => None,
            1 => {
                Some(Lut::Bit8(read_vec(buf, buf.len())?))
            }
            2 => {
                Some(Lut::Bit16(read_vec_u16(buf, buf.len())?))
            }
            _ => return Err("size error in NativeDisplayInfo".into())
        };

        Ok(NativeDisplayInfo{
            red_phosphor,
            green_phosphor,
            blue_phosphor,
            white_point,
            red_gamma_value,
            green_gamma_value,
            blue_gamma_value,
            gamma_channels,
            gamma_data,
        })
    }
}