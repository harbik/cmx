
pub mod chromaticity;
pub mod lut8;
pub mod make_model;
pub mod measurement;
pub mod multi_localized_unicode;
pub mod named_color2;
pub mod parametric_curve;
pub mod text_description;
pub mod vcgt;
pub mod viewing_conditions;

use crate::{util::*, signatures::tag::TagSignature, signatures::tagtype::TagTypeSignature, signatures::technology::TechnologySignature};
use num::FromPrimitive;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Tag {
    tag_signature: TagSignature,
    type_signature: TagTypeSignature,
    data: TagData, // enum(object) pattern
}

impl Tag {
    pub fn try_new(tag_signature: TagSignature, buf: &mut &[u8]) -> Result<Self> {
        let t = read_be_u32(buf)?;
        let type_signature = match FromPrimitive::from_u32(t) {
            Some(c) => c,
            None => {
           //     println!("undefined tag type {:x?} {:?}", t, std::str::from_utf8(&t.to_be_bytes())?);
                TagTypeSignature::UndefinedType
            }
        };
        let _reserved = read_be_u32(buf)?;
        Ok(Self {
            tag_signature: tag_signature.clone(),
            type_signature,
            data: TagData::try_new(tag_signature, type_signature, buf)?,
        })
    }
}


#[derive(Debug, Serialize)]
pub enum TagData {
    Chromaticity(Chromaticity),
    ColorantOrder(ColorantOrder), // 'clro'
    Curve(Curve), // 'data' with flag 1
    Data(Data), // 'data' with flag 1
    DateTime(DateTime), // 'dtim'
    Dict(Vec<u8>), // 'dict' 
    EmbeddedHeigthImage(Vec<u8>), // 'ehim'
    EmbeddedNormalImage(Vec<u8>), // 'enim'
    Float16Array(Vec<half::f16>), // 'fl16'
    Float32Array(Vec<f32>), // 'fl32'
    Float64Array(Vec<f64>), // 'fl64'
    GamutBoundaryDescription(Vec<u8>), // 'gbd'
    Lut8(Lut8),
    LutAToB(Vec<u8>), // 'mAB'
    LutBToA(Vec<u8>), // 'mBA'
    Measurement(Measurement), // 'meas'
    MakeAndModel(MakeAndModel), // 'mmod'
    MultiLocalizedUnicode(MultiLocalizedUnicode), // 'mluc'
    MultiProcessElements(Vec<u8>), // 'mpet'
    NamedColor2(NamedColor2), // 'ncl2'
    ParametricCurve(ParametricCurve), // 'para'
    S15Fixed16Array(Vec<f32>), // 'sf32'
    Signature([u8;4]), // 'sig'
    SparseMatrixArray(Vec<u8>), // 'smat'
    SpectralViewingConditions(Vec<u8>), // 'svcn'
    TagStruct(Vec<u8>), // 'tstr'
    Technology(TechnologySignature), // tag derived type
    Text(String),
    TextDescription(TextDescription),
    U16Fixed16Array(Vec<f32>), // 'uf32'
    UInt8Array(Vec<u8>), // 'ui16'
    UInt16Array(Vec<u16>), // 'ui16'
    UInt32Array(Vec<u32>), // 'ui16'
    UInt64Array(Vec<u64>), // 'ui64'
    Utf8(Vec<String>), // 'utf8'
    Utf16(Vec<String>), // 'ut16'
    Utf8Zip(Vec<String>), // 'zut8'
    Vcgt(Vcgt), // 'vcgt'
    ViewingConditions(ViewingConditions),
    XYZ(XYZ), // 'XYZ'
    Custom(TagTypeSignature, Vec<u8>), // unknown data type
}



