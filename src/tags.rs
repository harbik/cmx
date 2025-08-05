mod chromaticity;
mod common;
mod curve_type;
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
//mod tag_builders;

mod header_tags;

pub use header_tags::{GamutCheck, Interpolate, Quality, RenderingIntent, S15Fixed16};
use zerocopy::{BigEndian, IntoBytes, U16};

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

// This macro defines low-level wrapper types for ICC tag data, one for each tag type passed in.
// Each type is a simple `struct` that wraps the raw bytes (`Vec<u8>`) representing the tag’s payload,
// and implements a common interface (`TagTraits`) for working with the tag in a uniform way.
//
// These wrapper structs are intentionally minimal and efficient: they do not parse or interpret
// the contents of the byte buffer, but provide direct access to it for reading, writing, or deferred parsing.
//
// This macro accepts a list of identifiers (e.g., ChromaticityType, CurveType) and generates:
//   - A `pub struct Name(pub Vec<u8>)` for each identifier
//   - An implementation of the `TagTraits` trait for each struct,
//     providing methods for access and manipulation of the underlying byte buffer.
//
// Usage example:
//
// define_tag_types!(
//     ChromaticityType,
//     CurveType,
//     TextType,
// );
//
// This expands to:
//
//   pub struct ChromaticityType(pub Vec<u8>);
//   impl TagTraits for ChromaticityType { ... }
//
//   pub struct CurveType(pub Vec<u8>);
//   impl TagTraits for CurveType { ... }
//
//   pub struct TextType(pub Vec<u8>);
//   impl TagTraits for TextType { ... }
//
// Each type now supports:
//   - `.into_bytes()` to extract ownership of the internal byte buffer
//   - `.as_slice()` to get a reference to the raw data
//   - `.pad(n)` to resize the buffer with trailing zeros (e.g., to align to expected size)
//
// The `$(...)+` syntax in the macro ensures that the enclosed block is repeated for every type provided.
macro_rules! define_tag_types {
    ($($name:ident),+ $(,)?) => {
        // The `$(...)+` block will repeat its contents for each `$name` matched.
        paste! {
            $(
                #[derive(Debug, Serialize, Clone, PartialEq)]
                pub struct [< $name Type >](pub Vec<u8>);

                impl TagTraits for [< $name Type >] {
                    fn into_bytes(self) -> Vec<u8> {
                        // This is the most efficient implementation: it just moves
                        // the Vec<u8> out of the struct without any copying.
                        self.0
                    }
                    fn as_slice(&self) -> &[u8] {
                        // This returns a slice of the internal Vec<u8>.
                        &self.0
                    }
                    fn pad(&mut self, size: usize) {
                        // This pads the internal Vec<u8> to the specified size.
                        // If the current length is less than size, it appends zeros.
                        if self.0.len() < size {
                            self.0.resize(size, 0);
                        }
                    }
                }

                impl Default for [< $name Type >] {
                    fn default() -> Self {
                        // Default implementation creates an empty Vec<u8>
                        Self(Vec::new())
                    }
                }
            )+
        }
    };
}


