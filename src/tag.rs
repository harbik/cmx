// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

pub mod tagdata;
/*
mod parse;
pub use parse::{UnambiguousTag, IsCurveTag, IsParametricCurveTag, IsTextDescriptionTag, IsMultiLocalizedUnicodeTag, IsLut8DataTag, IsLut16DataTag, IsLutAtoBDataTag, IsLutBtoADataTag};
 */

mod tagtype;
pub use tagtype::ParsedTag;

mod header_tags;
pub use header_tags::{GamutCheck, Interpolate, Quality, RenderingIntent, S15Fixed16};

use tagdata::TagData;
use tagdata::DataSignature;

use serde::Serialize;

pub trait TagDataTraits {
    /// Converts the tag data into a byte vector.
    fn into_bytes(self) -> Vec<u8>;
    fn as_slice(&self) -> &[u8];
    fn len(&self) -> usize {
        self.as_slice().len()
    }
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }
    fn pad(&mut self, size: usize);
    fn type_signature(&self) -> DataSignature {
        let array: [u8; 4] = self.as_slice()[0..4].try_into().unwrap();
        array.into()
    }
}

/// Represents a single tag entry in an ICC profile,
/// containing an offset, size, and it's raw tag bytes,
/// and is the main interface for accessing tag data through
/// the IndexMap in the `RawProfile`.
/// These are the offset and size used to import the tag data from
/// the raw bytes of the ICC profile, and are also used to write the
/// tag data back to the raw bytes when exporting the profile.
/// When writing the data the size of all the tags is checked to see
/// if any tag data has changed in size, and if so all the tags are
/// re-arranged to fit the new size.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ProfileTagRecord {
    pub offset: u32,
    pub size: u32,
    pub tag: Tag,
}

impl ProfileTagRecord {
    /// Creates a new `ProfileTagRecord` with the given offset, size, and tag.
    /// It is used to represent a tag as present in a ICC profile, with its offset and size.
    pub fn new(offset: u32, size: u32, tag: Tag) -> Self {
        Self { offset, size, tag }
    }

    /// Returns the raw bytes of the tag.
    pub fn as_slice(&self) -> &[u8] {
        self.tag.as_slice()
    }
}

// Macro to which dispatches methods for each Tag variant to the corresponding TagData methods.
macro_rules! impl_tag_dispatch {
    (
        // This macro accepts a comma-separated list of Tag identifiers.
        $( $variant:ident ),+
        $(,)? // Allows an optional trailing comma
    ) => {
        // It generates the implementation block for the Tag enum.
        impl Tag {
            /// Returns the raw bytes of the tag.
            pub fn as_slice(&self) -> &[u8] {
                match self {
                    // It creates a match arm for every variant provided.
                    $(
                        Self::$variant(tagdata) => tagdata.as_slice(),
                    )+
                    Self::Unknown(_, tagdata) => tagdata.as_slice(),
                }
            }

            /// Converts the tag into a byte vector.
            pub fn into_bytes(self) -> Vec<u8> {
                match self {
                    $(
                        Self::$variant(tagdata) => tagdata.into_bytes(),
                    )+
                    Self::Unknown(_, tagdata) => tagdata.into_bytes(),
                }
            }

            pub fn len(&self) -> usize {
                self.as_slice().len()
            }

            pub fn is_empty(&self) -> bool {
                self.as_slice().is_empty()
            }

            pub fn pad(&mut self, size: usize) {
                match self {
                    $(
                        Self::$variant(tagdata) => tagdata.pad(size),
                    )+
                    Self::Unknown(_, tagdata) => tagdata.pad(size),
                }
            }

            pub fn to_parsed(&self) -> ParsedTag {
                match self {
                    $(
                        // Self::$variant(tagdata) => tagdata.to_toml(),
                        Self::$variant(tagdata) => ParsedTag::from(tagdata),
                    )+
                   // Self::Unknown(_, tagdata) => tagdata.to_toml(),
                    Self::Unknown(_, tagdata) => ParsedTag::from(tagdata),
                }
            }
        }

    };
}

