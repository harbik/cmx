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

/// ICC Data Signatures, which are used to identify the type of data stored in a tag data block.
///
/// Each variant corresponds to a specific 4-byte type signature as defined in the ICC specification.
/// These signatures determine how the tag's data should be interpreted and parsed.
///
/// Unknown or vendor-specific type signatures are represented by the `Unknown(u32)` variant.
#[derive(PartialEq, Clone, Copy, Debug, Serialize)]
#[repr(u32)]
pub enum DataSignature {
    UndefinedData = 0x00000000,
    ChromaticityData = 0x6368726D,              /* 'chrm' */
    CicpData = 0x63696370,                      /* 'cicp' */
    ColorantOrderData = 0x636C726F,             /* 'clro' */
    ColorantTableData = 0x636C7274,             /* 'clrt' */
    CrdInfoData = 0x63726469,                   /* 'crdi' Removed in V4 */
    CurveData = 0x63757276,                     /* 'curv' */
    DataData = 0x64617461,                      /* 'data' */
    DictData = 0x64696374,                      /* 'dict' */
    DateTimeData = 0x6474696D,                  /* 'dtim' */
    DeviceSettingsData = 0x64657673,            /* 'devs' Removed in V4 */
    EmbeddedHeightImageData = 0x6568696D,       /* 'ehim' */
    EmbeddedNormalImageData = 0x656e696d,       /* 'enim' */
    Float16ArrayData = 0x666c3136,              /* 'fl16' */
    Float32ArrayData = 0x666c3332,              /* 'fl32' */
    Float64ArrayData = 0x666c3634,              /* 'fl64' */
    GamutBoundaryDescData = 0x67626420,         /* 'gbd ' */
    Lut16Data = 0x6d667432,                     /* 'mft2' */
    Lut8Data = 0x6d667431,                      /* 'mft1' */
    LutAtoBData = 0x6d414220,                   /* 'mAB ' */
    LutBtoAData = 0x6d424120,                   /* 'mBA ' */
    MeasurementData = 0x6D656173,               /* 'meas' */
    MakeAndModelData = 0x6d6d6f64,              /* 'mmod' Apple Make and Model */
    MultiLocalizedUnicodeData = 0x6D6C7563,     /* 'mluc' */
    MultiProcessElementData = 0x6D706574,       /* 'mpet' */
    NamedColor2Data = 0x6E636C32,               /* 'ncl2' use v2-v4*/
    NativeDisplayInfoData = 0x6e64696e,         /* 'ndin' Apple Private Signature*/
    ParametricCurveData = 0x70617261,           /* 'para' */
    ProfileSequenceDescData = 0x70736571,       /* 'pseq' */
    ProfileSequceIdData = 0x70736964,           /* 'psid' */
    ResponseCurveSet16Data = 0x72637332,        /* 'rcs2' */
    S15Fixed16ArrayData = 0x73663332,           /* 'sf32' */
    ScreeningData = 0x7363726E,                 /* 'scrn' Removed in V4 */
    SegmentedCurveData = 0x63757266,            /* 'curf' */
    SignatureData = 0x73696720,                 /* 'sig ' */
    SparseMatrixArrayData = 0x736D6174,         /* 'smat' */
    SpectralViewingConditionsData = 0x7376636e, /* 'svcn' */
    SpectralDataInfoData = 0x7364696e,          /* 'sdin' */
    TagArrayData = 0x74617279,                  /* 'tary' */
    TagStructData = 0x74737472,                 /* 'tstr' */
    TextData = 0x74657874,                      /* 'text' */
    TextDescriptionData = 0x64657363,           /* 'desc' Removed in V4 */
    U16Fixed16ArrayData = 0x75663332,           /* 'uf32' */
    UcrBgData = 0x62666420,                     /* 'bfd ' Removed in V4 */
    UInt16ArrayData = 0x75693136,               /* 'ui16' */
    UInt32ArrayData = 0x75693332,               /* 'ui32' */
    UInt64ArrayData = 0x75693634,               /* 'ui64' */
    UInt8ArrayData = 0x75693038,                /* 'ui08' */
    ViewingConditionsData = 0x76696577,         /* 'view' */
    VcgpData = 0x76636770, /* 'vcgp' not icc, defacto standard, used for tag and type */
    VcgtData = 0x76636774, /* 'vcgt' not icc, defacto standard, used for tag and type */
    Utf8TextData = 0x75746638, /* 'utf8' */
    Utf16TextData = 0x75743136, /* 'ut16' */
    /* XYZData                      = 0x58595A20, // 'XYZ ' Name changed to XYZArrayData */
    XYZArrayData = 0x58595A20,        /* 'XYZ ' */
    ZipUtf8TextData = 0x7a757438,     /* 'zut8' */
    ZipXmlData = 0x5a584d4c,          /* 'ZXML' */
    EmbeddedProfileData = 0x49434370, /* 'ICCp' */
    Unknown(u32),
}

