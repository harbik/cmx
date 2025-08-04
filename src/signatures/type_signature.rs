//! ICC Type Signatures
//!
//! This module defines the `TypeSignature` enum, which represents the 4-byte type signatures
//! used in ICC profiles to identify the type of data stored in a tag data block.
//!
//! Each variant corresponds to a specific ICC type signature, as defined in the ICC specification.
//! These signatures are used to interpret the structure and meaning of the tag's data.
//!
//! Unknown or vendor-specific type signatures are represented by the `Unknown(u32)` variant.

use serde::Serialize;

/// ICC Type Signatures, which are used to identify the type of data stored in a tag data block.
///
/// Each variant corresponds to a specific 4-byte type signature as defined in the ICC specification.
/// These signatures determine how the tag's data should be interpreted and parsed.
///
/// Unknown or vendor-specific type signatures are represented by the `Unknown(u32)` variant.
#[derive(PartialEq, Clone, Copy, Debug, Serialize)]
#[repr(u32)]
pub enum TypeSignature {
    UndefinedType = 0x00000000,
    ChromaticityType = 0x6368726D,              /* 'chrm' */
    ColorantOrderType = 0x636C726F,             /* 'clro' */
    ColorantTableType = 0x636C7274,             /* 'clrt' */
    CrdInfoType = 0x63726469,                   /* 'crdi' Removed in V4 */
    CurveType = 0x63757276,                     /* 'curv' */
    DataType = 0x64617461,                      /* 'data' */
    DictType = 0x64696374,                      /* 'dict' */
    DateTimeType = 0x6474696D,                  /* 'dtim' */
    DeviceSettingsType = 0x64657673,            /* 'devs' Removed in V4 */
    EmbeddedHeightImageType = 0x6568696D,       /* 'ehim' */
    EmbeddedNormalImageType = 0x656e696d,       /* 'enim' */
    Float16ArrayType = 0x666c3136,              /* 'fl16' */
    Float32ArrayType = 0x666c3332,              /* 'fl32' */
    Float64ArrayType = 0x666c3634,              /* 'fl64' */
    GamutBoundaryDescType = 0x67626420,         /* 'gbd ' */
    Lut16Type = 0x6d667432,                     /* 'mft2' */
    Lut8Type = 0x6d667431,                      /* 'mft1' */
    LutAtoBType = 0x6d414220,                   /* 'mAB ' */
    LutBtoAType = 0x6d424120,                   /* 'mBA ' */
    MeasurementType = 0x6D656173,               /* 'meas' */
    MakeAndModelType = 0x6d6d6f64,              /* 'mmod' Apple Make and Model */
    MultiLocalizedUnicodeType = 0x6D6C7563,     /* 'mluc' */
    MultiProcessElementType = 0x6D706574,       /* 'mpet' */
    NamedColor2Type = 0x6E636C32,               /* 'ncl2' use v2-v4*/
    NativeDisplayInfoType = 0x6e64696e,         /* 'ndin' Apple Private Signature*/
    ParametricCurveType = 0x70617261,           /* 'para' */
    ProfileSequenceDescType = 0x70736571,       /* 'pseq' */
    ProfileSequceIdType = 0x70736964,           /* 'psid' */
    ResponseCurveSet16Type = 0x72637332,        /* 'rcs2' */
    S15Fixed16ArrayType = 0x73663332,           /* 'sf32' */
    ScreeningType = 0x7363726E,                 /* 'scrn' Removed in V4 */
    SegmentedCurveType = 0x63757266,            /* 'curf' */
    SignatureType = 0x73696720,                 /* 'sig ' */
    SparseMatrixArrayType = 0x736D6174,         /* 'smat' */
    SpectralViewingConditionsType = 0x7376636e, /* 'svcn' */
    SpectralDataInfoType = 0x7364696e,          /* 'sdin' */
    TagArrayType = 0x74617279,                  /* 'tary' */
    TagStructType = 0x74737472,                 /* 'tstr' */
    TextType = 0x74657874,                      /* 'text' */
    TextDescriptionType = 0x64657363,           /* 'desc' Removed in V4 */
    U16Fixed16ArrayType = 0x75663332,           /* 'uf32' */
    UcrBgType = 0x62666420,                     /* 'bfd ' Removed in V4 */
    UInt16ArrayType = 0x75693136,               /* 'ui16' */
    UInt32ArrayType = 0x75693332,               /* 'ui32' */
    UInt64ArrayType = 0x75693634,               /* 'ui64' */
    UInt8ArrayType = 0x75693038,                /* 'ui08' */
    ViewingConditionsType = 0x76696577,         /* 'view' */
    VcgpType = 0x76636770, /* 'vcgp' not icc, defacto standard, used for tag and type */
    VcgtType = 0x76636774, /* 'vcgt' not icc, defacto standard, used for tag and type */
    Utf8TextType = 0x75746638, /* 'utf8' */
    Utf16TextType = 0x75743136, /* 'ut16' */
    /* XYZType                      = 0x58595A20, // 'XYZ ' Name changed to XYZArrayType */
    XYZArrayType = 0x58595A20,        /* 'XYZ ' */
    ZipUtf8TextType = 0x7a757438,     /* 'zut8' */
    ZipXmlType = 0x5a584d4c,          /* 'ZXML' */
    EmbeddedProfileType = 0x49434370, /* 'ICCp' */
    Unknown(u32)
}

