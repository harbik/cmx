use crate::common::*;
use serde::Serialize;
use isolang::Language;
use isocountry::CountryCode;

#[derive(Debug, Serialize)]
pub struct MultiLocalizedUnicode(Vec<(Option<CountryCode>, Language, String)>);

impl MultiLocalizedUnicode {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let n = read_be_u32(buf)? as usize;
        let mut pos = Vec::with_capacity(n);
        let twelve = read_be_u32(buf)?;
        if twelve != 12 { return Err("Incorrect multilocalized record length".into())}
        for _ in 0..n {
            let lang = read_ascii_string(buf, 2)?;
            let mut country = read_ascii_string(buf, 2)?;
            if country == "FU" {country = String::from("FR")}; // found in Generic CMYK Profile MacOS
            if country == "PO" {country = String::from("PT")}; // found in Generic CMYK Profile
            let length = read_be_u32(buf)? as usize;
            let start = (read_be_u32(buf)? - (16 + 12 * n as u32)) as usize;
            pos.push((lang, country, start, length));
        }
        let data = read_vec_u16(buf, buf.len())?;
        let mut mlu = Vec::new();
        for (lang, country, start, length) in pos {
            mlu.push((
                CountryCode::for_alpha2_caseless(country.as_str()).ok(),
                Language::from_639_1(lang.as_str()).unwrap(),
                String::from_utf16(&data[start/2..start/2+length/2])?  
            ));
        }

        Ok(Self(mlu))
    }
}