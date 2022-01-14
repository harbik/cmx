
#![allow(unused)]
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::Serializer;
use serde::ser::SerializeStruct;
use std::ops::{RangeInclusive, Deref, DerefMut};
use std::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use half::f16;
use serde::Serialize;

use crate::util::*;
use crate::tags::{Tag};
use crate::signatures::tag::{TagSignature};

// ICC profile file signature, used at location 36..40 in the profile header
const ACSP: u32 = 0x61637370; 
const SIG_NONE: &str = "\0\0\0\0";

#[derive(Default, Debug, Serialize)]
#[serde(default)]
pub struct Profile {
    pub cmm: Option<crate::signatures::cmm::CmmSignature>,
    pub version: [u8;3],
    pub class: Class,
    pub colorspace: Option<ColorSpace>, // V5: if none use spectral_pcs as A side spectra

    pub pcs: Option<ColorSpace>,
    pub date_time: Option<DateTime<chrono::Utc>>,
    pub platform: Option<String>,
    pub flags: ProfileFlags,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>, // https://www.color.org/signatureRegistry/index.xalter

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>, // https://www.color.org/signatureRegistry/deviceRegistry/index.xalter

    pub attributes: DeviceAttributes,
    pub rendering_intent: RenderingIntent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcs_illuminant: Option<[f64;3]>, // V2-4: X=0.964, Y=1.0, Z=0.824

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>, // a manufacturer signature

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<u128>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectral_pcs: Option<SpectralColorSpace>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectral_pcs_wavelength_range: Option<WavelengthRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bi_spectral_pcs_wavelength_range: Option<WavelengthRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcs: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_device_sub_class: Option<u32>,
    // tags list
    pub tags: Vec<crate::tags::Tag>,
}

