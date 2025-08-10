mod chromaticity;
mod common;
mod curve;
mod lut8;
mod make_model;
mod measurement;
mod multi_localized_unicode;
mod named_color2;
mod native_display_info;
mod parametric_curve;
mod raw;
mod text_description;
mod vcgp;
mod vcgt;
mod viewing_conditions;
mod xyz;
//mod tag_builders;

mod toml;
pub use toml::TagToml;
mod header_tags;

pub use header_tags::{GamutCheck, Interpolate, Quality, RenderingIntent, S15Fixed16};

use crate::signatures::{type_signature::TypeSignature, TagSignature};

use paste::paste;
use serde::Serialize;

pub trait TagTraits {
    /// Converts the tag data into a byte vector.
    fn into_bytes(self) -> Vec<u8>;
    fn as_slice(&self) -> &[u8];
    fn len(&self) -> usize {
        self.as_slice().len()
    }
    fn pad(&mut self, size: usize);
    fn type_signature(&self) -> TypeSignature {
        // Default implementation to return a slice of the bytes.
        let array: [u8; 4] = self.as_slice()[0..4].try_into().unwrap();
        array.into()
    }
}

/// Defines all tag-related structs and the main `Tag` enum from a single list of names.
///
/// For each `Name` provided (e.g., `Curve`, `XYZ`), this macro generates:
/// 1. A `pub struct NameType(pub Vec<u8>)` to wrap the raw tag data.
/// 2. Implementations of `TagTraits` and `Default` for `NameType`.
/// 3. A `Tag` enum with a `Name(NameType)` variant for each name.
/// 4. Implementations of `TagTraits` for the `Tag` enum.
/// 5. Helper methods on `Tag` like `.as_curve()` and `.as_curve_mut()`.
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
/// new variant to the `Tag` enum, and implements all necessary traits and helper
/// functions. This prevents common errors that arise from manually keeping multiple
/// lists and implementations in sync.
///
/// It leverages the `paste` crate to dynamically create identifiers, such as turning
/// the input `Curve` into the struct name `CurveType` and the helper method name
/// `as_curve`. The generated helper methods (`as_...` and `as_..._mut`) are particularly important,
/// as they provide a safe, ergonomic, and idiomatic Rust API for accessing the data
/// within a `Tag` enum variant.
macro_rules! define_tags_and_types {
    ($($name:ident),+ $(,)?) => {
        paste! {
            // --- Part 1: Generate the individual `...Type` structs and their traits ---
            // This is the logic from your old `define_tag_types!` macro.
            $(
                #[derive(Debug, Serialize, Clone, PartialEq)]
                pub struct [< $name Type >](pub Vec<u8>);

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

            // --- Part 2: Generate the main `Tag` enum ---
            // This is the logic from your old `enum_tags!` macro.
            /// This enum encapsulates the various tag types defined in the ICC specification.
            #[derive(Debug, Serialize, Clone, PartialEq)]
            pub enum Tag {
                $($name([< $name Type >])),+
            }

            // --- Part 3: Generate trait impls and helpers for the `Tag` enum ---
            impl TagTraits for Tag {
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

            impl Tag {
                $(
                    // Returns a reference to the inner struct if the variant matches.
                    //
                    // Example:
                    // ```
                    // // Construct a Curve tag and query it with the generated helper.
                    // let tag = crate::tags::Tag::Curve(crate::tags::CurveType(vec![]));
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
                    // let mut tag = crate::tags::Tag::Curve(crate::tags::CurveType(vec![1, 2, 3]));
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
    S15Fixed16Array,
    Signature,
    SparseMatrixArray,
    SpectralViewingConditions,
    TagStruct,
    Technology,
    Text,
    TextDescription,
    U16Fixed16Array,
    UInt8Array,
    UInt16Array,
    UInt32Array,
    UInt64Array,
    Utf8,
    Utf16,
    Utf8Zip,
    Vcgt,
    Vcgp,
    ViewingConditions,
    XYZ
);

/// Represents a single raw ICC tag, with its offset, size, and data as bytes.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct TagEntry {
    pub offset: u32,
    pub size: u32,
    pub tag: Tag,
}

impl TagEntry {
    /// Creates a new `TagEntry` with the given offset, size, and tag.
    pub fn new(offset: u32, size: u32, tag: Tag) -> Self {
        Self { offset, size, tag }
    }

    /// Returns the raw bytes of the tag.
    pub fn as_slice(&self) -> &[u8] {
        self.tag.as_slice()
    }

    /// Converts the tag into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        self.tag.into_bytes()
    }
}

impl Tag {
    /// Creates a new `Tag` from a `TagSignature` and its raw byte data.
    ///
    /// This function acts as a factory, lazily dispatching to the correct parsing logic based on the
    /// provided `signature`. It encapsulates the complexity of the ICC tag structure, including
    /// how to handle tags that can represent multiple data types.
    ///
    /// # Arguments
    ///
    /// * `signature` - The 4-byte tag signature from the profile's tag table (e.g., `'desc'`, `'wtpt'`).
    ///   This determines the *semantic meaning* of the tag.
    /// * `data` - A `Vec<u8>` containing the raw byte data for the tag. Crucially, this data block
    ///   itself begins with its own 4-byte *type signature* (e.g., `'text'`, `'curv'`).
    ///
    /// # Handling Ambiguous Tags
    ///
    /// Some tag signatures in the ICC specification are ambiguous. For example, the `'desc'` tag
    /// can point to data of type `textDescriptionType` or `multiLocalizedUnicodeType`.
    ///
    /// This function resolves such ambiguities by inspecting the first 4 bytes of the `data` payload,
    /// which contains the data's *type signature*.
    ///
    /// For example:
    /// - If `signature` is `'desc'` and `data` starts with `'desc'`, it creates a `Tag::TextDescription`.
    /// - If `signature` is `'desc'` and `data` starts with `'mluc'`, it creates a `Tag::MultiLocalizedUnicode`.
    ///
    /// # Fallback
    ///
    // If the `signature` is not recognized or a specific parsing rule is not implemented, this function
    /// will return a `Tag::Raw` variant, which safely wraps the original byte data without interpretation.
    ///
    /// # Panics
    ///
    /// This function may panic if the `data` slice is less than 4 bytes long, as it needs to read
    /// the type signature from the data payload.
    pub fn new(signature: TagSignature, data: Vec<u8>) -> Self {
        let type_signature = TypeSignature::from(<[u8; 4]>::try_from(&data[0..4]).unwrap());
        match signature {
            // non-ambiguous tags
            TagSignature::ChromaticityTag => Self::Chromaticity(ChromaticityType(data)),
            TagSignature::ColorantOrderTag => Self::ColorantOrder(ColorantOrderType(data)),
            TagSignature::DataTag => Self::Data(DataType(data)),
            TagSignature::DateTimeTag => Self::DateTime(DateTimeType(data)),
            TagSignature::MeasurementTag => Self::Measurement(MeasurementType(data)),
            TagSignature::MakeAndModelTag => Self::MakeAndModel(MakeAndModelType(data)),
            TagSignature::NativeDisplayInfoTag => {
                Self::NativeDisplayInfo(NativeDisplayInfoType(data))
            }
            TagSignature::NamedColor2Tag => Self::NamedColor2(NamedColor2Type(data)),
            TagSignature::SpectralViewingConditionsTag => {
                Self::SpectralViewingConditions(SpectralViewingConditionsType(data))
            }
            TagSignature::TechnologyTag => Self::Technology(TechnologyType(data)),
            TagSignature::VcgtTag => Self::Vcgt(VcgtType(data)),
            TagSignature::VcgpTag => Self::Vcgp(VcgpType(data)),
            TagSignature::ViewingConditionsTag => {
                Self::ViewingConditions(ViewingConditionsType(data))
            }

            // ambiguous tags

            TagSignature::ProfileDescriptionTag => match type_signature {
                TypeSignature::TextDescriptionType => {
                    Self::TextDescription(TextDescriptionType(data))
                }
                TypeSignature::MultiLocalizedUnicodeType => {
                    Self::MultiLocalizedUnicode(MultiLocalizedUnicodeType(data))
                }
                _ => Self::Raw(RawType(data)),
            }
            TagSignature::GreenTRCTag => match type_signature {
                TypeSignature::CurveType => Self::Curve(CurveType(data)),
                TypeSignature::ParametricCurveType => {
                    Self::ParametricCurve(ParametricCurveType(data))
                }
                _ => Self::Raw(RawType(data)),
            },
            TagSignature::BlueTRCTag => match type_signature {
                TypeSignature::CurveType => Self::Curve(CurveType(data)),
                TypeSignature::ParametricCurveType => {
                    Self::ParametricCurve(ParametricCurveType(data))
                }
                _ => Self::Raw(RawType(data)),
            },
            TagSignature::RedTRCTag => match type_signature {
                TypeSignature::CurveType => Self::Curve(CurveType(data)),
                TypeSignature::ParametricCurveType => {
                    Self::ParametricCurve(ParametricCurveType(data))
                }
                _ => Self::Raw(RawType(data)),
            },
            TagSignature::NamedColorTag => match type_signature {
                TypeSignature::NamedColor2Type => Self::NamedColor2(NamedColor2Type(data)),
                _ => Self::Raw(RawType(data)),
            },
            TagSignature::ProfileSequenceDescTag => match type_signature {
                TypeSignature::MultiLocalizedUnicodeType => {
                    Self::MultiLocalizedUnicode(MultiLocalizedUnicodeType(data))
                }
                TypeSignature::TextDescriptionType => {
                    Self::TextDescription(TextDescriptionType(data))
                }
                TypeSignature::TextType => Self::Text(TextType(data)),
                _ => Self::Raw(RawType(data)),
            },

            _ => Self::Raw(RawType(data)),
        }
    }
}


/// A trait for tag signatures that have only one valid data type.
pub trait UnambiguousTag {
    /// The single data type associated with this tag signature.
    type TagType: Default;

    /// A function to create the correct `Tag` enum variant from the Tag block data.
    fn new_tag(tag_type_instance: Self::TagType) -> Tag;
}

/// A helper macro to reduce boilerplate when implementing `UnambiguousTag`.
macro_rules! impl_unambiguous_tag {
    ($tag_type_name:ident, /*$data_type:ty,*/ $tag_variant:ident) => {
        paste! {
            // No paste needed. Just write the full path to the type inside the macro.
            // The compiler will correctly substitute the identifier at the end.
            // This also uses the correct ZST pattern (implementing on the type, not a reference).
            impl UnambiguousTag for crate::signatures::tag_signature::$tag_type_name {
                type TagType = [< $tag_variant Type >];
                fn new_tag(tag_type_instance: Self::TagType) -> Tag {
                    Tag::$tag_variant(tag_type_instance)
                }
            }
        }
    };
}

// Tags of type XYZType
impl_unambiguous_tag!(MediaWhitePointTag, XYZ);
impl_unambiguous_tag!(MediaBlackPointTag, XYZ);
impl_unambiguous_tag!(LuminanceTag, XYZ);

// Tags of type CurveType
impl_unambiguous_tag!(RedTRCTag, Curve);
impl_unambiguous_tag!(GreenTRCTag, Curve);
impl_unambiguous_tag!(BlueTRCTag, Curve);
impl_unambiguous_tag!(GrayTRCTag, Curve); // Assuming you have a GrayTRCTag ZST

// Tags of type TextDescriptionType
impl_unambiguous_tag!(CopyrightTag, TextDescription);
impl_unambiguous_tag!(DeviceMfgDescTag, TextDescription);
impl_unambiguous_tag!(DeviceModelDescTag, TextDescription);
impl_unambiguous_tag!(ScreeningDescTag, TextDescription);
impl_unambiguous_tag!(ViewingCondDescTag, TextDescription);

// Tags of type TextType
impl_unambiguous_tag!(CharTargetTag, Text);

// Tags of type SignatureType
impl_unambiguous_tag!(TechnologyTag, Signature);
impl_unambiguous_tag!(ColorimetricIntentImageStateTag,Signature); // Assuming ZST exists

// Chromaticity and Colorant Tags
impl_unambiguous_tag!(ChromaticityTag,Chromaticity);
impl_unambiguous_tag!(ColorantOrderTag, ColorantOrder);
impl_unambiguous_tag!(ColorantTableTag, ColorantTable);
impl_unambiguous_tag!(ColorantTableOutTag, ColorantTable); // Often same type as clrt
impl_unambiguous_tag!(NamedColor2Tag, NamedColor2);

// Metadata and Informational Tags
impl_unambiguous_tag!(CalibrationDateTimeTag, DateTime);
impl_unambiguous_tag!(ProfileSequenceDescTag, ProfileSequenceDesc);
impl_unambiguous_tag!(CrdInfoTag, CrdInfo);

// Measurement and Viewing Conditions Tags
impl_unambiguous_tag!(MeasurementTag, Measurement);
impl_unambiguous_tag!(ViewingConditionsTag, ViewingConditions);

// Video Color Gamut Tags (from VCGT spec)
impl_unambiguous_tag!(VcgtTag, Vcgt);
impl_unambiguous_tag!(VcgpTag, Vcgp);

//  --- Implementations for Ambiguous Tags ---

pub trait IsCurveTag {}
pub trait IsParametricCurveTag {}
pub trait IsTextDescriptionTag {}
pub trait IsMultiLocalizedUnicodeTag {}
pub trait IsLut8TypeTag {}
pub trait IsLut16TypeTag {}
pub trait IsLutAtoBTypeTag {}
pub trait IsLutBtoATypeTag {}

// Add other LUT type traits if you have them, e.g., IsGamutTypeTag

// 2. Import the ZSTs for all the ambiguous tags you will define.
use crate::signatures::tag_signature::{
    AToB0Tag, AToB1Tag, AToB2Tag, BToA0Tag, BToA1Tag, BToA2Tag, BlueTRCTag, DeviceMfgDescTag,
    DeviceModelDescTag, GamutTag, GreenTRCTag, GrayTRCTag, Preview0Tag, Preview1Tag, Preview2Tag,
    ProfileDescriptionTag, RedTRCTag,
};

// 3. Implement the traits for each ambiguous tag.

// The 'desc' tag can be either textDescriptionType or multiLocalizedUnicodeType.
impl IsTextDescriptionTag for ProfileDescriptionTag {}
impl IsMultiLocalizedUnicodeTag for ProfileDescriptionTag {}

// The device manufacturer and model tags are also often ambiguous in practice.
impl IsTextDescriptionTag for DeviceMfgDescTag {}
impl IsMultiLocalizedUnicodeTag for DeviceMfgDescTag {}
impl IsTextDescriptionTag for DeviceModelDescTag {}
impl IsMultiLocalizedUnicodeTag for DeviceModelDescTag {}

// The Tone Reproduction Curve (TRC) tags can be a simple curve or a parametric curve.
// NOTE: Remove these from your `impl_unambiguous_tag!` list.
impl IsCurveTag for RedTRCTag {}
impl IsParametricCurveTag for RedTRCTag {}
impl IsCurveTag for GreenTRCTag {}
impl IsParametricCurveTag for GreenTRCTag {}
impl IsCurveTag for BlueTRCTag {}
impl IsParametricCurveTag for BlueTRCTag {}
impl IsCurveTag for GrayTRCTag {}
impl IsParametricCurveTag for GrayTRCTag {}

// The main Look-Up Table (LUT) tags for rendering intents are ambiguous.
impl IsLut8TypeTag for AToB0Tag {}
impl IsLut16TypeTag for AToB0Tag {}
impl IsLutAtoBTypeTag for AToB0Tag {}

impl IsLut8TypeTag for AToB1Tag {}
impl IsLut16TypeTag for AToB1Tag {}
impl IsLutAtoBTypeTag for AToB1Tag {}

impl IsLut8TypeTag for AToB2Tag {}
impl IsLut16TypeTag for AToB2Tag {}
impl IsLutAtoBTypeTag for AToB2Tag {}

impl IsLut8TypeTag for BToA0Tag {}
impl IsLut16TypeTag for BToA0Tag {}
impl IsLutBtoATypeTag for BToA0Tag {}

impl IsLut8TypeTag for BToA1Tag {}
impl IsLut16TypeTag for BToA1Tag {}
impl IsLutBtoATypeTag for BToA1Tag {}

impl IsLut8TypeTag for BToA2Tag {}
impl IsLut16TypeTag for BToA2Tag {}
impl IsLutBtoATypeTag for BToA2Tag {}

// The gamut tag is also a LUT.
impl IsLut8TypeTag for GamutTag {}
impl IsLut16TypeTag for GamutTag {}
impl IsLutBtoATypeTag for GamutTag {}

// The preview tags are also LUTs.
impl IsLut8TypeTag for Preview0Tag {}
impl IsLut16TypeTag for Preview0Tag {}
impl IsLutAtoBTypeTag for Preview0Tag {}

impl IsLut8TypeTag for Preview1Tag {}
impl IsLut16TypeTag for Preview1Tag {}
impl IsLutAtoBTypeTag for Preview1Tag {}

impl IsLut8TypeTag for Preview2Tag {}
impl IsLut16TypeTag for Preview2Tag {}
impl IsLutAtoBTypeTag for Preview2Tag {}


impl ColorantOrderType {
    pub fn new(colorant_order: Vec<u8>) -> Self {
        Self(colorant_order)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
