
#![allow(unused)]
use chrono::DateTime;
use std::ops::RangeInclusive;
use std::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Class {
    Input = 0x73636E72,
    Display = 0x6D6E7472,
    Output = 0x70727472,
    DeviceLink = 0x6C696E6B,
    ColorSpace = 0x73706163,
    Abstract = 0x061627374,
    NamedColor =  0x6E6D636C,
    // V5
    ColorEncodingSpace = 0x63656E63, 
    MultiplexIdentification = 0x6D696420,
    MultiplexLink = 0x6d6c6e6b,
    MultiplexVisualization = 0x6d766973,
}

impl Class {
    fn read(icc_buf: &mut &[u8]) -> Result<Class, Box< dyn std::error::Error + 'static>> {
        match FromPrimitive::from_u32(read_be_u32(icc_buf)?) {
            Some(c) => Ok(c),
            None => Err("illegal profile class".into()),
        }
    }
}

#[derive(FromPrimitive, PartialEq)]
pub enum ColorSpace {
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

impl ColorSpace {
    fn read(icc_buf: &mut &[u8]) -> Result<(ColorSpace, Option<u16>) , Box< dyn std::error::Error + 'static>> {
        let mut sig =read_be_u32(icc_buf)?;
        let n_channels = if (0x6e630001..=0x6e63ffff).contains(&sig) {
            let n = sig - 0x6e630000;
            sig = 0x6e630000;
            Some(n as u16)
        } else {
            None
        };
        match FromPrimitive::from_u32(sig) {
            Some(c) => Ok((c, n_channels)),
            None => Err("illegal profile color space".into()),
        }
    }
}

#[derive(FromPrimitive, PartialEq)]
pub enum RenderingIntent {
    Perceptual = 0,
    MediaRelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,
}

impl RenderingIntent {
    fn read(icc_buf: &mut &[u8]) -> Result<Self, Box< dyn std::error::Error + 'static>> {
        let sig =read_be_u32(icc_buf)?;
        Ok(FromPrimitive::from_u32(sig).ok_or("Illegal rendering intent value")?)
    }
}

// V5 BToDx/DToBx or brdfBToDx/brdfDToBx or directionalBToDx/directionalDToBx spectral colour space signatures
pub enum SpectralColorSpace {
    Reflectance(u16),
    Transmission(u16),
    RadiantEmission(u16),
    BiSpectralReflectance(u16),
    BiSpectralReflectanceSparse(u16),
}

pub struct Tag {
    id: u32,
    data: Vec<u8>
}

pub struct Profile {
    pub cmm: Option<String>,
    pub version: [u8;3],
    pub class: Class,
    pub colorspace: ColorSpace, // V5: if none use spectral_pcs as A side spectra
    pub colorspace_channels: Option<u16>, // 1 to 0xFFFF
    pub pcs: ColorSpace,
    pub date_time: DateTime<chrono::Utc>,
    pub platform: Option<String>,
    pub profile_embedded: bool,
    pub profile_embedded_dependent: bool,
    pub profile_mcs_subset: Option<bool>, // V5
    pub manufacturer: Option<String>, // https://www.color.org/signatureRegistry/index.xalter
    pub device: Option<String>, // https://www.color.org/signatureRegistry/deviceRegistry/index.xalter
    pub media_transparent: bool,
    pub media_matt: bool,
    pub media_negative: bool,
    pub media_bw: bool,
    pub media_non_paper: Option<bool>, // V5
    pub media_textured: Option<bool>, // V5
    pub media_non_isotropic: Option<bool>, // V5
    pub media_self_luminous: Option<bool>, // V5
    pub rendering_intent: RenderingIntent,
    pub pcs_illuminant: [f64;3], // V2-4: X=0.964, Y=1.0, Z=0.824
    pub creator: Option<String>, // a manufacturer signature
    pub profile_id: Option<u128>,
    pub spectral_pcs: Option<SpectralColorSpace>,
    pub spectral_pcs_wavelength_range: Option<RangeInclusive<f64>>,
    pub spectral_pcs_wavelength_steps: Option<u16>,
    pub bi_spectral_pcs_wavelength_range: Option<RangeInclusive<f64>>,
    pub bi_spectral_pcs_wavelength_steps: Option<u16>,
    pub multiplex_n_channels: Option<u16>,
    pub profile_device_sub_class: Option<u32>,
    // tags list
    pub tags: Vec<Tag>
}