// KEEP define_tag_signatures! as-is; we will reuse it from define_tags!.
macro_rules! define_tag_signatures {
    ($(($variant:ident, $tag:expr)),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, strum::AsRefStr)]
        pub enum TagSignature {
            $($variant,)*
            Unknown(u32),
        }

        impl From<u32> for TagSignature {
            fn from(tag: u32) -> Self {
                match tag {
                    $($tag => Self::$variant,)*
                    other => Self::Unknown(other),
                }
            }
        }

        impl TagSignature {
            pub fn to_u32(&self) -> u32 {
                match self {
                    $(Self::$variant => $tag,)*
                    Self::Unknown(value) => *value,
                }
            }
            
            /// Creates a new `TagSignature` from a u32 value.
            pub fn new(tag: u32) -> Self {
                Self::from(tag)
            }
        }
        
        impl std::fmt::Display for TagSignature {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let value = self.to_u32();
                let bytes = value.to_be_bytes();
                let s = String::from_utf8_lossy(&bytes);
                if s.is_ascii() && s.len() == 4 {
                    write!(f, "{s}")
                } else {
                    write!(f, "{value:08X}")
                }
            }
        }
    }
}

// New macro: single source of truth to define TagSignature + Tag + Tag::new + dispatch.
macro_rules! define_tags {
    ($(($variant:ident, $sig_variant:ident, $tag:expr)),* $(,)?) => {
        // Generate TagSignature using the existing macro.
        define_tag_signatures!($(($sig_variant, $tag)),*);

        #[derive(Debug, Serialize, Clone, PartialEq)]
        pub enum Tag {
            $($variant(TagData),)*
            Unknown(u32, TagData),
        }

        impl Tag {
            pub fn new(tag_sig_u32: u32, tagdata: TagData) -> Self {
                match tag_sig_u32 {
                    $($tag => Self::$variant(tagdata),)*
                    _ => Self::Unknown(tag_sig_u32, tagdata),
                }
            }
        }

        // Generate as_slice/into_bytes/len/is_empty/pad/to_parsed
        impl_tag_dispatch!($($variant),*);
    }
}