impl Profile {
    pub fn from_buffer(mut icc_buf: &[u8]) -> Result<Profile> {
        let buf_len = icc_buf.len();
        let size = read_be_u32(&mut icc_buf)? as usize;
        if size<132 || buf_len!=size {return Err("ICC profile size error".into())}; // 128 header + 4 byte number of tags
      //  let cmm = read_signature(&mut icc_buf)?;
        let cmm = crate::signatures::cmm::CmmSignature::new(read_be_u32(&mut icc_buf)?);
        let version = read_version(&mut icc_buf)?;
        let class = Class::read(&mut icc_buf)?;
        let colorspace = ColorSpace::read(&mut icc_buf)?;
        //let (colorspace, colorspace_channels) = ColorSpaceSignature::read(&mut icc_buf)?;
        let pcs= ColorSpace::read(&mut icc_buf)?;
        let date_time = read_date_time(&mut icc_buf)?;
        let profile_file_signature = read_be_u32(&mut icc_buf)?;
        if profile_file_signature!= ACSP { return Err("Profile file signature error".into())};
        let platform = read_signature(&mut icc_buf)?;
        let flags = ProfileFlags::new(&mut icc_buf)?;
        let manufacturer = read_signature(&mut icc_buf)?;
        let device= read_signature(&mut icc_buf)?;
        let attributes = DeviceAttributes::new(&mut icc_buf, version[0])?;
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
            cmm, version, class, colorspace, pcs, date_time,
            platform, flags, 
            manufacturer, device, attributes,
            rendering_intent, pcs_illuminant, creator, profile_id, spectral_pcs, spectral_pcs_wavelength_range,
            bi_spectral_pcs_wavelength_range, mcs, profile_device_sub_class, tags,
        })

    }

    pub fn from_file(iccfile: &str) -> Result<Profile>  {
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

    pub fn to_file(&self, iccfile: &str) -> Result<()>  {
        let icc_buf = self.to_buffer()?;
        Ok(std::fs::write(iccfile, icc_buf)?)
    }

    pub fn to_buffer(&self) -> Result<Vec<u8>> {
        let length = 128 + 4 + self.tags.len() * 100;
        let mut buf: Vec<u8> = Vec::with_capacity(length); // actual length might be smaller, correct at end
        buf.extend((length as u32).to_be_bytes());
        buf.extend([self.version[0], self.version[1]<<4_u8 | self.version[2], 0, 0]);
        buf.extend((self.class as u32).to_be_bytes());
        buf.extend(self.colorspace.unwrap_or_default().to_be_bytes());
        buf.extend(self.pcs.unwrap_or_default().to_be_bytes());
        buf.extend(datetime_to_be_bytes(self.date_time));
        buf.extend(ACSP.to_be_bytes());
        buf.extend(self.platform.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(self.flags.to_be_bytes());
        buf.extend(self.manufacturer.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(self.device.clone().unwrap_or(SIG_NONE.to_string()).as_bytes());
        buf.extend(self.attributes.to_be_bytes());
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

#[derive(FromPrimitive, Clone, Copy, Debug, Serialize)]
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
    fn read(icc_buf: &mut &[u8]) -> Result<Class> {
        match FromPrimitive::from_u32(read_be_u32(icc_buf)?) {
            Some(c) => Ok(c),
            None => Err("illegal profile class".into()),
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct ProfileFlags{
    pub embedded_profile: bool,
    pub use_with_embedded_data_only: bool,
    pub mcs_needs_subset: bool,
}

impl ProfileFlags {

    fn new(icc_buf: &mut &[u8]) -> Result<Self> {
        let pf = read_be_u32(icc_buf)?;
        Ok(Self{
            embedded_profile: (pf & (1<<0)) !=0,
            use_with_embedded_data_only: (pf & (1<<1)) !=0,
            mcs_needs_subset: (pf & (1<<2)) !=0,
        })
    }

    fn to_be_bytes(&self) -> [u8;4] {
        let v = self.embedded_profile as u32 
        | (self.use_with_embedded_data_only as u32) << 1
        | (self.mcs_needs_subset as u32) << 2;
        v.to_be_bytes()
    }
}

#[derive(Default, Debug)]
pub struct DeviceAttributes{ // u64!
    pub transparency: bool,
    pub matte: bool,
    pub media_negative: bool,
    pub media_black_and_white: bool, 
    pub non_paper_based: bool,
    pub textured: bool,
    pub non_isotropic: bool,
    pub self_luminous: bool,
    pub vendor: u32,
    pub version: u8,

}

impl DeviceAttributes {

    fn new(icc_buf: &mut &[u8], version: u8) -> Result<Self> {
        let v = read_be_u64(icc_buf)?;
        Ok(Self{
            transparency: (v & (1<<0)) !=0,
            matte: (v & (1<<1)) !=0,
            media_negative: (v & (1<<2)) !=0,
            media_black_and_white: (v & (1<<3)) !=0,
            non_paper_based: (v & (1<<4)) !=0,
            textured: (v & (1<<5)) !=0,
            non_isotropic: (v & (1<<6)) !=0,
            self_luminous: (v & (1<<7)) !=0,
            vendor: (v>>32) as u32,
            version,
        })
    }

    fn to_be_bytes(&self) -> [u8;8] {
        let v = (self.vendor as u64) << 32;
        v
        | (self.transparency as u64) << 0
        | (self.matte as u64) << 1
        | (self.media_negative as u64) << 2
        | (self.media_black_and_white as u64) << 3
        | (self.non_paper_based as u64) << 4
        | (self.textured as u64) << 5
        | (self.non_isotropic as u64) << 6
        | (self.self_luminous as u64) << 7;
        v.to_be_bytes()
    }
}

impl Serialize for DeviceAttributes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let n: usize = match (self.version, self.vendor) {
            (5..,0) => 8,
            (5..,_) => 9,
            (_, 0) => 4,
            _ => 5,
        };
        let mut state = serializer.serialize_struct("attributes", n)?;
        state.serialize_field("transparancy", &self.transparency)?;
        state.serialize_field("matte", &self.matte)?;
        state.serialize_field("media_negative", &self.media_negative)?;
        state.serialize_field("media_black_and_white", &self.media_black_and_white)?;
        if self.version >=5 {
            state.serialize_field("non_paper_based", &self.non_paper_based)?;
            state.serialize_field("textured", &self.textured)?;
            state.serialize_field("non_isotropic", &self.non_isotropic)?;
            state.serialize_field("self_luminous", &self.self_luminous)?;
        }
        if self.vendor!=0 {
            state.serialize_field("vendor", &self.vendor)?;
        }
        state.end()
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize)]
#[serde(default)]
pub struct ColorSpace {
    space: ColorSpaceSignature,

    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<u16>,
}

impl ColorSpace {
    fn read(icc_buf: &mut &[u8]) -> Result<Option<ColorSpace>> {
        let (signature, channels) = ColorSpaceSignature::read(icc_buf)?;
        match signature {
            Some(sig) =>  Ok(Some(Self { space: sig, channels})),
            None => Ok(None),
        }
    }

    fn to_be_bytes(&self) -> [u8;4] {
        match self.channels {
            Some(n) => (ColorSpaceSignature::NC as u32 + n as u32).to_be_bytes(),
            None => (self.space as u32).to_be_bytes()
        }
    }
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self { space: ColorSpaceSignature::NONE, channels: Default::default() }
    }
}


#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
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
    fn read(icc_buf: &mut &[u8]) -> Result<(Option<ColorSpaceSignature>, Option<u16>)> {
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

    fn to_be_bytes(&self, n_ch: u16) -> [u8;4] {
        if n_ch>0 {
            (Self::NC as u32 + n_ch as u32).to_be_bytes()
        } else {
            (*self as u32).to_be_bytes()
        }
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
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
    fn read(icc_buf: &mut &[u8]) -> Result<Self> {
        let sig =read_be_u32(icc_buf)?;
        Ok(FromPrimitive::from_u32(sig).ok_or("Illegal rendering intent value")?)
    }
}

// V5 BToDx/DToBx or brdfBToDx/brdfDToBx or directionalBToDx/directionalDToBx spectral colour space signatures
#[derive(Clone, Copy, Debug, Serialize)]
pub enum SpectralColorSpace {
    None,
    Reflectance(u16),
    Transmission(u16),
    RadiantEmission(u16),
    BiSpectralReflectance(u16),
    BiSpectralReflectanceSparse(u16),
}

impl SpectralColorSpace {
    fn read(icc_buf: &mut &[u8]) -> Result<Option<Self>> {
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

#[derive(Clone, Debug, Serialize)]
pub struct WavelengthRange ( RangeInclusive<f64>, usize);

impl WavelengthRange {

    fn read(icc_buf: &mut &[u8]) -> Result<Option<Self>> {
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

#[derive(Debug, Serialize)]
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
