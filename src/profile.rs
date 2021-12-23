
#![allow(unused)]
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::ops::{RangeInclusive, Deref, DerefMut};
use std::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use half::f16;

// ICC profile file signature, used at location 36..40 in the profile header
const ACSP: u32 = 0x61637370; 
const SIG_NONE: &str = "\0\0\0\0";

#[derive(Default, Debug)]
pub struct Profile {
    pub cmm: Option<String>,
    pub version: [u8;3],
    pub class: Class,
    pub colorspace: Option<ColorSpace>, // V5: if none use spectral_pcs as A side spectra
    pub colorspace_channels: Option<u16>, // 1 to 0xFFFF
    pub pcs: Option<ColorSpace>,
    pub date_time: Option<DateTime<chrono::Utc>>,
    pub platform: Option<String>,
    pub profile_embedded: bool,
    pub profile_embedded_dependent: bool,
    pub profile_mcs_subset: Option<bool>, // V5 Flag
    pub manufacturer: Option<String>, // https://www.color.org/signatureRegistry/index.xalter
    pub device: Option<String>, // https://www.color.org/signatureRegistry/deviceRegistry/index.xalter
    pub media_transparent: bool,
    pub media_matt: bool,
    pub media_negative: bool,
    pub media_bw: bool,
    pub media_non_paper: Option<bool>, // V5 flag
    pub media_textured: Option<bool>, // V5
    pub media_non_isotropic: Option<bool>, // V5
    pub media_self_luminous: Option<bool>, // V5
    pub rendering_intent: RenderingIntent,
    pub pcs_illuminant: Option<[f64;3]>, // V2-4: X=0.964, Y=1.0, Z=0.824
    pub creator: Option<String>, // a manufacturer signature
    pub profile_id: Option<u128>,
    pub spectral_pcs: Option<SpectralColorSpace>,
    pub spectral_pcs_wavelength_range: Option<WavelengthRange>,
    pub bi_spectral_pcs_wavelength_range: Option<WavelengthRange>,
    pub mcs: Option<u16>,
    pub profile_device_sub_class: Option<u32>,
    // tags list
    pub tags: Vec<TagTableRow>,
}

impl Profile {
    pub fn from_buffer(mut icc_buf: &[u8]) -> Result<Profile, Box<dyn std::error::Error + 'static>> {
        let buf_len = icc_buf.len();
        let size = read_be_u32(&mut icc_buf)? as usize;
        if size<132 || buf_len!=size {return Err("ICC profile size error".into())}; // 128 header + 4 byte number of tags
        let cmm = read_signature(&mut icc_buf)?;
        let version = read_version(&mut icc_buf)?;
        let class = Class::read(&mut icc_buf)?;
        let (colorspace, colorspace_channels) = ColorSpace::read(&mut icc_buf)?;
        let (pcs, _)= ColorSpace::read(&mut icc_buf)?;
        let date_time = read_date_time(&mut icc_buf)?;
        let profile_file_signature = read_be_u32(&mut icc_buf)?;
        if profile_file_signature!= ACSP { return Err("Profile file signature error".into())};
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
        let profile_id = zero_as_none(read_be_u128(&mut icc_buf)?);
        let spectral_pcs = SpectralColorSpace::read(&mut icc_buf)?;
        let spectral_pcs_wavelength_range = WavelengthRange::read(&mut icc_buf)?;
        let bi_spectral_pcs_wavelength_range = WavelengthRange::read(&mut icc_buf)?;
        let mcs = read_mcs(&mut icc_buf)?;
        let profile_device_sub_class = zero_as_none(read_be_u32(&mut icc_buf)?);
        let _reserved = read_be_u32(&mut &mut icc_buf);

        // read tags pass 1
        // this will fill the `sig`, `offset`, and `length` fields.
        // as all the fixed length fields are consumed, the offset needs to be adjusted

        let tags_length = read_be_u32(&mut icc_buf)? as usize;
        let data_start = 128 + 4 + 12 * tags_length;

        let mut tags = Vec::with_capacity(tags_length);
        for i in 0..tags_length {
            let sig = read_signature(&mut icc_buf)?.ok_or("illegal tag signature")?;
            let offset = read_be_u32(&mut icc_buf)? as usize - data_start; // offset
            let length = read_be_u32(&mut icc_buf)? as usize;
            tags.push(TagTableRow::new(sig, offset, length));
        }


        // fill data pass 2
        // read tag data without consuming, as the data maybe accessed multiple times
        // and might not be in the same order as the tags
        for i in 0..tags_length {
            let start = tags[i].offset;
            let end = tags[i].offset + tags[i].length;
           // tags[i].data = Some(icc_buf[start..end].to_vec());
            tags[i].data = Some(crate::tags::Tag::try_new(&mut &icc_buf[start..end])?);
        }
        