impl TagData {
    pub fn try_new(tag_signature: TagSignature, type_signature: TagTypeSignature, buf: &mut &[u8]) -> Result<Self> {
        match (tag_signature, type_signature) {
            (_, TagTypeSignature::ChromaticityType) => {
                Ok(Self::Chromaticity(Chromaticity::try_new(buf)?))
            },
            (_, TagTypeSignature::ColorantOrderType) => Ok(Self::ColorantOrder(ColorantOrder(buf.to_owned()))),
            (_, TagTypeSignature::CurveType) => {
                let n = read_be_u32(buf)? as usize;
                let mut v: Vec<u16> = Vec::with_capacity(n);
                for _ in 0..n {
                    v.push(read_be_u16(buf)?);
                }
                Ok(Self::Curve(Curve(v)))
            }
            (_, TagTypeSignature::DataType) => {
                let _n = read_be_u32(buf)? as usize;
                Ok(Self::Data(Data(buf.to_owned())))
            },
            (_, TagTypeSignature::DateTimeType) => {
                Ok(Self::DateTime(DateTime(read_date_time(buf)?.unwrap())))
            },
            (_, TagTypeSignature::Float16ArrayType)=> {
                let mut v = Vec::with_capacity(buf.len()/std::mem::size_of::<half::f16>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f16(buf)?)
                }
                Ok(Self::Float16Array(v))
            },
            (_, TagTypeSignature::Float32ArrayType) => {
                let mut v = Vec::with_capacity(buf.len()/std::mem::size_of::<f32>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f32(buf)?)
                }
                Ok(Self::Float32Array(v))
            },
            (_, TagTypeSignature::Float64ArrayType) => {
                let mut v = Vec::with_capacity(buf.len()/std::mem::size_of::<f64>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f64(buf)?)
                }
                Ok(Self::Float64Array(v))
            },
            (_, TagTypeSignature::Lut8Type) => {
                Ok(Self::Lut8(Lut8::try_new(buf)?))
            },
            (_, TagTypeSignature::MakeAndModelType) => {
                Ok(Self::MakeAndModel(MakeAndModel::try_new(buf)?))
            },
            (_, TagTypeSignature::MeasurementType) => {
                Ok(Self::Measurement(Measurement::try_new(buf)?))
            },
            (_, TagTypeSignature::MultiLocalizedUnicodeType) => {
                Ok(Self::MultiLocalizedUnicode(MultiLocalizedUnicode::try_new(buf)?))
            },
            (_, TagTypeSignature::NamedColor2Type) => {
                Ok(Self::NamedColor2(NamedColor2::try_new(buf, 3)?)) // TODO! pcs size???
            },
            (_, TagTypeSignature::ParametricCurveType) => {
                Ok(Self::ParametricCurve(ParametricCurve::try_new(buf)?))
            },
            (_, TagTypeSignature::S15Fixed16ArrayType) => {
                Ok(Self::S15Fixed16Array(read_s15fixed16_array(buf, None)?))
            },
            (_, TagTypeSignature::TextType) => {
                Ok(Self::Text(std::str::from_utf8(buf)?.trim_end_matches(char::from(0)).to_owned()))
            },
            (_, TagTypeSignature::TextDescriptionType) => {
                Ok(Self::TextDescription(TextDescription::try_new(buf)?))
            },
            (_, TagTypeSignature::ViewingConditionsType) => {
                Ok(Self::ViewingConditions(ViewingConditions::try_new(buf)?))
            },
            (_, TagTypeSignature::XYZArrayType) => {
                let n = buf.len()/12;
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    if let Some(xyz) = read_xyz(buf)? {
                        v.push(xyz);
                    } else {
                        v.push([0.0, 0.0, 0.0]);
                    }
                }
                Ok(Self::XYZ(XYZ(v)))

            },
            (TagSignature::VcgtTag, TagTypeSignature::VcgtType) => { 
                Ok(Self::Vcgt(Vcgt::try_new(buf)?))
            },
            (TagSignature::TechnologyTag, TagTypeSignature::SignatureType) => {
                Ok(Self::Technology(FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default()))
            },
            _  => Ok(Self::Custom(type_signature, buf.to_owned())),
        } 
    }
}



// Tag Type definitions
// Simple tag types defined here, complex tag types in separate files

use chromaticity::Chromaticity;

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

use lut8::Lut8;

use make_model::MakeAndModel;

use measurement::Measurement;

use multi_localized_unicode::MultiLocalizedUnicode;

use named_color2::NamedColor2;

use parametric_curve::ParametricCurve;
#[derive(Debug, Serialize)]
pub struct Text(String);

use text_description::TextDescription;

use vcgt::Vcgt;

use viewing_conditions::ViewingConditions;

#[derive(Debug, Serialize)]
pub struct XYZ(Vec<[f64;3]>);



