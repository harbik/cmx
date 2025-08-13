// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

pub mod chromaticity;
pub mod curve;
pub mod lut8;
pub mod make_model;
pub mod measurement;
pub mod multi_localized_unicode;
pub mod named_color2;
pub mod native_display_info;
pub mod parametric_curve;
pub mod raw;
pub mod s15fixed16array;
pub mod text;
pub mod text_description;
pub mod vcgp;
pub mod vcgt;
pub mod viewing_conditions;
pub mod xyz;

use serde::Serialize;

use crate::error::ParseError;

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
    CicpType = 0x63696370,                      /* 'cicp' */
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
    Unknown(u32),
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
            TypeSignature::CicpType => 0x63696370,
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
        let value = u32::from_str_radix(s, 16).map_err(|e| ParseError::new(e.to_string()))?;
        Ok(Self::from_u32(value))
    }
}

impl From<[u8; 4]> for TypeSignature {
    fn from(bytes: [u8; 4]) -> Self {
        let value = u32::from_be_bytes(bytes);
        Self::from_u32(value)
    }
}

impl From<TypeSignature> for [u8; 4] {
    fn from(signature: TypeSignature) -> Self {
        let value = signature.to_u32();
        value.to_be_bytes()
    }
}

use paste::paste;

use super::TagTraits;

/// Defines all tag-related structs and the main `TagData` enum from a single list of names.
///
/// For each `Name` provided (e.g., `Curve`, `XYZ`), this macro generates:
/// 1. A `pub struct NameType(pub Vec<u8>)` to wrap the raw tag data.
/// 2. Implementations of `TagTraits` and `Default` for `NameType`.
/// 3. A `TagData` enum with a `Name(NameType)` variant for each name.
/// 4. Implementations of `TagTraits` for the `TagData` enum.
/// 5. Helper methods on `TagData` like `.as_curve()` and `.as_curve_mut()`.
///
/// # Macro-Driven Design
///
/// This macro is the cornerstone of the tag system, designed to eliminate a significant
/// amount of repetitive boilerplate code. By defining a single, authoritative list of
/// tag type names, it ensures consistency across the entire module.
///
/// The core benefit is maintainability. To add a new tag type to the system, one only
/// needs to add its name to the list in the `define_tags_and_types!` invocation. The
/// macro then automatically generates the corresponding `...Type` struct, adds the
/// new variant to the `TagData` enum, and implements all necessary traits and helper
/// functions. This prevents common errors that arise from manually keeping multiple
/// lists and implementations in sync.
///
/// It leverages the `paste` crate to dynamically create identifiers, such as turning
/// the input `Curve` into the struct name `CurveType` and the helper method name
/// `as_curve`. The generated helper methods (`as_...` and `as_..._mut`) are particularly important,
/// as they provide a safe, ergonomic, and idiomatic Rust API for accessing the data
/// within a `TagData` enum variant.
macro_rules! define_tags_and_types {
    ($($name:ident),+ $(,)?) => {
        paste! {
            $(
                // Define the `TagData` types, which are, at this point, just wrappers
                // around `Vec<u8>`, the raw data for each tag, and the core of the
                // internal representation in this libray.
                // Examples of these are `CurveType`, `XYZType`, etc.
                #[derive(Debug, Serialize, Clone, PartialEq)]
                pub struct [< $name Type >](pub Vec<u8>);

                // Implement the `TagTraits` for each tag type.
                // As these are just wrappers around `Vec<u8>`, the implementation
                // is straightforward: we can convert them to bytes, slice them,
                // and pad them to a specific size.
                impl TagTraits for [< $name Type >] {
                    fn into_bytes(self) -> Vec<u8> {
                        self.0
                    }
                    fn as_slice(&self) -> &[u8] {
                        &self.0
                    }
                    fn pad(&mut self, size: usize) {
                        if self.0.len() < size {
                            self.0.resize(size, 0);
                        }
                    }
                }

                impl Default for [< $name Type >] {
                    fn default() -> Self {
                       Self(Vec::new())
                    }
                }
            )+

            // All the tag types are now defined, we can create the main `TagData` enum
            // which will encapsulate all the tag types defined above.
            // This enum will have a variant for each tag type, allowing us to
            // represent any tag in the ICC profile as a single type.
            #[derive(Debug, Serialize, Clone, PartialEq)]
            pub enum TagData {
                $($name([< $name Type >])),+
            }

            // The `TagTraits` trait s also be implemented for the `TagData` enum,
            // implemented by direct dispatching to the appropriate variant's
            // implementation of `TagTraits`. This allows us to treat `TagData` as a
            // single type that can be converted to bytes, sliced, and padded,
            // regardless of which specific tag type it contains.
            impl TagTraits for TagData {
                fn into_bytes(self) -> Vec<u8> {
                    match self {
                        $(Self::$name(t) => t.into_bytes()),+
                    }
                }
                fn as_slice(&self) -> &[u8] {
                    match self {
                        $(Self::$name(t) => t.as_slice()),+
                    }
                }
                fn pad(&mut self, size: usize) {
                    match self {
                        $(Self::$name(t) => t.pad(size)),+
                    }
                }
            }

            impl TagData {
                $(
                    // Returns a reference to the inner struct if the variant matches.
                    //
                    // Example:
                    // ```
                    // // Construct a Curve tag and query it with the generated helper.
                    // let tag = crate::tags::TagData::Curve(crate::tags::CurveType(vec![]));
                    // assert!(tag.as_curve().is_some());
                    // // Querying for a different variant returns None.
                    // assert!(tag.as_xyz().is_none());
                    // ```
                    pub fn [< as_ $name:snake >](&self) -> Option<&[< $name Type >]> {
                        if let Self::$name(v) = self {
                            Some(v)
                        } else {
                            None
                        }
                    }

                    // Returns a mutable reference to the inner struct if the variant matches.
                    //
                    // Example (for mutable access):
                    // ```
                    // // Construct a Curve tag.
                    // let mut tag = crate::tags::TagData::Curve(crate::tags::CurveType(vec![1, 2, 3]));
                    //
                    // // Get a mutable reference and modify the data.
                    // if let Some(curve) = tag.as_curve_mut() {
                    //     curve.0.push(4);
                    // }
                    //
                    // // Verify the change.
                    // assert_eq!(tag.as_slice(), &[1, 2, 3, 4]);
                    // ```
                    pub fn [< as_ $name:snake _mut >](&mut self) -> Option<&mut [< $name Type >]> {
                        if let Self::$name(v) = self {
                            Some(v)
                        } else {
                            None
                        }
                    }
                )+
            }
        }
    };
}