        Ok(Profile {
            cmm, version, class, colorspace, colorspace_channels, pcs, date_time,
            platform, profile_embedded, profile_embedded_dependent, profile_mcs_subset,
            manufacturer, device, media_transparent, media_matt, media_negative, media_bw,
            media_non_paper, media_textured, media_non_isotropic, media_self_luminous,
            rendering_intent, pcs_illuminant, creator, profile_id, spectral_pcs, spectral_pcs_wavelength_range,
            bi_spectral_pcs_wavelength_range, mcs, profile_device_sub_class, tags,
        })

    }

    pub fn from_file(iccfile: &str) -> Result<Profile, Box<dyn std::error::Error + 'static>>  {
        let icc_data = std::fs::read(iccfile)?;
        Self::from_buffer(icc_data.as_slice())
    }


    pub fn new(version: [u8;3], class: Class) -> Self {
        let mut profile = Profile::default();
        profile.version = version;
        profile.class = class;
        profile.date_time = Some(chrono::Utc::now());
        profile
    }

    pub fn to_file(&self, iccfile: &str) -> Result<(), Box<dyn std::error::Error + 'static>>  {
        let icc_buf = self.to_buffer()?;
        Ok(std::fs::write(iccfile, icc_buf)?)
    }

    pub fn to_buffer(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
        let tags_data_length = self.tags.iter().fold(0usize, |len, t| len + t.aligned_length());
        let length = 128 + 4 + self.tags.len() * 12 + tags_data_length;
        let mut buf: Vec<u8> = Vec::with_capacity(length); // actual length might be smaller, correct at end
        buf.extend((length as u32).to_be_bytes());
        buf.extend([self.version[0], self.version[1]<<4_u8 | self.version[2], 0, 0]);
        buf.extend((self.class as u32).to_be_bytes());
        buf.extend(self.colorspace.unwrap_or(ColorSpace::NONE).to_be_bytes(self.colorspace_channels.unwrap_or(0)));
        buf.extend(self.pcs.unwrap_or(ColorSpace::NONE).to_be_bytes(0));
        buf.extend(datetime_to_be_bytes(self.date_time));
        buf.extend(ACSP.to_be_bytes());
        buf.extend(self.platform.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(self.profile_flags().to_be_bytes());
        buf.extend(self.manufacturer.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(self.device.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(self.device_flags().to_be_bytes());
        buf.extend((self.rendering_intent as u32).to_be_bytes());
        buf.extend(xyz_to_be_bytes(self.pcs_illuminant));
        buf.extend(self.creator.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(0u32.to_be_bytes()); // profile id
        buf.extend(self.spectral_pcs.unwrap_or(SpectralColorSpace::None).to_be_bytes());
        buf.extend(self.spectral_pcs_wavelength_range.clone().unwrap_or_default().to_be_bytes());
        buf.extend(self.bi_spectral_pcs_wavelength_range.clone().unwrap_or_default().to_be_bytes());
        buf.extend(mcs_to_be_bytes(self.mcs));
       
        Ok(buf)
    }

    pub fn profile_flags(&self) -> u32 {
        (self.profile_embedded as u32) << 0
        | (self.profile_embedded_dependent as u32) << 1
        | (self.profile_mcs_subset.unwrap_or(false) as u32) << 2
    }

    pub fn device_flags(&self) -> u32 {
        (self.media_transparent as u32) << 0
        | (self.media_matt as u32) << 1
        | (self.media_negative as u32) << 2
        | (self.media_bw as u32) << 3
        | (self.media_non_paper.unwrap_or(false) as u32) << 4
        | (self.media_textured.unwrap_or(false) as u32) << 5
        | (self.media_non_isotropic.unwrap_or(false) as u32) << 6
        | (self.media_self_luminous.unwrap_or(false) as u32) << 7
    }

}

#[derive(FromPrimitive, Clone, Copy, Debug)]
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

impl Default for Class {
    fn default() -> Self {
        Class::Input
    }
}

impl Class {
    fn read(icc_buf: &mut &[u8]) -> Result<Class, Box< dyn std::error::Error + 'static>> {
        match FromPrimitive::from_u32(read_be_u32(icc_buf)?) {
            Some(c) => Ok(c),
            None => Err("illegal profile class".into()),
        }
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug)]
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
    fn read(icc_buf: &mut &[u8]) -> Result<(Option<ColorSpace>, Option<u16>) , Box< dyn std::error::Error + 'static>> {
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
                    ColorSpace::NONE => Ok((None, None)),
                    _ => Ok((Some(c), n_channels)),
                } 
            None => Err("illegal profile color space".into()),
        }
    }

    fn to_be_bytes(&self, n_ch: u16) -> [u8;4] {
        if n_ch>0 {
            (Self::NC as u32 + n_ch as u32).to_be_bytes()
        } else {
            (*self as u32).to_be_bytes()
        }
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug)]
pub enum RenderingIntent {
    Perceptual = 0,
    MediaRelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,
}

