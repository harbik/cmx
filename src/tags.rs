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

mod header_tags;

pub use header_tags::{GamutCheck, Interpolate, Quality, RenderingIntent, S15Fixed16};
use zerocopy::{BigEndian, IntoBytes, U16};

use crate::{
    signatures::{
        type_signature::TypeSignature,
        TagSignature,
    },
};

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

macro_rules! define_tags {
    // Match one or more identifiers, separated by commas.
    // The `$(,)?` at the end allows an optional trailing comma for better formatting.
    ($($name:ident),+ $(,)?) => {
        // The `$(...)+` block will repeat its contents for each `$name` matched.
        $(
            #[derive(Debug, Serialize, Clone, PartialEq)]
            pub struct $name(pub Vec<u8>);

            impl TagTraits for $name {
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
        )+
    };
}

define_tags!(
    Raw,
    Chromaticity,
    ColorantOrder,
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
/// TODO, all of these to implement the IntoBytes trait, so that they can be serialized to bytes easily.

macro_rules! enum_tag {
    (
        $(#[$doc:meta])*
        $($variant:ident),+ $(,)?) => {
        /// An enum representing all possible ICC profile tag types.
        #[derive(Debug, Serialize, Clone, PartialEq)]
        pub enum Tag {
            // Generate a variant for each identifier passed to the macro.
            // e.g., Chromaticity(Chromaticity), Curve(Curve), etc.
            $($variant($variant)),+
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
    };
}

// enums and type
enum_tag!(
    /// This enum is used to encapsulate the various tag types defined in the ICC specification.
    /// Each variant corresponds to a specific tag type, allowing for type-safe handling of ICC profile tags.   
    Raw,
    Chromaticity,
    ColorantOrder,
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
    /// Creates a new `Tag` instance from a `TagSignature` and raw data.
    /// As a couple of tag signatures map to multiple types,
    pub fn new(tag_signature: TagSignature, data: Vec<u8>) -> Self {
        let type_signature = TypeSignature::from(<[u8; 4]>::try_from(&data[0..4]).unwrap());
        match (tag_signature, type_signature) {
            (TagSignature::GreenTRCTag, TypeSignature::CurveType) => Self::Curve(Curve(data)),
            (TagSignature::GreenTRCTag, TypeSignature::ParametricCurveType) => {
                Self::ParametricCurve(ParametricCurve(data))
            }
            (TagSignature::ProfileDescriptionTag, _) => {
                Self::MultiLocalizedUnicode(MultiLocalizedUnicode(data))
            }
            _ => Self::Raw(Raw(data)),
        }
    }
}

// Tag Type definitions
// Simple tag types defined here, complex tag types in separate files


impl ColorantOrder {
    pub fn new(colorant_order: Vec<u8>) -> Self {
        Self(colorant_order)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}


impl Curve {
    pub fn new(data: &[u16]) -> Self {
        let be_values: Vec<U16<BigEndian>> = data.iter().map(|&val| U16::new(val)).collect();

        Self(be_values.as_bytes().to_vec())
    }
}

impl Data {
    pub fn new(data: &[u16]) -> Self {
        let be_values: Vec<U16<BigEndian>> = data.iter().map(|&val| U16::new(val)).collect();

        Self(be_values.as_bytes().to_vec())
    }
}

