use crate::util::*;
use serde::Serialize;
use num::Zero;

#[derive(Debug, Serialize)]
#[serde(default)]
pub struct TextDescription{
    pub ascii: String,
    #[serde(skip_serializing_if = "u32::is_zero")]
    pub unicode_language_code: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub unicode: String,
    #[serde(skip_serializing_if = "u16::is_zero")]
    pub scriptcode_code: u16,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub scriptcode: String,
}

impl TextDescription {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let n = read_be_u32(buf)? as usize;
        let ascii = read_ascii_string(buf, n)?;
        let unicode_language_code = read_be_u32(buf)?;
        let m = read_be_u32(buf)? as usize;
        let unicode = read_unicode_string(buf, m)?;
        let scriptcode_code = read_be_u16(buf)?;
        let l = read_u8(buf)? as usize;
        let scriptcode= read_ascii_string(buf, l)?;
        Ok(TextDescription{
            ascii,
            unicode_language_code,
            unicode,
            scriptcode_code,
            scriptcode
        })
    }
}