impl Default for RenderingIntent {
    fn default() -> Self {
        RenderingIntent::Perceptual
    }
}

impl RenderingIntent {
    fn read(icc_buf: &mut &[u8]) -> Result<Self, Box< dyn std::error::Error + 'static>> {
        let sig =read_be_u32(icc_buf)?;
        Ok(FromPrimitive::from_u32(sig).ok_or("Illegal rendering intent value")?)
    }
}

// V5 BToDx/DToBx or brdfBToDx/brdfDToBx or directionalBToDx/directionalDToBx spectral colour space signatures
#[derive(Clone, Copy, Debug)]
pub enum SpectralColorSpace {
    None,
    Reflectance(u16),
    Transmission(u16),
    RadiantEmission(u16),
    BiSpectralReflectance(u16),
    BiSpectralReflectanceSparse(u16),
}

impl SpectralColorSpace {
    fn read(icc_buf: &mut &[u8]) -> Result<Option<Self>, Box< dyn std::error::Error + 'static>> {
        let sig = read_be_u16(icc_buf)?;
        let ch = read_be_u16(icc_buf)?;
        match sig {
            0 => Ok((None)),
            0x7273 => Ok(Some(SpectralColorSpace::Reflectance(ch))),
            0x7473 => Ok(Some(SpectralColorSpace::Transmission(ch))),
            0x6573 => Ok(Some(SpectralColorSpace::RadiantEmission(ch))),
            0x6273 => Ok(Some(SpectralColorSpace::BiSpectralReflectance(ch))),
            0x736d => Ok(Some(SpectralColorSpace::BiSpectralReflectanceSparse(ch))),
            _ => Err("Undefined Spectral Color Space found".into()),
        }
    }

    fn to_be_bytes(&self) -> [u8;4] {
        match self {
            &SpectralColorSpace::Reflectance(ch) => (0x7273u32 << 2 | ch as u32).to_be_bytes(),
            &SpectralColorSpace::Transmission(ch) => (0x7473u32 << 2 | ch as u32).to_be_bytes(),
            &SpectralColorSpace::RadiantEmission(ch) => (0x6573u32 << 2 | ch as u32).to_be_bytes(),
            &SpectralColorSpace::BiSpectralReflectance(ch) => (0x6273u32 << 2 | ch as u32).to_be_bytes(),
            &SpectralColorSpace::BiSpectralReflectanceSparse(ch) => (0x736du32 << 2 | ch as u32).to_be_bytes(),
            _ => [0,0,0,0],
        }
    }
}

#[derive(Clone, Debug)]
pub struct WavelengthRange ( RangeInclusive<f64>, usize);

impl WavelengthRange {

    fn read(icc_buf: &mut &[u8]) -> Result<Option<Self>, Box< dyn std::error::Error + 'static>> {
        let start = read_be_f16(icc_buf)?.to_f64();
        let end = read_be_f16(icc_buf)?.to_f64();
        let length = read_be_u16(icc_buf)? as usize;
        if length == 0 {
            Ok(None)
        } else {
            Ok(Some(Self(start..=end, length)))
        }
    }

    fn to_be_bytes(&self) -> [u8;12] {
        if self.1>0 {
            let mut v : Vec<u8> = Vec::new();
            v.extend(self.0.start().to_be_bytes());
            v.extend(self.0.end().to_be_bytes());
            v.extend(self.1.to_be_bytes());
            v.truncate(12);
            v.try_into().unwrap()
        } else {
            [0u8;12]
        }
    }
}

impl Default for WavelengthRange {
    fn default() -> Self {
        Self(0.0..=0.0, Default::default())
    }
}

#[derive(Debug)]
pub struct TagTableRow {
    sig: String,
    offset: usize,
    length: usize,
    data: Option<crate::tags::Tag>,
}

impl TagTableRow {
    pub fn new(sig: String, offset: usize, length: usize) -> Self { 
        Self { sig, offset, length, data: None } 
    }

    pub fn aligned_length(&self) -> usize {
        let rem = self.length%4;
        if rem == 0 {
            self.length
        } else {
            self.length - rem + 4
        }
    }
}


fn zero_as_none<T: num::Num + num::Zero>(v: T) -> Option<T> {
    if v.is_zero() {
        None
    } else {
        Some(v)
    }
}


