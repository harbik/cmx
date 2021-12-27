
#![allow(unused)]
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::ops::{RangeInclusive, Deref, DerefMut};
use std::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use half::f16;

use crate::tags::{TagSignature, Tag};

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
    pub flags: ProfileFlags,
    pub manufacturer: Option<String>, // https://www.color.org/signatureRegistry/index.xalter
    pub device: Option<String>, // https://www.color.org/signatureRegistry/deviceRegistry/index.xalter
    pub attributes: DeviceAttributes,
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
    pub tags: Vec<crate::tags::Tag>,
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
        let flags = ProfileFlags::new(&mut icc_buf)?;
        let manufacturer = read_signature(&mut icc_buf)?;
        let device= read_signature(&mut icc_buf)?;
        let attributes = DeviceAttributes::new(&mut icc_buf)?;
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

        let tags_length = read_be_u32(&mut icc_buf)? as usize;
        let data_start = 128 + 4 + 12 * tags_length;

        let mut tag_table = Vec::with_capacity(tags_length);
        for i in 0..tags_length {
            let sig = read_tag_signature(&mut icc_buf)?;
            let offset = read_be_u32(&mut icc_buf)? as usize - data_start; // offset
            let length = read_be_u32(&mut icc_buf)? as usize;
            tag_table.push(TagTableRow::new(sig, offset, length));
        }

        let mut tags = Vec::with_capacity(tags_length);
        for tag_record in tag_table {
            let start = tag_record.offset;
            let end = start + tag_record.length;
            // tags[i].data = Some(icc_buf[start..end].to_vec());
            tags.push(crate::tags::Tag::try_new(tag_record.sig, &mut &icc_buf[start..end])?);
        }
        
        Ok(Profile {
            cmm, version, class, colorspace, colorspace_channels, pcs, date_time,
            platform, flags, 
            manufacturer, device, attributes,
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
        let length = 128 + 4 + self.tags.len() * 100;
        let mut buf: Vec<u8> = Vec::with_capacity(length); // actual length might be smaller, correct at end
        buf.extend((length as u32).to_be_bytes());
        buf.extend([self.version[0], self.version[1]<<4_u8 | self.version[2], 0, 0]);
        buf.extend((self.class as u32).to_be_bytes());
        buf.extend(self.colorspace.unwrap_or(ColorSpace::NONE).to_be_bytes(self.colorspace_channels.unwrap_or(0)));
        buf.extend(self.pcs.unwrap_or(ColorSpace::NONE).to_be_bytes(0));
        buf.extend(datetime_to_be_bytes(self.date_time));
        buf.extend(ACSP.to_be_bytes());
        buf.extend(self.platform.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(self.flags.0.to_be_bytes());
        buf.extend(self.manufacturer.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(self.device.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(self.attributes.0.to_be_bytes());
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

#[derive(Default, Debug)]
pub struct ProfileFlags(u32);

impl ProfileFlags {

    fn new(icc_buf: &mut &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_be_u32(icc_buf)?))
    }

    fn embedded(&self) -> bool {
       (self.0 & (1<<0)) !=0
    }

    fn dependent(&self) -> bool {
       (self.0 & (1<<1)) !=0
    }

    fn mcs(&self) -> bool {
       (self.0 & (1<<2)) !=0
    }
}

#[derive(Default, Debug)]
pub struct DeviceAttributes(u64);

impl DeviceAttributes {

    fn new(icc_buf: &mut &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_be_u64(icc_buf)?))
    }

    pub fn transparent(&self) -> bool { self.get(0) }
    pub fn matt(&self) -> bool {self.get(1) }
    pub fn negative(&self) -> bool { self.get(2) }
    pub fn black_and_white(&self) -> bool { self.get(3) }
    pub fn not_paper(&self) -> bool {self.get(4) }
    pub fn textured(&self) -> bool { self.get(5) }
    pub fn non_isotropic(&self) -> bool { self.get(6) }
    pub fn self_luminous(&self) -> bool { self.get(7) }

    pub fn set_transparent(&mut self) { self.set(0) }
    pub fn set_reflective(&mut self) { self.clear(0) }
    pub fn set_matt(&mut self) {self.set(1) }
    pub fn set_gloss(&mut self) { self.clear(1) }
    pub fn set_negative(&mut self) { self.set(2) }
    pub fn set_positive(&mut self) { self.clear(2) }
    pub fn set_black_and_white(&mut self) { self.set(3) }
    pub fn set_color(&mut self) { self.clear(3) }
    pub fn set_non_paper(&mut self) {self.set(4) }
    pub fn set_paper(&mut self) { self.clear(4) }
    pub fn set_textured(&mut self) { self.set(5) }
    pub fn set_smooth(&mut self) { self.clear(5) }
    pub fn set_non_isotropic(&mut self) { self.set(6) }
    pub fn set_isotropic(&mut self) { self.clear(6) }
    pub fn set_self_luminous(&mut self) { self.set(7) }
    pub fn set_colorant(&mut self) { self.clear(7) }

    pub fn get(&self, i: usize) -> bool { (self.0 & (1<<i)) !=0 }
    pub fn set(&mut self, i: usize) { self.0 |= (1<<i) }
    pub fn clear(&mut self, i: usize) { self.0 &= !(1<<i) }
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
    sig: TagSignature,
    offset: usize,
    length: usize,
}

impl TagTableRow {
    pub fn new(sig: TagSignature, offset: usize, length: usize) -> Self { 
        Self { sig, offset, length } 
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

pub fn read_be_f32(input: &mut &[u8]) -> Result<f32, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<f32>());
    *input = rest;
    Ok(f32::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_f64(input: &mut &[u8]) -> Result<f64, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<f64>());
    *input = rest;
    Ok(f64::from_be_bytes(int_bytes.try_into()?))
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

pub fn read_tag_signature(icc_buf: &mut &[u8]) -> Result<TagSignature, Box<dyn std::error::Error + 'static>>{
    let s = read_be_u32(icc_buf)?;
    match FromPrimitive::from_u32(s) {
        Some(tag_sig) => Ok(tag_sig),
        None => Err("Unknown tag".into()),
    }
    
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