// This defines all the tag types, as wrappers around `Vec<u8>`, the raw data for each tag.
// It alo implements the `TagTraits` for each tag type, allowing them to be converted to bytes,
// sliced, and padded as needed. The length and type signature methods are also provided through
// the trait.
// Change to TagTypes
define_tags_and_types!(
    Raw,
    CrdInfo,
    Cicp,
    Chromaticity,
    ColorantOrder,
    ColorantTable,
    Curve,
    Data,
    DateTime,
    Dict,
    EmbeddedHeigthImage,
    EmbeddedNormalImage,
    Float16Array,
    Float32Array,
    Float64Array,
    GamutBoundaryDescription,
    Lut8,
    Lut16,
    LutAToB,
    LutBToA,
    Measurement,
    MakeAndModel,
    MultiLocalizedUnicode,
    MultiProcessElements,
    NativeDisplayInfo,
    NamedColor2,
    ParametricCurve,
    ProfileSequenceDesc,
    ProfileSequenceId,
    S15Fixed16Array,
    Signature,
    SparseMatrixArray,
    SpectralViewingConditions,
    TagStruct,
    //   Technology,
    Text,
    TextDescription,
    U16Fixed16Array,
    UInt8Array,
    UInt16Array,
    UInt32Array,
    UInt64Array,
    Utf8Text,
    Utf16Text,
    ZipUtf8Text,
    ZipXmlType,
    Vcgt,
    Vcgp,
    ViewingConditions,
    XYZArray,
    EmbeddedProfile,
);