pub fn read_be_f16(input: &mut &[u8]) -> Result<f16, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<f16>());
    *input = rest;
    Ok(f16::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_u16(input: &mut &[u8]) -> Result<u16, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
    *input = rest;
    Ok(u16::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_u32(input: &mut &[u8]) -> Result<u32, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    Ok(u32::from_be_bytes(int_bytes.try_into()?))
}


pub fn read_be_i32(input: &mut &[u8]) -> Result<i32, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<i32>());
    *input = rest;
    Ok(i32::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_u64(input: &mut &[u8]) -> Result<u64, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
    *input = rest;
    Ok(u64::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_u128(input: &mut &[u8]) -> Result<u128, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u128>());
    *input = rest;
    Ok(u128::from_be_bytes(int_bytes.try_into()?))
}

fn read_version(input: &mut &[u8]) -> Result<[u8;3], Box<dyn std::error::Error + 'static>> {
    let (version, rest) = input.split_at(std::mem::size_of::<[u8;4]>());
    *input = rest;
    Ok([version[0], version[1]>>4_u8, version[1]&0x0F_u8])
}


pub fn read_date_time(icc_buf: &mut &[u8]) -> Result <Option<DateTime<chrono::Utc>>, Box<dyn std::error::Error + 'static>> {
    let year = read_be_u16(icc_buf)?;
    let month = read_be_u16(icc_buf)?;
    let day = read_be_u16(icc_buf)?;
    let hour = read_be_u16(icc_buf)?;
    let minute = read_be_u16(icc_buf)?;
    let second = read_be_u16(icc_buf)?;
    if year == 0 && month == 0 && day == 0 {
        Ok(None)
    } else {
        let d = chrono::NaiveDate::from_ymd(year as i32, month as u32, day as u32);
        let t = chrono::NaiveTime::from_hms(hour as u32, minute as u32, second as u32);
        let dt = chrono::NaiveDateTime::new(d,t);
        Ok(Some(DateTime::from_utc(dt, chrono::Utc)))
    }
}

fn datetime_to_be_bytes(dt: Option<DateTime<chrono::Utc>>) -> [u8;12] {
    match dt {
        None => [0;12],
        Some(dt) => {
            let y = dt.date().year() as u16;
            let m = dt.date().month() as u16;
            let d = dt.date().day() as u16;
            let h = dt.time().hour() as u16;
            let m = dt.time().minute() as u16;
            let s = dt.time().second() as u16;
            let mut v: Vec<u8> = Vec::with_capacity(12);
            v.extend(y.to_be_bytes());
            v.extend(m.to_be_bytes());
            v.extend(d.to_be_bytes());
            v.extend(h.to_be_bytes());
            v.extend(m.to_be_bytes());
            v.extend(s.to_be_bytes());
            v.try_into().unwrap() // should not fail as generated from a datetime
        }
    }
}


pub fn read_signature(icc_buf: &mut &[u8]) -> Result<Option<String>, Box<dyn std::error::Error + 'static>>{
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

pub fn read_xyz(icc_buf: &mut &[u8]) -> Result< Option<[f64;3]>, Box<dyn std::error::Error + 'static>> {
    let x_i32 = read_be_i32(icc_buf)?;
    let y_i32 = read_be_i32(icc_buf)?;
    let z_i32 = read_be_i32(icc_buf)?;
    if x_i32 == 0 && y_i32 == 0 && z_i32 == 0 {
        Ok(None)
    } else {
        let x = x_i32 as f64/65536.0;
        let y = y_i32 as f64/65536.0;
        let z = z_i32 as f64/65536.0;
        Ok(Some([x,y,z]))
    }
}


fn xyz_to_be_bytes(xyz: Option<[f64;3]>) -> [u8;12] {
    match xyz {
       None =>  [0;12],
       Some([x,y,z]) => {
        let mut v: Vec<u8> = Vec::with_capacity(12);
        v.extend(((x * 655536.0) as i32).to_be_bytes());
        v.extend(((y * 655536.0) as i32).to_be_bytes());
        v.extend(((z * 655536.0) as i32).to_be_bytes());
        v.truncate(12);
        v.try_into().unwrap()

       }
    }
}

fn read_mcs(icc_buf: &mut &[u8]) -> Result<Option<u16>, Box<dyn std::error::Error + 'static>> {
    let sig = read_be_u16(icc_buf)?;
    let n = read_be_u16(icc_buf)?;
    if sig==0 || n==0 {
        Ok(None)
    } else {
        Ok(Some(n))
    }
}

fn mcs_to_be_bytes(n_mcs: Option<u16>) -> [u8;4] {
    let n = n_mcs.unwrap_or(0) as u32;
    if n == 0 {
        [0, 0, 0, 0]
    } else {
        (0x6d63u32 << 2 | n).to_be_bytes()
    }
}