impl TypeSignature {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0x00000000 => Self::UndefinedType,
            0x6368726D => Self::ChromaticityType,
            0x636C726F => Self::ColorantOrderType,
            0x636C7274 => Self::ColorantTableType,
            0x63726469 => Self::CrdInfoType,
            0x63757276 => Self::CurveType,
            0x64617461 => Self::DataType,
            0x64696374 => Self::DictType,
            0x6474696D => Self::DateTimeType,
            0x64657673 => Self::DeviceSettingsType,
            0x6568696D => Self::EmbeddedHeightImageType,
            0x656E696D => Self::EmbeddedNormalImageType,
            0x666C3136 => Self::Float16ArrayType,
            0x666C3332 => Self::Float32ArrayType,
            0x666C3634 => Self::Float64ArrayType,
            0x67626420 => Self::GamutBoundaryDescType,
            0x6D667432 => Self::Lut16Type,
            0x6D667431 => Self::Lut8Type,
            0x6D414220 => Self::LutAtoBType,
            0x6D424120 => Self::LutBtoAType,
            0x6D656173 => Self::MeasurementType,
            0x6D6D6F64 => Self::MakeAndModelType,
            0x6D6C7563 => Self::MultiLocalizedUnicodeType,
            0x6D706574 => Self::MultiProcessElementType,
            0x6E636C32 => Self::NamedColor2Type,
            0x6E64696E => Self::NativeDisplayInfoType,
            0x70617261 => Self::ParametricCurveType,
            0x70736571 => Self::ProfileSequenceDescType,
            0x70736964 => Self::ProfileSequceIdType,
            0x72637332 => Self::ResponseCurveSet16Type,
            0x73663332 => Self::S15Fixed16ArrayType,
            0x7363726E => Self::ScreeningType,
            0x63757266 => Self::SegmentedCurveType,
            0x73696720 => Self::SignatureType,
            0x736D6174 => Self::SparseMatrixArrayType,
            0x7376636E => Self::SpectralViewingConditionsType,
            0x7364696E => Self::SpectralDataInfoType,
            0x74617279 => Self::TagArrayType,
            0x74737472 => Self::TagStructType,
            0x74657874 => Self::TextType,
            0x64657363 => Self::TextDescriptionType,
            0x75663332 => Self::U16Fixed16ArrayType,
            0x62666420 => Self::UcrBgType,
            0x75693136 => Self::UInt16ArrayType,
            0x75693332 => Self::UInt32ArrayType,
            0x75693634 => Self::UInt64ArrayType,
            0x75693038 => Self::UInt8ArrayType,
            0x76696577 => Self::ViewingConditionsType,
            0x76636770 => Self::VcgpType,
            0x76636774 => Self::VcgtType,
            0x75746638 => Self::Utf8TextType,
            0x75743136 => Self::Utf16TextType,
            0x58595A20 => Self::XYZArrayType,
            0x7A757438 => Self::ZipUtf8TextType,
            0x5A584D4C => Self::ZipXmlType,
            0x49434370 => Self::EmbeddedProfileType,
            _ => Self::Unknown(value),
        }
    }

    pub fn to_u32(self) -> u32 {
        match self {
            TypeSignature::UndefinedType => 0x00000000,
            TypeSignature::ChromaticityType => 0x6368726D,
            TypeSignature::ColorantOrderType => 0x636C726F,
            TypeSignature::ColorantTableType => 0x636C7274,
            TypeSignature::CrdInfoType => 0x63726469,
            TypeSignature::CurveType => 0x63757276,
            TypeSignature::DataType => 0x64617461,
            TypeSignature::DictType => 0x64696374,
            TypeSignature::DateTimeType => 0x6474696D,
            TypeSignature::DeviceSettingsType => 0x64657673,
            TypeSignature::EmbeddedHeightImageType => 0x6568696D,
            TypeSignature::EmbeddedNormalImageType => 0x656e696d,
            TypeSignature::Float16ArrayType => 0x666c3136,
            TypeSignature::Float32ArrayType => 0x666c3332,
            TypeSignature::Float64ArrayType => 0x666c3634,
            TypeSignature::GamutBoundaryDescType => 0x67626420,
            TypeSignature::Lut16Type => 0x6d667432,
            TypeSignature::Lut8Type => 0x6d667431,
            TypeSignature::LutAtoBType => 0x6d414220,
            TypeSignature::LutBtoAType => 0x6d424120,
            TypeSignature::MeasurementType => 0x6D656173,
            TypeSignature::MakeAndModelType => 0x6d6d6f64,
            TypeSignature::MultiLocalizedUnicodeType => 0x6D6C7563,
            TypeSignature::MultiProcessElementType => 0x6D706574,
            TypeSignature::NamedColor2Type => 0x6E636C32,
            TypeSignature::NativeDisplayInfoType => 0x6e64696e,
            TypeSignature::ParametricCurveType => 0x70617261,
            TypeSignature::ProfileSequenceDescType => 0x70736571,
            TypeSignature::ProfileSequceIdType => 0x70736964,
            TypeSignature::ResponseCurveSet16Type => 0x72637332,
            TypeSignature::S15Fixed16ArrayType => 0x73663332,
            TypeSignature::ScreeningType => 0x7363726E,
            TypeSignature::SegmentedCurveType => 0x63757266,
            TypeSignature::SignatureType => 0x73696720,
            TypeSignature::SparseMatrixArrayType => 0x736D6174,
            TypeSignature::SpectralViewingConditionsType => 0x7376636e,
            TypeSignature::SpectralDataInfoType => 0x7364696e,
            TypeSignature::TagArrayType => 0x74617279,
            TypeSignature::TagStructType => 0x74737472,
            TypeSignature::TextType => 0x74657874,
            TypeSignature::TextDescriptionType => 0x64657363,
            TypeSignature::U16Fixed16ArrayType => 0x75663332,
            TypeSignature::UcrBgType => 0x62666420,
            TypeSignature::UInt16ArrayType => 0x75693136,
            TypeSignature::UInt32ArrayType => 0x75693332,
            TypeSignature::UInt64ArrayType => 0x75693634,
            TypeSignature::UInt8ArrayType => 0x75693038,
            TypeSignature::ViewingConditionsType => 0x76696577,
            TypeSignature::VcgpType => 0x76636770,
            TypeSignature::VcgtType => 0x76636774,
            TypeSignature::Utf8TextType => 0x75746638,
            TypeSignature::Utf16TextType => 0x75743136,
            TypeSignature::XYZArrayType => 0x58595A20,
            TypeSignature::ZipUtf8TextType => 0x7a757438,
            TypeSignature::ZipXmlType => 0x5a584d4c,
            TypeSignature::EmbeddedProfileType => 0x49434370,
            TypeSignature::Unknown(v) => v,
        }
    }
}

/// Converts the TagTypeSignature to a string representation in hexadecimal format.
///
/// # Examples
/// ```
/// use std::str::FromStr;
/// use cmx::signatures::TagTypeSignature;
/// let tag = TagTypeSignature::from_str("6368726D").unwrap(); // 'chrm'
/// assert_eq!(tag.to_string(), "6368726D");
/// ```
impl std::fmt::Display for TypeSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X}", self.to_u32())
    }
}
impl std::str::FromStr for TypeSignature {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u32::from_str_radix(s, 16).map_err(|e| crate::error::Error::ParseError(e.to_string()))?;
        Ok(Self::from_u32(value))
    }
}

impl From<[u8; 4]> for TypeSignature {
    fn from(bytes: [u8; 4]) -> Self {
        let value = u32::from_be_bytes(bytes);
        Self::from_u32(value)
    }
}