macro_rules! define_tag_type {
    ($name:ident) => {
        paste! {
            #[derive(Debug, Serialize, Clone, PartialEq)]
            pub struct [< $name Type >](pub Vec<u8>);

            impl TagTraits for [< $name Type >] {
                fn into_bytes(self) -> Vec<u8> {
                    // This is the most efficient implementation: it just moves
                    // the Vec<u8> out of the struct without any copying.
                    self.0
                }
                fn as_slice(&self) -> &[u8] {
                    // This returns a slice of the internal Vec<u8>.
                    &self.0
                }
                fn pad(&mut self, size: usize) {
                    // This pads the internal Vec<u8> to the specified size.
                    // If the current length is less than size, it appends zeros.
                    if self.0.len() < size {
                        self.0.resize(size, 0);
                    }
                }
            }

            impl Default for [< $name Type >] {
                fn default() -> Self {
                    // Default implementation creates an empty Vec<u8>
                    Self(Vec::new())
                }
            }
        }
    };
}
// This defines all the tag types, as wrappers around `Vec<u8>`, the raw data for each tag.
// It alo implements the `TagTraits` for each tag type, allowing them to be converted to bytes,
// sliced, and padded as needed. The length and type signature methods are also provided through
// the trait.
// Change to TagTypes
define_tag_types!(
    Raw,
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

// The `Tag` enum serves as a unified wrapper for all supported ICC tag types.
// Each variant of the enum corresponds to a specific ICC tag type defined in the ICC specification,
// and wraps a strongly-typed struct (e.g., ChromaticityType, CurveType), which itself wraps the raw `Vec<u8>` data.
//
// The purpose of the `Tag` enum is to allow type-safe, centralized handling of heterogeneous tag types
// while preserving the ability to perform runtime type checks and access tag-specific functionality.
//
// Variants are generated using a macro, with one variant per ICC tag type identifier passed to it.
// For example, the macro generates variants such as:
//     Tag::Chromaticity(ChromaticityType)
//     Tag::Curve(CurveType)
//     Tag::Text(TextType)
//     ...and so on.
//
// The `Tag` enum implements the `TagTraits` trait, which defines shared behavior for all tag types,
// such as serialization, deserialization, and type signature retrieval.
//
// For ergonomic and type-safe access, the macro also generates variant-specific accessors ("named inherent methods")
// for each tag type:
//     - `tag.as_chromaticity()` returns `Some(&ChromaticityType)` if the tag is of type Chromaticity, otherwise `None`.
//     - `tag.as_curve_mut()` returns `Some(&mut CurveType)` if the tag is a mutable CurveType, otherwise `None`.
//     - Similar methods exist for all supported tag types.
//
// These accessors allow clients to write clear, safe code when interacting with the dynamic set of tags
// in an ICC profile, without resorting to manual downcasting or unsafe operations.
//
// This design pattern enables flexible runtime dispatch over tag types, while retaining compile-time type safety
// and encapsulating the raw binary representation within each tag-specific type.
macro_rules! enum_tags {
    ( $(#[$doc:meta])* $($variant:ident),+ $(,)?) => {
        paste! {
            /// An enum representing all possible ICC profile tag types.
            #[derive(Debug, Serialize, Clone, PartialEq)]
            pub enum Tag {
                $($variant([< $variant Type >])),+
            }

            /// Implement the `TagTraits` for the `Tag` enum itself.
            /// This implementation dispatches the method call to the specific
            /// variant contained within the enum.
            impl TagTraits for Tag {
                fn as_slice(&self) -> &[u8] {
                    match self {
                        // For each variant, generate a match arm that calls `.as_slice()`
                        // on the inner struct.
                        $(
                            Self::$variant(tag) => tag.as_slice(),
                        )+
                    }
                }

                fn into_bytes(self) -> Vec<u8> {
                    match self {
                        $(
                            Self::$variant(tag) => tag.into_bytes(),
                        )+
                    }
                }

                fn pad(&mut self, size: usize) {
                    match self {
                        $(
                            Self::$variant(tag) => tag.pad(size),
                        )+
                    }
                }
            }


            impl Tag {
                // For each variant name passed to the macro...
                $(
                    /// Returns a reference to the inner struct if the variant matches, otherwise
                    /// returns `None`.
                    pub fn [< as_ $variant:snake >](&self) -> Option<&[< $variant Type >]> {
                        if let Self::$variant(v) = self {
                            Some(v)
                        } else {
                            None
                        }
                    }

                    /// Returns a mutable reference to the inner struct if the variant matches,
                    /// otherwise returns `None`.
                    pub fn [< as_ $variant:snake _mut >](&mut self) -> Option<&mut [< $variant Type >]> {
                        if let Self::$variant(v) = self {
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


// enums and type
enum_tags!(
    /// This enum is used to encapsulate the various tag types defined in the ICC specification.
    /// Each variant corresponds to a specific tag type, allowing for type-safe handling of ICC profile tags.
    Raw,
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
    Technology, // tag derived type
    Text,
    TextDescription,
    U16Fixed16Array, // 'uf32'
    UInt8Array,      // 'ui16'
    UInt16Array,     // 'ui16'
    UInt32Array,     // 'ui32'
    UInt64Array,     // 'ui64'
    Utf8,            // 'utf8'
    Utf16,           // 'ut16'
    Utf8Zip,         // 'zut8'
    Vcgt,            // 'vcgt'
    Vcgp,            // 'vcgp'
    ViewingConditions,
    XYZ
);


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
            TagSignature::ProfileDescriptionTag => {
                Self::MultiLocalizedUnicode(MultiLocalizedUnicodeType(data))
            }

            // ambiguous tags
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

/// Marker traits for tag signatures that can be used in a type-safe manner.
pub trait IsCurveTag {}
pub trait IsTextDescriptionTag {}
pub trait IsMultiLocalizedUnicodeTag {}

/// A trait for tag signatures that have only one valid data type.
pub trait UnambiguousTag {
    /// The single data type associated with this tag signature.
    type DataType: Default;

    /// A function to create the correct `Tag` enum variant from the data.
    fn new_tag(data: Self::DataType) -> Tag;
}

/// A helper macro to reduce boilerplate when implementing `UnambiguousTag`.
macro_rules! impl_unambiguous_tag {
    // Change the first argument to an `ident` to capture just the name (e.g., `RedTRCTag`).
    ($tag_type_name:ident, $data_type:ty, $tag_variant:ident) => {
        // No paste needed. Just write the full path to the type inside the macro.
        // The compiler will correctly substitute the identifier at the end.
        // This also uses the correct ZST pattern (implementing on the type, not a reference).
        impl UnambiguousTag for crate::signatures::tag_signature::$tag_type_name {
            type DataType = $data_type;
            fn new_tag(data: Self::DataType) -> Tag {
                Tag::$tag_variant(data)
            }
        }
    };
}

// Tags of type XYZType
impl_unambiguous_tag!(MediaWhitePointTag, XYZType, XYZ);
impl_unambiguous_tag!(MediaBlackPointTag, XYZType, XYZ);
impl_unambiguous_tag!(LuminanceTag, XYZType, XYZ);

// Tags of type CurveType
impl_unambiguous_tag!(RedTRCTag, CurveType, Curve);
impl_unambiguous_tag!(GreenTRCTag, CurveType, Curve);
impl_unambiguous_tag!(BlueTRCTag, CurveType, Curve);
impl_unambiguous_tag!(GrayTRCTag, CurveType, Curve); // Assuming you have a GrayTRCTag ZST

// Tags of type TextDescriptionType
impl_unambiguous_tag!(CopyrightTag, TextDescriptionType, TextDescription);
impl_unambiguous_tag!(DeviceMfgDescTag, TextDescriptionType, TextDescription);
impl_unambiguous_tag!(DeviceModelDescTag, TextDescriptionType, TextDescription);
impl_unambiguous_tag!(ScreeningDescTag, TextDescriptionType, TextDescription);
impl_unambiguous_tag!(ViewingCondDescTag, TextDescriptionType, TextDescription);

// Tags of type TextType
impl_unambiguous_tag!(CharTargetTag, TextType, Text);

// Tags of type SignatureType
impl_unambiguous_tag!(TechnologyTag, SignatureType, Signature);
impl_unambiguous_tag!(ColorimetricIntentImageStateTag, SignatureType, Signature); // Assuming ZST exists

// Chromaticity and Colorant Tags
impl_unambiguous_tag!(ChromaticityTag, ChromaticityType, Chromaticity);
impl_unambiguous_tag!(ColorantOrderTag, ColorantOrderType, ColorantOrder);
impl_unambiguous_tag!(ColorantTableTag, ColorantTableType, ColorantTable);
impl_unambiguous_tag!(ColorantTableOutTag, ColorantTableType, ColorantTable); // Often same type as clrt
impl_unambiguous_tag!(NamedColor2Tag, NamedColor2Type, NamedColor2);

// Metadata and Informational Tags
impl_unambiguous_tag!(CalibrationDateTimeTag, DateTimeType, DateTime);
impl_unambiguous_tag!(
    ProfileSequenceDescTag,
    ProfileSequenceDescType,
    ProfileSequenceDesc
);
impl_unambiguous_tag!(CrdInfoTag, CrdInfoType, CrdInfo); // Assuming CrdInfoType exists

// Measurement and Viewing Conditions Tags
impl_unambiguous_tag!(MeasurementTag, MeasurementType, Measurement);
impl_unambiguous_tag!(
    ViewingConditionsTag,
    ViewingConditionsType,
    ViewingConditions
);

// Video Color Gamut Tags (from VCGT spec)
impl_unambiguous_tag!(VcgtTag, VcgtType, Vcgt);
impl_unambiguous_tag!(VcgpTag, VcgpType, Vcgp); // Assuming VcgpType exists

impl ColorantOrderType {
    pub fn new(colorant_order: Vec<u8>) -> Self {
        Self(colorant_order)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