impl DataSignature {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0x00000000 => Self::UndefinedData,
            0x6368726D => Self::ChromaticityData,
            0x636C726F => Self::ColorantOrderData,
            0x636C7274 => Self::ColorantTableData,
            0x63726469 => Self::CrdInfoData,
            0x63757276 => Self::CurveData,
            0x64617461 => Self::DataData,
            0x64696374 => Self::DictData,
            0x6474696D => Self::DateTimeData,
            0x64657673 => Self::DeviceSettingsData,
            0x6568696D => Self::EmbeddedHeightImageData,
            0x656E696D => Self::EmbeddedNormalImageData,
            0x666C3136 => Self::Float16ArrayData,
            0x666C3332 => Self::Float32ArrayData,
            0x666C3634 => Self::Float64ArrayData,
            0x67626420 => Self::GamutBoundaryDescData,
            0x6D667432 => Self::Lut16Data,
            0x6D667431 => Self::Lut8Data,
            0x6D414220 => Self::LutAtoBData,
            0x6D424120 => Self::LutBtoAData,
            0x6D656173 => Self::MeasurementData,
            0x6D6D6F64 => Self::MakeAndModelData,
            0x6D6C7563 => Self::MultiLocalizedUnicodeData,
            0x6D706574 => Self::MultiProcessElementData,
            0x6E636C32 => Self::NamedColor2Data,
            0x6E64696E => Self::NativeDisplayInfoData,
            0x70617261 => Self::ParametricCurveData,
            0x70736571 => Self::ProfileSequenceDescData,
            0x70736964 => Self::ProfileSequceIdData,
            0x72637332 => Self::ResponseCurveSet16Data,
            0x73663332 => Self::S15Fixed16ArrayData,
            0x7363726E => Self::ScreeningData,
            0x63757266 => Self::SegmentedCurveData,
            0x73696720 => Self::SignatureData,
            0x736D6174 => Self::SparseMatrixArrayData,
            0x7376636E => Self::SpectralViewingConditionsData,
            0x7364696E => Self::SpectralDataInfoData,
            0x74617279 => Self::TagArrayData,
            0x74737472 => Self::TagStructData,
            0x74657874 => Self::TextData,
            0x64657363 => Self::TextDescriptionData,
            0x75663332 => Self::U16Fixed16ArrayData,
            0x62666420 => Self::UcrBgData,
            0x75693136 => Self::UInt16ArrayData,
            0x75693332 => Self::UInt32ArrayData,
            0x75693634 => Self::UInt64ArrayData,
            0x75693038 => Self::UInt8ArrayData,
            0x76696577 => Self::ViewingConditionsData,
            0x76636770 => Self::VcgpData,
            0x76636774 => Self::VcgtData,
            0x75746638 => Self::Utf8TextData,
            0x75743136 => Self::Utf16TextData,
            0x58595A20 => Self::XYZArrayData,
            0x7A757438 => Self::ZipUtf8TextData,
            0x5A584D4C => Self::ZipXmlData,
            0x49434370 => Self::EmbeddedProfileData,
            _ => Self::Unknown(value),
        }
    }

    pub fn to_u32(self) -> u32 {
        match self {
            DataSignature::UndefinedData => 0x00000000,
            DataSignature::ChromaticityData => 0x6368726D,
            DataSignature::ColorantOrderData => 0x636C726F,
            DataSignature::ColorantTableData => 0x636C7274,
            DataSignature::CicpData => 0x63696370,
            DataSignature::CrdInfoData => 0x63726469,
            DataSignature::CurveData => 0x63757276,
            DataSignature::DataData => 0x64617461,
            DataSignature::DictData => 0x64696374,
            DataSignature::DateTimeData => 0x6474696D,
            DataSignature::DeviceSettingsData => 0x64657673,
            DataSignature::EmbeddedHeightImageData => 0x6568696D,
            DataSignature::EmbeddedNormalImageData => 0x656e696d,
            DataSignature::Float16ArrayData => 0x666c3136,
            DataSignature::Float32ArrayData => 0x666c3332,
            DataSignature::Float64ArrayData => 0x666c3634,
            DataSignature::GamutBoundaryDescData => 0x67626420,
            DataSignature::Lut16Data => 0x6d667432,
            DataSignature::Lut8Data => 0x6d667431,
            DataSignature::LutAtoBData => 0x6d414220,
            DataSignature::LutBtoAData => 0x6d424120,
            DataSignature::MeasurementData => 0x6D656173,
            DataSignature::MakeAndModelData => 0x6d6d6f64,
            DataSignature::MultiLocalizedUnicodeData => 0x6D6C7563,
            DataSignature::MultiProcessElementData => 0x6D706574,
            DataSignature::NamedColor2Data => 0x6E636C32,
            DataSignature::NativeDisplayInfoData => 0x6e64696e,
            DataSignature::ParametricCurveData => 0x70617261,
            DataSignature::ProfileSequenceDescData => 0x70736571,
            DataSignature::ProfileSequceIdData => 0x70736964,
            DataSignature::ResponseCurveSet16Data => 0x72637332,
            DataSignature::S15Fixed16ArrayData => 0x73663332,
            DataSignature::ScreeningData => 0x7363726E,
            DataSignature::SegmentedCurveData => 0x63757266,
            DataSignature::SignatureData => 0x73696720,
            DataSignature::SparseMatrixArrayData => 0x736D6174,
            DataSignature::SpectralViewingConditionsData => 0x7376636e,
            DataSignature::SpectralDataInfoData => 0x7364696e,
            DataSignature::TagArrayData => 0x74617279,
            DataSignature::TagStructData => 0x74737472,
            DataSignature::TextData => 0x74657874,
            DataSignature::TextDescriptionData => 0x64657363,
            DataSignature::U16Fixed16ArrayData => 0x75663332,
            DataSignature::UcrBgData => 0x62666420,
            DataSignature::UInt16ArrayData => 0x75693136,
            DataSignature::UInt32ArrayData => 0x75693332,
            DataSignature::UInt64ArrayData => 0x75693634,
            DataSignature::UInt8ArrayData => 0x75693038,
            DataSignature::ViewingConditionsData => 0x76696577,
            DataSignature::VcgpData => 0x76636770,
            DataSignature::VcgtData => 0x76636774,
            DataSignature::Utf8TextData => 0x75746638,
            DataSignature::Utf16TextData => 0x75743136,
            DataSignature::XYZArrayData => 0x58595A20,
            DataSignature::ZipUtf8TextData => 0x7a757438,
            DataSignature::ZipXmlData => 0x5a584d4c,
            DataSignature::EmbeddedProfileData => 0x49434370,
            DataSignature::Unknown(v) => v,
        }
    }
}