impl TagData {
    pub fn new(data: Vec<u8>) -> Self {
        let type_sig_bytes = data[0..4].try_into().unwrap_or([0; 4]);
        let type_signature = TypeSignature::from(type_sig_bytes);

        match type_signature {
            TypeSignature::CrdInfoType => Self::CrdInfo(CrdInfoType(data)),
            TypeSignature::CicpType => Self::Cicp(CicpType(data)),
            TypeSignature::ChromaticityType => Self::Chromaticity(ChromaticityType(data)),
            TypeSignature::ColorantOrderType => Self::ColorantOrder(ColorantOrderType(data)),
            TypeSignature::ColorantTableType => Self::ColorantTable(ColorantTableType(data)),
            TypeSignature::CurveType => Self::Curve(CurveType(data)),
            TypeSignature::DataType => Self::Data(DataType(data)),
            TypeSignature::DateTimeType => Self::DateTime(DateTimeType(data)),
            TypeSignature::DictType => Self::Dict(DictType(data)),
            TypeSignature::EmbeddedHeightImageType => {
                Self::EmbeddedHeigthImage(EmbeddedHeigthImageType(data))
            }
            TypeSignature::EmbeddedNormalImageType => {
                Self::EmbeddedNormalImage(EmbeddedNormalImageType(data))
            }
            TypeSignature::Float16ArrayType => Self::Float16Array(Float16ArrayType(data)),
            TypeSignature::Float32ArrayType => Self::Float32Array(Float32ArrayType(data)),
            TypeSignature::Float64ArrayType => Self::Float64Array(Float64ArrayType(data)),
            TypeSignature::GamutBoundaryDescType => {
                Self::GamutBoundaryDescription(GamutBoundaryDescriptionType(data))
            }
            TypeSignature::Lut8Type => Self::Lut8(Lut8Type(data)),
            TypeSignature::Lut16Type => Self::Lut16(Lut16Type(data)),
            TypeSignature::LutAtoBType => Self::LutAToB(LutAToBType(data)),
            TypeSignature::LutBtoAType => Self::LutBToA(LutBToAType(data)),
            TypeSignature::MeasurementType => Self::Measurement(MeasurementType(data)),
            TypeSignature::MakeAndModelType => Self::MakeAndModel(MakeAndModelType(data)),
            TypeSignature::MultiLocalizedUnicodeType => {
                Self::MultiLocalizedUnicode(MultiLocalizedUnicodeType(data))
            }
            TypeSignature::NativeDisplayInfoType => {
                Self::NativeDisplayInfo(NativeDisplayInfoType(data))
            }
            TypeSignature::NamedColor2Type => Self::NamedColor2(NamedColor2Type(data)),
            TypeSignature::ParametricCurveType => Self::ParametricCurve(ParametricCurveType(data)),
            TypeSignature::ProfileSequenceDescType => {
                Self::ProfileSequenceDesc(ProfileSequenceDescType(data))
            }
            TypeSignature::ProfileSequceIdType => {
                Self::ProfileSequenceId(ProfileSequenceIdType(data))
            }
            TypeSignature::S15Fixed16ArrayType => Self::S15Fixed16Array(S15Fixed16ArrayType(data)),
            TypeSignature::SignatureType => Self::Signature(SignatureType(data)),
            TypeSignature::SparseMatrixArrayType => {
                Self::SparseMatrixArray(SparseMatrixArrayType(data))
            }
            TypeSignature::SpectralViewingConditionsType => {
                Self::SpectralViewingConditions(SpectralViewingConditionsType(data))
            }
            TypeSignature::TagStructType => Self::TagStruct(TagStructType(data)),
            TypeSignature::TextType => Self::Text(TextType(data)),
            TypeSignature::TextDescriptionType => Self::TextDescription(TextDescriptionType(data)),
            TypeSignature::U16Fixed16ArrayType => Self::U16Fixed16Array(U16Fixed16ArrayType(data)),
            TypeSignature::UInt8ArrayType => Self::UInt8Array(UInt8ArrayType(data)),
            TypeSignature::UInt16ArrayType => Self::UInt16Array(UInt16ArrayType(data)),
            TypeSignature::UInt32ArrayType => Self::UInt32Array(UInt32ArrayType(data)),
            TypeSignature::UInt64ArrayType => Self::UInt64Array(UInt64ArrayType(data)),
            TypeSignature::Utf8TextType => Self::Utf8Text(Utf8TextType(data)),
            TypeSignature::Utf16TextType => Self::Utf16Text(Utf16TextType(data)),
            TypeSignature::ZipUtf8TextType => Self::ZipUtf8Text(ZipUtf8TextType(data)),
            TypeSignature::VcgtType => Self::Vcgt(VcgtType(data)),
            TypeSignature::VcgpType => Self::Vcgp(VcgpType(data)),
            TypeSignature::ViewingConditionsType => {
                Self::ViewingConditions(ViewingConditionsType(data))
            }
            TypeSignature::XYZArrayType => Self::XYZArray(XYZArrayType(data)),
            // If the type signature is not recognized, we return a Raw tag.
            _ => Self::Raw(RawType(data)),
        }
    }
}

impl ColorantOrderType {
    pub fn new(colorant_order: Vec<u8>) -> Self {
        Self(colorant_order)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
