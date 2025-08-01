mod chromaticity;
mod common;
mod lut8;
mod make_model;
mod measurement;
mod multi_localized_unicode;
mod named_color2;
mod native_display_info;
mod parametric_curve;
mod text_description;
mod vcgp;
mod vcgt;
mod viewing_conditions;

use parametric_curve::ParametricCurve;
use serde::Serialize;
use text_description::TextDescription;
use vcgt::Vcgt;
use vcgp::Vcgp;
use viewing_conditions::ViewingConditions;
use crate::{
    tags::{technology::Technology, typesignatures::TypeSignature},
    types::common::{
        read_be_f16, read_be_f32, read_be_f64, read_be_u16, read_be_u32, read_date_time,
        read_s15fixed16_array, read_xyz,
    },
};
use chromaticity::Chromaticity;
use lut8::Lut8;
use make_model::MakeAndModel;
use measurement::Measurement;
use multi_localized_unicode::MultiLocalizedUnicode;
use named_color2::NamedColor2;
use native_display_info::NativeDisplayInfo;


#[derive(Debug, Serialize)]
pub enum TagData {
    Chromaticity(Chromaticity),
    ColorantOrder(ColorantOrder),                  // 'clro'
    Curve(Curve),                                  // 'data' with flag 1
    Data(Data),                                    // 'data' with flag 1
    DateTime(DateTime),                            // 'dtim'
    Dict(Vec<u8>),                                 // 'dict'
    EmbeddedHeigthImage(Vec<u8>),                  // 'ehim'
    EmbeddedNormalImage(Vec<u8>),                  // 'enim'
    Float16Array(Vec<half::f16>),                  // 'fl16'
    Float32Array(Vec<f32>),                        // 'fl32'
    Float64Array(Vec<f64>),                        // 'fl64'
    GamutBoundaryDescription(Vec<u8>),             // 'gbd'
    Lut8(Lut8),
    LutAToB(Vec<u8>),                              // 'mAB'
    LutBToA(Vec<u8>),                              // 'mBA'
    Measurement(Measurement),                      // 'meas'
    MakeAndModel(MakeAndModel),                    // 'mmod'
    MultiLocalizedUnicode(MultiLocalizedUnicode),  // 'mluc'
    MultiProcessElements(Vec<u8>),                 // 'mpet'
    NativeDisplayInfo(NativeDisplayInfo),
    NamedColor2(NamedColor2),                      // 'ncl2'
    ParametricCurve(ParametricCurve),              // 'para'
    S15Fixed16Array(Vec<f32>),                     // 'sf32'
    Signature([u8; 4]),                            // 'sig'
    SparseMatrixArray(Vec<u8>),                    // 'smat'
    SpectralViewingConditions(Vec<u8>),            // 'svcn'
    TagStruct(Vec<u8>),                            // 'tstr'
    Technology(Technology),               // tag derived type
    Text(String),
    TextDescription(TextDescription),
    U16Fixed16Array(Vec<f32>),                     // 'uf32'
    UInt8Array(Vec<u8>),                           // 'ui16'
    UInt16Array(Vec<u16>),                         // 'ui16'
    UInt32Array(Vec<u32>),                         // 'ui16'
    UInt64Array(Vec<u64>),                         // 'ui64'
    Utf8(Vec<String>),                             // 'utf8'
    Utf16(Vec<String>),                            // 'ut16'
    Utf8Zip(Vec<String>),                          // 'zut8'
    Vcgt(Vcgt),                                    // 'vcgt'
    Vcgp(Vcgp),                                    // 'vcgt'
    ViewingConditions(ViewingConditions),
    XYZ(XYZ),                                      // 'XYZ'
    Custom(TypeSignature, Vec<u8>),               // unknown data type
}