/// Converts the TagDataSignature to a string representation in hexadecimal format.
///
/// # Examples
/// ```
/// use std::str::FromStr;
/// use cmx::signatures::TagDataSignature;
/// let tag = TagDataSignature::from_str("6368726D").unwrap(); // 'chrm'
/// assert_eq!(tag.to_string(), "6368726D");
/// ```
impl std::fmt::Display for DataSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X}", self.to_u32())
    }
}
impl std::str::FromStr for DataSignature {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u32::from_str_radix(s, 16).map_err(|e| ParseError::new(e.to_string()))?;
        Ok(Self::from_u32(value))
    }
}

impl From<[u8; 4]> for DataSignature {
    fn from(bytes: [u8; 4]) -> Self {
        let value = u32::from_be_bytes(bytes);
        Self::from_u32(value)
    }
}

impl From<DataSignature> for [u8; 4] {
    fn from(signature: DataSignature) -> Self {
        let value = signature.to_u32();
        value.to_be_bytes()
    }
}

use paste::paste;

use super::TagDataTraits;

/// Defines all tag-related structs and the main `TagData` enum from a single list of names.
///
/// For each `Name` provided (e.g., `Curve`, `XYZ`), this macro generates:
/// 1. A `pub struct NameData(pub Vec<u8>)` to wrap the raw tag data.
/// 2. Implementations of `TagDataTraits` and `Default` for `NameData`.
/// 3. A `TagData` enum with a `Name(NameData)` variant for each name.
/// 4. Implementations of `TagDataTraits` for the `TagData` enum.
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
/// macro then automatically generates the corresponding `...Data` struct, adds the
/// new variant to the `TagData` enum, and implements all necessary traits and helper
/// functions. This prevents common errors that arise from manually keeping multiple
/// lists and implementations in sync.
///
/// It leverages the `paste` crate to dynamically create identifiers, such as turning
/// the input `Curve` into the struct name `CurveData` and the helper method name
/// `as_curve`. The generated helper methods (`as_...` and `as_..._mut`) are particularly important,
/// as they provide a safe, ergonomic, and idiomatic Rust API for accessing the data
/// within a `TagData` enum variant.
macro_rules! define_datatags {
    ($($name:ident),+ $(,)?) => {
        paste! {
            $(
                // Define the `TagData` types, which are, at this point, just wrappers
                // around `Vec<u8>`, the raw data for each tag, and the core of the
                // internal representation in this libray.
                // Examples of these are `CurveData`, `XYZData`, etc.
                #[derive(Debug, Serialize, Clone, PartialEq)]
                pub struct [< $name Data >](pub Vec<u8>);

                // Implement the `TagDataTraits` for each tag type.
                // As these are just wrappers around `Vec<u8>`, the implementation
                // is straightforward: we can convert them to bytes, slice them,
                // and pad them to a specific size.
                impl TagDataTraits for [< $name Data >] {
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

                impl Default for [< $name Data >] {
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
                $($name([< $name Data >])),+
            }

            // The `TagDataTraits` trait s also be implemented for the `TagData` enum,
            // implemented by direct dispatching to the appropriate variant's
            // implementation of `TagDataTraits`. This allows us to treat `TagData` as a
            // single type that can be converted to bytes, sliced, and padded,
            // regardless of which specific tag type it contains.
            impl TagDataTraits for TagData {
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
                    // let tag = crate::tags::TagData::Curve(crate::tags::CurveData(vec![]));
                    // assert!(tag.as_curve().is_some());
                    // // Querying for a different variant returns None.
                    // assert!(tag.as_xyz().is_none());
                    // ```
                    pub fn [< as_ $name:snake >](&self) -> Option<&[< $name Data >]> {
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
                    // let mut tag = crate::tags::TagData::Curve(crate::tags::CurveData(vec![1, 2, 3]));
                    //
                    // // Get a mutable reference and modify the data.
                    // if let Some(curve) = tag.as_curve_mut() {
                    //     curve.0.push(4);
                    // }
                    //
                    // // Verify the change.
                    // assert_eq!(tag.as_slice(), &[1, 2, 3, 4]);
                    // ```
                    pub fn [< as_ $name:snake _mut >](&mut self) -> Option<&mut [< $name Data >]> {
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
// It alo implements the `TagDataTraits` for each tag type, allowing them to be converted to bytes,
// sliced, and padded as needed. The length and type signature methods are also provided through
// the trait.
// Change to TagDatas
define_datatags!(
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
    ZipXmlData,
    Vcgt,
    Vcgp,
    ViewingConditions,
    XYZArray,
    EmbeddedProfile,
);

impl TagData {
    pub fn new(data: Vec<u8>) -> Self {
        let data_sig_bytes = data[0..4].try_into().unwrap_or([0; 4]);
        let data_signature = DataSignature::from(data_sig_bytes);

        match data_signature {
            DataSignature::CrdInfoData => Self::CrdInfo(CrdInfoData(data)),
            DataSignature::CicpData => Self::Cicp(CicpData(data)),
            DataSignature::ChromaticityData => Self::Chromaticity(ChromaticityData(data)),
            DataSignature::ColorantOrderData => Self::ColorantOrder(ColorantOrderData(data)),
            DataSignature::ColorantTableData => Self::ColorantTable(ColorantTableData(data)),
            DataSignature::CurveData => Self::Curve(CurveData(data)),
            DataSignature::DataData => Self::Data(DataData(data)),
            DataSignature::DateTimeData => Self::DateTime(DateTimeData(data)),
            DataSignature::DictData => Self::Dict(DictData(data)),
            DataSignature::EmbeddedHeightImageData => {
                Self::EmbeddedHeigthImage(EmbeddedHeigthImageData(data))
            }
            DataSignature::EmbeddedNormalImageData => {
                Self::EmbeddedNormalImage(EmbeddedNormalImageData(data))
            }
            DataSignature::Float16ArrayData => Self::Float16Array(Float16ArrayData(data)),
            DataSignature::Float32ArrayData => Self::Float32Array(Float32ArrayData(data)),
            DataSignature::Float64ArrayData => Self::Float64Array(Float64ArrayData(data)),
            DataSignature::GamutBoundaryDescData => {
                Self::GamutBoundaryDescription(GamutBoundaryDescriptionData(data))
            }
            DataSignature::Lut8Data => Self::Lut8(Lut8Data(data)),
            DataSignature::Lut16Data => Self::Lut16(Lut16Data(data)),
            DataSignature::LutAtoBData => Self::LutAToB(LutAToBData(data)),
            DataSignature::LutBtoAData => Self::LutBToA(LutBToAData(data)),
            DataSignature::MeasurementData => Self::Measurement(MeasurementData(data)),
            DataSignature::MakeAndModelData => Self::MakeAndModel(MakeAndModelData(data)),
            DataSignature::MultiLocalizedUnicodeData => {
                Self::MultiLocalizedUnicode(MultiLocalizedUnicodeData(data))
            }
            DataSignature::NativeDisplayInfoData => {
                Self::NativeDisplayInfo(NativeDisplayInfoData(data))
            }
            DataSignature::NamedColor2Data => Self::NamedColor2(NamedColor2Data(data)),
            DataSignature::ParametricCurveData => Self::ParametricCurve(ParametricCurveData(data)),
            DataSignature::ProfileSequenceDescData => {
                Self::ProfileSequenceDesc(ProfileSequenceDescData(data))
            }
            DataSignature::ProfileSequceIdData => {
                Self::ProfileSequenceId(ProfileSequenceIdData(data))
            }
            DataSignature::S15Fixed16ArrayData => Self::S15Fixed16Array(S15Fixed16ArrayData(data)),
            DataSignature::SignatureData => Self::Signature(SignatureData(data)),
            DataSignature::SparseMatrixArrayData => {
                Self::SparseMatrixArray(SparseMatrixArrayData(data))
            }
            DataSignature::SpectralViewingConditionsData => {
                Self::SpectralViewingConditions(SpectralViewingConditionsData(data))
            }
            DataSignature::TagStructData => Self::TagStruct(TagStructData(data)),
            DataSignature::TextData => Self::Text(TextData(data)),
            DataSignature::TextDescriptionData => Self::TextDescription(TextDescriptionData(data)),
            DataSignature::U16Fixed16ArrayData => Self::U16Fixed16Array(U16Fixed16ArrayData(data)),
            DataSignature::UInt8ArrayData => Self::UInt8Array(UInt8ArrayData(data)),
            DataSignature::UInt16ArrayData => Self::UInt16Array(UInt16ArrayData(data)),
            DataSignature::UInt32ArrayData => Self::UInt32Array(UInt32ArrayData(data)),
            DataSignature::UInt64ArrayData => Self::UInt64Array(UInt64ArrayData(data)),
            DataSignature::Utf8TextData => Self::Utf8Text(Utf8TextData(data)),
            DataSignature::Utf16TextData => Self::Utf16Text(Utf16TextData(data)),
            DataSignature::ZipUtf8TextData => Self::ZipUtf8Text(ZipUtf8TextData(data)),
            DataSignature::VcgtData => Self::Vcgt(VcgtData(data)),
            DataSignature::VcgpData => Self::Vcgp(VcgpData(data)),
            DataSignature::ViewingConditionsData => {
                Self::ViewingConditions(ViewingConditionsData(data))
            }
            DataSignature::XYZArrayData => Self::XYZArray(XYZArrayData(data)),
            // If the type signature is not recognized, we return a Raw tag.
            _ => Self::Raw(RawData(data)),
        }
    }
}

impl ColorantOrderData {
    pub fn new(colorant_order: Vec<u8>) -> Self {
        Self(colorant_order)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