impl Profile {
    pub fn from_slice(mut icc_buf: &[u8]) -> Result<Profile, Box<dyn std::error::Error + 'static>> {

        let size = read_be_u32(&mut icc_buf)?;
        if size<128 || icc_buf.len()<128 {return Err("ICC profile size error".into())};
        let cmm = read_signature(&mut icc_buf)?;
        let version = read_version(&mut icc_buf)?;
        let class = Class::read(&mut icc_buf)?;
        let (colorspace, colorspace_channels) = ColorSpace::read(&mut icc_buf)?;
        let (pcs, _)= ColorSpace::read(&mut icc_buf)?;
        if  pcs != ColorSpace::XYZ && pcs != ColorSpace::Lab && pcs != ColorSpace::NONE  {
            return Err("PCS Color Space should be 'XYZ', 'Lab', or 'NONE'".into())
        }
        let date_time = read_date_time(&mut icc_buf)?;
        //println!("{}", date_time);
        let profile_file_signature = read_be_u32(&mut icc_buf)?;
        if profile_file_signature!=0x61637370 { return Err("Profile file signature error".into())};
        let platform = read_signature(&mut icc_buf)?;
        let (profile_embedded, profile_embedded_dependent, pf_mcs) = read_profile_flags(&mut icc_buf)?;
        let profile_mcs_subset = if version[0] >= 5 {
            Some(pf_mcs)
        } else {
            None
        };
        let manufacturer = read_signature(&mut icc_buf)?;
        let device= read_signature(&mut icc_buf)?;
        let ([media_transparent, media_matt, media_negative, media_bw], v5attr) = read_attribute_flags(&mut icc_buf)?;
        let media_non_paper = if version[0]>=5 { Some(v5attr[0])} else {None};
        let media_textured = if version[0]>=5 { Some(v5attr[1])} else {None};
        let media_non_isotropic = if version[0]>=5 { Some(v5attr[2])} else {None};
        let media_self_luminous = if version[0]>=5 { Some(v5attr[3])} else {None};
        let rendering_intent = RenderingIntent::read(&mut icc_buf)?;
        let pcs_illuminant = read_xyz(&mut icc_buf)?;
        let creator= read_signature(&mut icc_buf)?;
        let profile_id = {
            let id = read_be_u128(&mut icc_buf)?;
            if id == 0 {
            None
            }  else {
            Some(id)
            }
        };
        let spectral_pcs = None;
        let spectral_pcs_wavelength_range = None;
        let spectral_pcs_wavelength_steps = None;
        let bi_spectral_pcs_wavelength_range = None;
        let bi_spectral_pcs_wavelength_steps = None;
        let multiplex_n_channels = None;
        let profile_device_sub_class = None;
        let tags = vec![];
        
        Ok(Profile {
            cmm, version, class, colorspace, colorspace_channels, pcs, date_time,
            platform, profile_embedded, profile_embedded_dependent, profile_mcs_subset,
            manufacturer, device, media_transparent, media_matt, media_negative, media_bw,
            media_non_paper, media_textured, media_non_isotropic, media_self_luminous,
            rendering_intent, pcs_illuminant, creator, profile_id, spectral_pcs, spectral_pcs_wavelength_range,
            spectral_pcs_wavelength_steps, bi_spectral_pcs_wavelength_range, bi_spectral_pcs_wavelength_steps, 
            multiplex_n_channels, profile_device_sub_class, tags
        })

    }

    pub fn from_file(iccfile: &str) -> Result<Profile, Box<dyn std::error::Error + 'static>>  {
        let icc_data = std::fs::read(iccfile)?;
        Self::from_slice(icc_data.as_slice())
    }
}