impl TagData {
    /// Parse the Data Block
    /// Every data block starts with a 4 bytes type signature, and followed
    /// by 4 bytes of reserved data.
    /// The actual data starts at byte offset 8.
    pub fn new(
        buf: &mut &[u8],
    ) -> common::Result<Self> {
        let u32 = read_be_u32(buf)?;
        let type_signature = TypeSignature::from_u32(u32);
        match type_signature {
            TypeSignature::ChromaticityType => {
                Ok(Self::Chromaticity(Chromaticity::new(buf)?))
            }
            TypeSignature::ColorantOrderType => {
                Ok(Self::ColorantOrder(ColorantOrder(buf.to_owned())))
            }
            TypeSignature::CurveType => {
                let n = read_be_u32(buf)? as usize;
                let mut v: Vec<u16> = Vec::with_capacity(n);
                for _ in 0..n {
                    v.push(read_be_u16(buf)?);
                }
                Ok(Self::Curve(Curve(v)))
            }
            TypeSignature::DataType => {
                let _n = read_be_u32(buf)? as usize;
                Ok(Self::Data(Data(buf.to_owned())))
            }
            TypeSignature::DateTimeType => {
                Ok(Self::DateTime(DateTime(read_date_time(buf)?.unwrap())))
            }
            TypeSignature::Float16ArrayType => {
                let mut v = Vec::with_capacity(buf.len() / std::mem::size_of::<half::f16>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f16(buf)?)
                }
                Ok(Self::Float16Array(v))
            }
            TypeSignature::Float32ArrayType => {
                let mut v = Vec::with_capacity(buf.len() / std::mem::size_of::<f32>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f32(buf)?)
                }
                Ok(Self::Float32Array(v))
            }
            TypeSignature::Float64ArrayType => {
                let mut v = Vec::with_capacity(buf.len() / std::mem::size_of::<f64>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f64(buf)?)
                }
                Ok(Self::Float64Array(v))
            }
            TypeSignature::Lut8Type => Ok(Self::Lut8(Lut8::try_new(buf)?)),
            TypeSignature::MakeAndModelType => {
                Ok(Self::MakeAndModel(MakeAndModel::try_new(buf)?))
            }
            TypeSignature::MeasurementType => {
                Ok(Self::Measurement(Measurement::try_new(buf)?))
            }
            TypeSignature::MultiLocalizedUnicodeType => Ok(Self::MultiLocalizedUnicode(
                MultiLocalizedUnicode::try_new(buf)?,
            )),
            TypeSignature::NativeDisplayInfoType => {
                Ok(Self::NativeDisplayInfo(NativeDisplayInfo::try_new(buf)?))
            }
            TypeSignature::NamedColor2Type => {
                Ok(Self::NamedColor2(NamedColor2::try_new(buf, 3)?)) // TODO! pcs size???
            }
            TypeSignature::ParametricCurveType => {
                Ok(Self::ParametricCurve(ParametricCurve::try_new(buf)?))
            }
            TypeSignature::S15Fixed16ArrayType => {
                Ok(Self::S15Fixed16Array(read_s15fixed16_array(buf, None)?))
            }
            TypeSignature::TextType => Ok(Self::Text(
                std::str::from_utf8(buf)?
                    .trim_end_matches(char::from(0))
                    .to_owned(),
            )),
            TypeSignature::TextDescriptionType => {
                Ok(Self::TextDescription(TextDescription::try_new(buf)?))
            }
            TypeSignature::ViewingConditionsType => {
                Ok(Self::ViewingConditions(ViewingConditions::try_new(buf)?))
            }
            TypeSignature::XYZArrayType => {
                let n = buf.len() / 12;
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    if let Some(xyz) = read_xyz(buf)? {
                        v.push(xyz);
                    } else {
                        v.push([0.0, 0.0, 0.0]);
                    }
                }
                Ok(Self::XYZ(XYZ(v)))
            }
            TypeSignature::VcgtType => {
                Ok(Self::Vcgt(Vcgt::try_new(buf)?))
            }
            TypeSignature::VcgpType => {
                Ok(Self::Vcgp(Vcgp::try_new(buf)?))
            }
            TypeSignature::SignatureType => Ok(Self::Technology(
                num::FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
            )),
            _ => Ok(Self::Custom(type_signature, buf.to_owned())),
        }
    }
}

// Tag Type definitions
// Simple tag types defined here, complex tag types in separate files

#[derive(Debug, Serialize)]
pub struct ColorantOrder(Vec<u8>);

#[derive(Debug, Serialize)]
pub struct Curve(Vec<u16>);

#[derive(Debug, Serialize)]
pub struct Data(Vec<u8>);

#[derive(Debug, Serialize)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

#[derive(Debug, Serialize)]
pub struct Float16Array(Vec<half::f16>);

#[derive(Debug, Serialize)]
pub struct Float32Array(Vec<f32>);

#[derive(Debug, Serialize)]
pub struct Float64Array(Vec<f64>);

#[derive(Debug, Serialize)]
pub struct Text(String);


#[derive(Debug, Serialize)]
pub struct XYZ(Vec<[f64; 3]>);