// Use the new macro to define everything once.
// Note: list mirrors your current one; keep entries unchanged, just add the Tag variant name up front.
define_tags!(
    (AToB0, AToB0Tag, 0x41324230),
    (AToB1, AToB1Tag, 0x41324231),
    (AToB2, AToB2Tag, 0x41324232),
    (AToB3, AToB3Tag, 0x41324233),
    (AToM0, AToM0Tag, 0x41324D30),
    (BlueMatrixColumn, BlueMatrixColumnTag, 0x6258595A),
    (BlueTRC, BlueTRCTag, 0x62545243),
    (BrdfColorimetricParameter0, BrdfColorimetricParameter0Tag, 0x62637030),
    (BrdfColorimetricParameter1, BrdfColorimetricParameter1Tag, 0x62637031),
    (BrdfColorimetricParameter2, BrdfColorimetricParameter2Tag, 0x62637032),
    (BrdfColorimetricParameter3, BrdfColorimetricParameter3Tag, 0x62637033),
    (BrdfSpectralParameter0, BrdfSpectralParameter0Tag, 0x62737030),
    (BrdfSpectralParameter1, BrdfSpectralParameter1Tag, 0x62737031),
    (BrdfSpectralParameter2, BrdfSpectralParameter2Tag, 0x62737032),
    (BrdfSpectralParameter3, BrdfSpectralParameter3Tag, 0x62737033),
    (BRDFAToB0, BRDFAToB0Tag, 0x62414230),
    (BRDFAToB1, BRDFAToB1Tag, 0x62414231),
    (BRDFAToB2, BRDFAToB2Tag, 0x62414232),
    (BRDFAToB3, BRDFAToB3Tag, 0x62414233),
    (BRDFDToB0, BRDFDToB0Tag, 0x62444230),
    (BRDFDToB1, BRDFDToB1Tag, 0x62444231),
    (BRDFDToB2, BRDFDToB2Tag, 0x62444232),
    (BRDFDToB3, BRDFDToB3Tag, 0x62444233),
    (BRDFMToB0, BRDFMToB0Tag, 0x624D4230),
    (BRDFMToB1, BRDFMToB1Tag, 0x624D4231),
    (BRDFMToB2, BRDFMToB2Tag, 0x624D4232),
    (BRDFMToB3, BRDFMToB3Tag, 0x624D4233),
    (BRDFMToS0, BRDFMToS0Tag, 0x624D5330),
    (BRDFMToS1, BRDFMToS1Tag, 0x624D5331),
    (BRDFMToS2, BRDFMToS2Tag, 0x624D5332),
    (BRDFMToS3, BRDFMToS3Tag, 0x624D5333),
    (BToA0, BToA0Tag, 0x42324130),
    (BToA1, BToA1Tag, 0x42324131),
    (BToA2, BToA2Tag, 0x42324132),
    (BToA3, BToA3Tag, 0x42324133),
    (CalibrationDateTime, CalibrationDateTimeTag, 0x63616C74),
    (CharTarget, CharTargetTag, 0x74617267),
    (ChromaticAdaptation, ChromaticAdaptationTag, 0x63686164),
    (Chromaticity, ChromaticityTag, 0x6368726D),
    (ColorEncodingParams, ColorEncodingParamsTag, 0x63657074),
    (ColorSpaceName, ColorSpaceNameTag, 0x63736E6D),
    (ColorantInfo, ColorantInfoTag, 0x636C696E),
    (ColorantInfoOut, ColorantInfoOutTag, 0x636C696F),
    (ColorantOrder, ColorantOrderTag, 0x636C726F),
    (ColorantOrderOut, ColorantOrderOutTag, 0x636C6F6F),
    (ColorantTable, ColorantTableTag, 0x636C7274),
    (ColorantTableOut, ColorantTableOutTag, 0x636C6F74),
    (ColorimetricIntentImageState, ColorimetricIntentImageStateTag, 0x63696973),
    (Copyright, CopyrightTag, 0x63707274),
    (CrdInfo, CrdInfoTag, 0x63726469),
    (CustomToStandardPcc, CustomToStandardPccTag, 0x63327370),
    (CxF, CxFTag, 0x43784620),
    (Data, DataTag, 0x64617461),
    (DateTime, DateTimeTag, 0x6474696D),
    (DeviceMediaWhitePoint, DeviceMediaWhitePointTag, 0x646D7770),
    (DeviceMfgDesc, DeviceMfgDescTag, 0x646D6E64),
    (DeviceModelDesc, DeviceModelDescTag, 0x646D6464),
    (DeviceSettings, DeviceSettingsTag, 0x64657673),
    (DToB0, DToB0Tag, 0x44324230),
    (DToB1, DToB1Tag, 0x44324231),
    (DToB2, DToB2Tag, 0x44324232),
    (DToB3, DToB3Tag, 0x44324233),
    (BToD0, BToD0Tag, 0x42324430),
    (BToD1, BToD1Tag, 0x42324431),
    (BToD2, BToD2Tag, 0x42324432),
    (BToD3, BToD3Tag, 0x42324433),
    (Gamut, GamutTag, 0x67616D74),
    (GamutBoundaryDescription0, GamutBoundaryDescription0Tag, 0x67626430),
    (GamutBoundaryDescription1, GamutBoundaryDescription1Tag, 0x67626431),
    (GamutBoundaryDescription2, GamutBoundaryDescription2Tag, 0x67626432),
    (GamutBoundaryDescription3, GamutBoundaryDescription3Tag, 0x67626433),
    (GrayTRC, GrayTRCTag, 0x6B545243),
    (GreenMatrixColumn, GreenMatrixColumnTag, 0x6758595A),
    (GreenTRC, GreenTRCTag, 0x67545243),
    (Luminance, LuminanceTag, 0x6C756D69),
    (MaterialDefaultValues, MaterialDefaultValuesTag, 0x6D647620),
    (MaterialDataArray, MaterialDataArrayTag, 0x6D637461),
    (MToA0, MToA0Tag, 0x4D324130),
    (MToB0, MToB0Tag, 0x4D324230),
    (MToB1, MToB1Tag, 0x4D324231),
    (MToB2, MToB2Tag, 0x4D324232),
    (MToB3, MToB3Tag, 0x4D324233),
    (MToS0, MToS0Tag, 0x4D325330),
    (MToS1, MToS1Tag, 0x4D325331),
    (MToS2, MToS2Tag, 0x4D325332),
    (MToS3, MToS3Tag, 0x4D325333),
    (Measurement, MeasurementTag, 0x6D656173),
    (MediaBlackPoint, MediaBlackPointTag, 0x626B7074),
    (MediaWhitePoint, MediaWhitePointTag, 0x77747074),
    (MetaData, MetaDataTag, 0x6D657461),
    (NamedColor, NamedColorTag, 0x6E636F6C),
    (NamedColorV5, NamedColorV5Tag, 0x6E6D636C),
    (NamedColor2, NamedColor2Tag, 0x6E636C32),
    (OutputResponse, OutputResponseTag, 0x72657370),
    (PerceptualRenderingIntentGamut, PerceptualRenderingIntentGamutTag, 0x72696730),
    (Preview0, Preview0Tag, 0x70726530),
    (Preview1, Preview1Tag, 0x70726531),
    (Preview2, Preview2Tag, 0x70726532),
    (PrintCondition, PrintConditionTag, 0x7074636E),
    (ProfileDescription, ProfileDescriptionTag, 0x64657363),
    (ProfileSequenceDesc, ProfileSequenceDescTag, 0x70736571),
    (ProfileSequceId, ProfileSequceIdTag, 0x70736964),
    (Ps2CRD0, Ps2CRD0Tag, 0x70736430),
    (Ps2CRD1, Ps2CRD1Tag, 0x70736431),
    (Ps2CRD2, Ps2CRD2Tag, 0x70736432),
    (Ps2CRD3, Ps2CRD3Tag, 0x70736433),
    (Ps2CSA, Ps2CSATag, 0x70733273),
    (Ps2RenderingIntent, Ps2RenderingIntentTag, 0x70733269),
    (RedMatrixColumn, RedMatrixColumnTag, 0x7258595A),
    (RedTRC, RedTRCTag, 0x72545243),
    (ReferenceName, ReferenceNameTag, 0x72666E6D),
    (SaturationRenderingIntentGamut, SaturationRenderingIntentGamutTag, 0x72696732),
    (ScreeningDesc, ScreeningDescTag, 0x73637264),
    (Screening, ScreeningTag, 0x7363726E),
    (SpectralDataInfo, SpectralDataInfoTag, 0x7364696E),
    (SpectralWhitePoint, SpectralWhitePointTag, 0x73777074),
    (SpectralViewingConditions, SpectralViewingConditionsTag, 0x7376636E),
    (StandardToCustomPcc, StandardToCustomPccTag, 0x73326370),
    (SurfaceMap, SurfaceMapTag, 0x736D6170),
    (Technology, TechnologyTag, 0x74656368),
    (UcrBg, UcrBgTag, 0x62666420),
    (ViewingCondDesc, ViewingCondDescTag, 0x76756564),
    (ViewingConditions, ViewingConditionsTag, 0x76696577),
    (EmbeddedV5Profile, EmbeddedV5ProfileTag, 0x49434335),
    (MakeAndModel, MakeAndModelTag, 0x6D6D6F64),
    (MultilocalizedDescriptionString, MultilocalizedDescriptionStringTag, 0x6473636D),
    (NativeDisplayInfo, NativeDisplayInfoTag, 0x6E64696E),
    (Vcgt, VcgtTag, 0x76636774),
    (Vcgp, VcgpTag, 0x76636770),
    (AbsToRelTransSpace, AbsToRelTransSpaceTag, 0x61727473),
);