fn read_be_u16(input: &mut &[u8]) -> Result<u16, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
    *input = rest;
    Ok(u16::from_be_bytes(int_bytes.try_into()?))
}

fn read_be_u32(input: &mut &[u8]) -> Result<u32, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    Ok(u32::from_be_bytes(int_bytes.try_into()?))
}

fn read_be_i32(input: &mut &[u8]) -> Result<i32, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<i32>());
    *input = rest;
    Ok(i32::from_be_bytes(int_bytes.try_into()?))
}

fn read_be_u64(input: &mut &[u8]) -> Result<u64, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
    *input = rest;
    Ok(u64::from_be_bytes(int_bytes.try_into()?))
}

fn read_be_u128(input: &mut &[u8]) -> Result<u128, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u128>());
    *input = rest;
    Ok(u128::from_be_bytes(int_bytes.try_into()?))
}

fn read_version(input: &mut &[u8]) -> Result<[u8;3], Box<dyn std::error::Error + 'static>> {
    let (version, rest) = input.split_at(std::mem::size_of::<[u8;4]>());
    *input = rest;
    Ok([version[0], version[1]>>4_u8, version[1]&0x0F_u8])
}

fn read_date_time(icc_buf: &mut &[u8]) -> Result <DateTime<chrono::Utc>, Box<dyn std::error::Error + 'static>> {
    let year = read_be_u16(icc_buf)?;
    let month = read_be_u16(icc_buf)?;
    let day = read_be_u16(icc_buf)?;
    let hour = read_be_u16(icc_buf)?;
    let minute = read_be_u16(icc_buf)?;
    let second = read_be_u16(icc_buf)?;
    let d = chrono::NaiveDate::from_ymd(year as i32, month as u32, day as u32);
    let t = chrono::NaiveTime::from_hms(hour as u32, minute as u32, second as u32);
    let dt = chrono::NaiveDateTime::new(d,t);
    Ok(DateTime::from_utc(dt, chrono::Utc))
}

fn read_signature(icc_buf: &mut &[u8]) -> Result<Option<String>, Box<dyn std::error::Error + 'static>>{
    let (s, rest) = icc_buf.split_at(std::mem::size_of::<[u8;4]>());
    *icc_buf = rest;
    if s[0]!=0 && s[1]!=0 && s[2]!=0 && s[3]!=0 {
        Ok(Some(std::str::from_utf8(s)?.to_owned()))
    } else {
        Ok(None)
    }
}

fn read_profile_flags(icc_buf: &mut &[u8]) -> Result<(bool, bool, bool), Box<dyn std::error::Error + 'static>> {
    let pf = read_be_u32(icc_buf)?;
    Ok((
        (pf & (1 << 0)) != 0,
        (pf & (1 << 1)) != 0,
        (pf & (1 << 2)) != 0,
    ))
}

fn read_attribute_flags(icc_buf: &mut &[u8]) -> Result<([bool;4], [bool;4]), Box<dyn std::error::Error + 'static>> {
    let pf = read_be_u64(icc_buf)?;
    let mut flags: Vec<bool> = Vec::with_capacity(8);
    for i in 0..8 {
        flags.push((pf & (1 << i)) != 0)
    }
    Ok((flags[0..4].try_into().unwrap(), flags[4..8].try_into().unwrap()))
}

fn read_xyz(icc_buf: &mut &[u8]) -> Result< [f64;3], Box<dyn std::error::Error + 'static>> {
    let x = read_be_i32(icc_buf)? as f64/65536.0;
    let y = read_be_i32(icc_buf)? as f64/65536.0;
    let z = read_be_i32(icc_buf)? as f64/65536.0;
    Ok([x,y,z])
}
