// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

pub mod tagdata;
/*
mod parse;
pub use parse::{UnambiguousTag, IsCurveTag, IsParametricCurveTag, IsTextDescriptionTag, IsMultiLocalizedUnicodeTag, IsLut8DataTag, IsLut16DataTag, IsLutAtoBDataTag, IsLutBtoADataTag};
 */

mod parsed_tag;
pub use parsed_tag::ParsedTag;

mod header_tags;
pub use header_tags::{GamutCheck, Interpolate, Quality, RenderingIntent};

use tagdata::DataSignature;
use tagdata::TagData;

use serde::Serialize;

pub trait TagDataTraits {
    /// Converts the tag data into a byte vector.
    fn into_bytes(self) -> Vec<u8>;
    fn as_slice(&self) -> &[u8];
    fn as_mut_slice(&mut self) -> &mut [u8];
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

// Macro to which dispatches methods for each Tag variant to the corresponding TagData methods.
macro_rules! impl_tag_dispatch {
    (
        $( $(#[$meta:meta])? $variant:ident ),+
        $(,)?
    ) => {
        impl Tag {
            pub fn as_slice(&self) -> &[u8] {
                match self {
                    $(
                        $(#[$meta])? Self::$variant(tagdata) => tagdata.as_slice(),
                    )+
                    Self::Unknown(_, tagdata) => tagdata.as_slice(),
                }
            }

            pub fn into_bytes(self) -> Vec<u8> {
                match self {
                    $(
                        $(#[$meta])? Self::$variant(tagdata) => tagdata.into_bytes(),
                    )+
                    Self::Unknown(_, tagdata) => tagdata.into_bytes(),
                }
            }

            pub fn len(&self) -> usize { self.as_slice().len() }
            pub fn is_empty(&self) -> bool { self.as_slice().is_empty() }

            pub fn pad(&mut self, size: usize) {
                match self {
                    $(
                        $(#[$meta])? Self::$variant(tagdata) => tagdata.pad(size),
                    )+
                    Self::Unknown(_, tagdata) => tagdata.pad(size),
                }
            }

            pub fn to_parsed(&self) -> ParsedTag {
                match self {
                    $(
                        $(#[$meta])? Self::$variant(tagdata) => ParsedTag::from(tagdata),
                    )+
                    Self::Unknown(_, tagdata) => ParsedTag::from(tagdata),
                }
            }

            pub fn data(&self) -> &TagData {
                match self {
                    $(
                        $(#[$meta])? Self::$variant(tagdata) => tagdata,
                    )+
                    Self::Unknown(_, tagdata) => tagdata,
                }
            }

            pub fn data_mut(&mut self) -> &mut TagData {
                match self {
                    $(
                        $(#[$meta])? Self::$variant(tagdata) => tagdata,
                    )+
                    Self::Unknown(_, tagdata) => tagdata,
                }
            }
        }
    };
}

// KEEP define_tag_signatures! as-is; we will reuse it from define_tags!.
macro_rules! define_tag_signatures {
    ($( $(#[$meta:meta])? ($variant:ident, $tag:expr) ),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, strum::AsRefStr)]
        pub enum TagSignature {
            $(
                $(#[$meta])?
                $variant,
            )*
            Unknown(u32),
        }

        impl From<u32> for TagSignature {
            fn from(tag: u32) -> Self {
                match tag {
                    $(
                        $(#[$meta])?
                        $tag => Self::$variant,
                    )*
                    other => Self::Unknown(other),
                }
            }
        }

        impl TagSignature {
            pub fn to_u32(&self) -> u32 {
                match self {
                    $(
                        $(#[$meta])?
                        Self::$variant => $tag,
                    )*
                    Self::Unknown(value) => *value,
                }
            }

            pub fn new(tag: u32) -> Self { Self::from(tag) }
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
    ($( $(#[$meta:meta])? ($variant:ident, $tag_zst:ident, $sig_variant:ident, $tag:expr) ),* $(,)?) => {
        define_tag_signatures!($( $(#[$meta])? ($sig_variant, $tag) ),*);

        #[derive(Debug, Serialize, Clone, PartialEq)]
        pub enum Tag {
            $(
                $(#[$meta])?
                $variant(TagData),
            )*
            Unknown(u32, TagData),
        }

        impl Tag {
            pub fn new(tag_sig_u32: u32, tagdata: TagData) -> Self {
                match tag_sig_u32 {
                    $(
                        $(#[$meta])?
                        $tag => Self::$variant(tagdata),
                    )*
                    _ => Self::Unknown(tag_sig_u32, tagdata),
                }
            }
        }

        impl_tag_dispatch!($(
            $(#[$meta])? $variant
        ),*);

        pub mod tags {
            use super::TagSignature;
            $(
                $(#[$meta])?
                #[derive(Debug, Clone, Copy, Default)]
                pub struct $tag_zst;
                $(#[$meta])?
                impl From<$tag_zst> for TagSignature {
                    fn from(_: $tag_zst) -> TagSignature {
                        TagSignature::$sig_variant
                    }
                }
            )*
        }
    }
}

define_tags!(
    (AToB0, AToB0Tag, AToB0, 0x41324230),
    (AToB1, AToB1Tag, AToB1, 0x41324231),
    (AToB2, AToB2Tag, AToB2, 0x41324232),
    (
        BlueMatrixColumn,
        BlueMatrixColumnTag,
        BlueMatrixColumn,
        0x6258595A
    ),
    (BlueTRC, BlueTRCTag, BlueTRC, 0x62545243),
    (BToA0, BToA0Tag, BToA0, 0x42324130),
    (BToA1, BToA1Tag, BToA1, 0x42324131),
    (BToA2, BToA2Tag, BToA2, 0x42324132),
    (BToD0, BToD0Tag, BToD0, 0x42324430),
    (BToD1, BToD1Tag, BToD1, 0x42324431),
    (BToD2, BToD2Tag, BToD2, 0x42324432),
    (BToD3, BToD3Tag, BToD3, 0x42324433),
    (
        CalibrationDateTime,
        CalibrationDateTimeTag,
        CalibrationDateTime,
        0x63616C74
    ),
    (CharTarget, CharTargetTag, CharTarget, 0x74617267),
    (
        ChromaticAdaptation,
        ChromaticAdaptationTag,
        ChromaticAdaptation,
        0x63686164
    ),
    (Chromaticity, ChromaticityTag, Chromaticity, 0x6368726D),
    (Cicp, CicpTag, Cicp, 0x63696370), // v4.0 to enable HDR metadata
    (ColorantOrder, ColorantOrderTag, ColorantOrder, 0x636C726F),
    (ColorantTable, ColorantTableTag, ColorantTable, 0x636C7274),
    (
        ColorantTableOut,
        ColorantTableOutTag,
        ColorantTableOut,
        0x636C6F74
    ),
    (
        ColorimetricIntentImageState,
        ColorimetricIntentImageStateTag,
        ColorimetricIntentImageState,
        0x63696973
    ),
    (Copyright, CopyrightTag, Copyright, 0x63707274),
    (DeviceMfgDesc, DeviceMfgDescTag, DeviceMfgDesc, 0x646D6E64),
    (
        DeviceModelDesc,
        DeviceModelDescTag,
        DeviceModelDesc,
        0x646D6464
    ),
    (DToB0, DToB0Tag, DToB0, 0x44324230),
    (DToB1, DToB1Tag, DToB1, 0x44324231),
    (DToB2, DToB2Tag, DToB2, 0x44324232),
    (DToB3, DToB3Tag, DToB3, 0x44324233),
    (Gamut, GamutTag, Gamut, 0x67616D74),
    (GrayTRC, GrayTRCTag, GrayTRC, 0x6B545243),
    (
        GreenMatrixColumn,
        GreenMatrixColumnTag,
        GreenMatrixColumn,
        0x6758595A
    ),
    (GreenTRC, GreenTRCTag, GreenTRC, 0x67545243),
    (Luminance, LuminanceTag, Luminance, 0x6C756D69),
    (Measurement, MeasurementTag, Measurement, 0x6D656173),
    (Metadata, MetadataTag, Metadata, 0x6D657461), // v4.4
    (
        MediaBlackPoint,
        MediaBlackPointTag,
        MediaBlackPoint,
        0x626B7074
    ),
    (
        MediaWhitePoint,
        MediaWhitePointTag,
        MediaWhitePoint,
        0x77747074
    ),
    (NamedColor, NamedColorTag, NamedColor, 0x6E636F6C),
    (NamedColor2, NamedColor2Tag, NamedColor2, 0x6E636C32),
    (
        OutputResponse,
        OutputResponseTag,
        OutputResponse,
        0x72657370
    ),
    (
        PerceptualRenderingIntentGamut,
        PerceptualRenderingIntentGamutTag,
        PerceptualRenderingIntentGamut,
        0x72696730
    ),
    (Preview0, Preview0Tag, Preview0, 0x70726530),
    (Preview1, Preview1Tag, Preview1, 0x70726531),
    (Preview2, Preview2Tag, Preview2, 0x70726532),
    (
        ProfileDescription,
        ProfileDescriptionTag,
        ProfileDescription,
        0x64657363
    ),
    (
        ProfileSequenceDesc,
        ProfileSequenceDescTag,
        ProfileSequenceDesc,
        0x70736571
    ),
    (
        ProfileSeqeunceIdentifier,
        ProfileSeqeunceIdendtifierTag,
        ProfileSequenceIdendtifier,
        0x70736964
    ),
    (
        RedMatrixColumn,
        RedMatrixColumnTag,
        RedMatrixColumn,
        0x7258595A
    ),
    (RedTRC, RedTRCTag, RedTRC, 0x72545243),
    (ReferenceName, ReferenceNameTag, ReferenceName, 0x72666E6D),
    (Technology, TechnologyTag, Technology, 0x74656368),
    (
        ViewingCondDesc,
        ViewingCondDescTag,
        ViewingCondDesc,
        0x76756564
    ),
    (
        ViewingConditions,
        ViewingConditionsTag,
        ViewingConditions,
        0x76696577
    ),
    (
        NativeDisplayInfo,
        NativeDisplayInfoTag,
        NativeDisplayInfo,
        0x6E64696E
    ),
    (Vcgt, VcgtTag, Vcgt, 0x76636774),
    (Vcgp, VcgpTag, Vcgp, 0x76636770),
    // V5-only tags
    #[cfg(feature = "v5")]
    (AToB3, AToB3Tag, AToB3, 0x41324233),
    #[cfg(feature = "v5")]
    (AToM0, AToM0Tag, AToM0, 0x41324D30),
    #[cfg(feature = "v5")]
    (
        BrdfColorimetricParameter0,
        BrdfColorimetricParameter0Tag,
        BrdfColorimetricParameter0,
        0x62637030
    ),
    #[cfg(feature = "v5")]
    (
        BrdfColorimetricParameter1,
        BrdfColorimetricParameter1Tag,
        BrdfColorimetricParameter1,
        0x62637031
    ),
    #[cfg(feature = "v5")]
    (
        BrdfColorimetricParameter2,
        BrdfColorimetricParameter2Tag,
        BrdfColorimetricParameter2,
        0x62637032
    ),
    #[cfg(feature = "v5")]
    (
        BrdfColorimetricParameter3,
        BrdfColorimetricParameter3Tag,
        BrdfColorimetricParameter3,
        0x62637033
    ),
    #[cfg(feature = "v5")]
    (
        BrdfSpectralParameter0,
        BrdfSpectralParameter0Tag,
        BrdfSpectralParameter0,
        0x62737030
    ),
    #[cfg(feature = "v5")]
    (
        BrdfSpectralParameter1,
        BrdfSpectralParameter1Tag,
        BrdfSpectralParameter1,
        0x62737031
    ),
    #[cfg(feature = "v5")]
    (
        BrdfSpectralParameter2,
        BrdfSpectralParameter2Tag,
        BrdfSpectralParameter2,
        0x62737032
    ),
    #[cfg(feature = "v5")]
    (
        BrdfSpectralParameter3,
        BrdfSpectralParameter3Tag,
        BrdfSpectralParameter3,
        0x62737033
    ),
    #[cfg(feature = "v5")]
    (BRDFAToB0, BRDFAToB0Tag, BRDFAToB0, 0x62414230),
    #[cfg(feature = "v5")]
    (BRDFAToB1, BRDFAToB1Tag, BRDFAToB1, 0x62414231),
    #[cfg(feature = "v5")]
    (BRDFAToB2, BRDFAToB2Tag, BRDFAToB2, 0x62414232),
    #[cfg(feature = "v5")]
    (BRDFAToB3, BRDFAToB3Tag, BRDFAToB3, 0x62414233),
    #[cfg(feature = "v5")]
    (BRDFDToB0, BRDFDToB0Tag, BRDFDToB0, 0x62444230),
    #[cfg(feature = "v5")]
    (BRDFDToB1, BRDFDToB1Tag, BRDFDToB1, 0x62444231),
    #[cfg(feature = "v5")]
    (BRDFDToB2, BRDFDToB2Tag, BRDFDToB2, 0x62444232),
    #[cfg(feature = "v5")]
    (BRDFDToB3, BRDFDToB3Tag, BRDFDToB3, 0x62444233),
    #[cfg(feature = "v5")]
    (BRDFMToB0, BRDFMToB0Tag, BRDFMToB0, 0x624D4230),
    #[cfg(feature = "v5")]
    (BRDFMToB1, BRDFMToB1Tag, BRDFMToB1, 0x624D4231),
    #[cfg(feature = "v5")]
    (BRDFMToB2, BRDFMToB2Tag, BRDFMToB2, 0x624D4232),
    #[cfg(feature = "v5")]
    (BRDFMToB3, BRDFMToB3Tag, BRDFMToB3, 0x624D4233),
    #[cfg(feature = "v5")]
    (BRDFMToS0, BRDFMToS0Tag, BRDFMToS0, 0x624D5330),
    #[cfg(feature = "v5")]
    (BRDFMToS1, BRDFMToS1Tag, BRDFMToS1, 0x624D5331),
    #[cfg(feature = "v5")]
    (BRDFMToS2, BRDFMToS2Tag, BRDFMToS2, 0x624D5332),
    #[cfg(feature = "v5")]
    (BRDFMToS3, BRDFMToS3Tag, BRDFMToS3, 0x624D5333),
    #[cfg(feature = "v5")]
    (BToA3, BToA3Tag, BToA3, 0x42324133),
    #[cfg(feature = "v5")]
    (
        ColorEncodingParams,
        ColorEncodingParamsTag,
        ColorEncodingParams,
        0x63657074
    ),
    #[cfg(feature = "v5")]
    (
        ColorSpaceName,
        ColorSpaceNameTag,
        ColorSpaceName,
        0x63736E6D
    ),
    #[cfg(feature = "v5")]
    (ColorantInfo, ColorantInfoTag, ColorantInfo, 0x636C696E),
    #[cfg(feature = "v5")]
    (
        ColorantInfoOut,
        ColorantInfoOutTag,
        ColorantInfoOut,
        0x636C696F
    ),
    #[cfg(feature = "v5")]
    (
        ColorantOrderOut,
        ColorantOrderOutTag,
        ColorantOrderOut,
        0x636C6F6F
    ),
    #[cfg(feature = "v5")]
    (CrdInfo, CrdInfoTag, CrdInfo, 0x63726469),
    #[cfg(feature = "v5")]
    (
        CustomToStandardPcc,
        CustomToStandardPccTag,
        CustomToStandardPcc,
        0x63327370
    ),
    #[cfg(feature = "v5")]
    (CxF, CxFTag, CxF, 0x43784620),
    #[cfg(feature = "v5")]
    (Data, DataTag, Data, 0x64617461),
    #[cfg(feature = "v5")]
    (DateTime, DateTimeTag, DateTime, 0x6474696D),
    #[cfg(feature = "v5")]
    (
        DeviceMediaWhitePoint,
        DeviceMediaWhitePointTag,
        DeviceMediaWhitePoint,
        0x646D7770
    ),
    #[cfg(feature = "v5")]
    (
        DeviceSettings,
        DeviceSettingsTag,
        DeviceSettings,
        0x64657673
    ),
    #[cfg(feature = "v5")]
    (
        GamutBoundaryDescription0,
        GamutBoundaryDescription0Tag,
        GamutBoundaryDescription0,
        0x67626430
    ),
    #[cfg(feature = "v5")]
    (
        GamutBoundaryDescription1,
        GamutBoundaryDescription1Tag,
        GamutBoundaryDescription1,
        0x67626431
    ),
    #[cfg(feature = "v5")]
    (
        GamutBoundaryDescription2,
        GamutBoundaryDescription2Tag,
        GamutBoundaryDescription2,
        0x67626432
    ),
    #[cfg(feature = "v5")]
    (
        GamutBoundaryDescription3,
        GamutBoundaryDescription3Tag,
        GamutBoundaryDescription3,
        0x67626433
    ),
    #[cfg(feature = "v5")]
    (
        MaterialDefaultValues,
        MaterialDefaultValuesTag,
        MaterialDefaultValues,
        0x6D647620
    ),
    #[cfg(feature = "v5")]
    (
        MaterialDataArray,
        MaterialDataArrayTag,
        MaterialDataArray,
        0x6D637461
    ),
    #[cfg(feature = "v5")]
    (MToA0, MToA0Tag, MToA0, 0x4D324130),
    #[cfg(feature = "v5")]
    (MToB0, MToB0Tag, MToB0, 0x4D324230),
    #[cfg(feature = "v5")]
    (MToB1, MToB1Tag, MToB1, 0x4D324231),
    #[cfg(feature = "v5")]
    (MToB2, MToB2Tag, MToB2, 0x4D324232),
    #[cfg(feature = "v5")]
    (MToB3, MToB3Tag, MToB3, 0x4D324233),
    #[cfg(feature = "v5")]
    (MToS0, MToS0Tag, MToS0, 0x4D325330),
    #[cfg(feature = "v5")]
    (MToS1, MToS1Tag, MToS1, 0x4D325331),
    #[cfg(feature = "v5")]
    (MToS2, MToS2Tag, MToS2, 0x4D325332),
    #[cfg(feature = "v5")]
    (MToS3, MToS3Tag, MToS3, 0x4D325333),
    #[cfg(feature = "v5")]
    (NamedColorV5, NamedColorV5Tag, NamedColorV5, 0x6E6D636C),
    #[cfg(feature = "v5")]
    (
        PrintCondition,
        PrintConditionTag,
        PrintCondition,
        0x7074636E
    ),
    #[cfg(feature = "v5")]
    (Ps2CRD0, Ps2CRD0Tag, Ps2CRD0, 0x70736430),
    #[cfg(feature = "v5")]
    (Ps2CRD1, Ps2CRD1Tag, Ps2CRD1, 0x70736431),
    #[cfg(feature = "v5")]
    (Ps2CRD2, Ps2CRD2Tag, Ps2CRD2, 0x70736432),
    #[cfg(feature = "v5")]
    (Ps2CRD3, Ps2CRD3Tag, Ps2CRD3, 0x70736433),
    #[cfg(feature = "v5")]
    (Ps2CSA, Ps2CSATag, Ps2CSA, 0x70733273),
    #[cfg(feature = "v5")]
    (
        Ps2RenderingIntent,
        Ps2RenderingIntentTag,
        Ps2RenderingIntent,
        0x70733269
    ),
    #[cfg(feature = "v5")]
    (
        SaturationRenderingIntentGamut,
        SaturationRenderingIntentGamutTag,
        SaturationRenderingIntentGamut,
        0x72696732
    ),
    #[cfg(feature = "v5")]
    (ScreeningDesc, ScreeningDescTag, ScreeningDesc, 0x73637264),
    #[cfg(feature = "v5")]
    (Screening, ScreeningTag, Screening, 0x7363726E),
    #[cfg(feature = "v5")]
    (
        SpectralDataInfo,
        SpectralDataInfoTag,
        SpectralDataInfo,
        0x7364696E
    ),
    #[cfg(feature = "v5")]
    (
        SpectralWhitePoint,
        SpectralWhitePointTag,
        SpectralWhitePoint,
        0x73777074
    ),
    #[cfg(feature = "v5")]
    (
        SpectralViewingConditions,
        SpectralViewingConditionsTag,
        SpectralViewingConditions,
        0x7376636E
    ),
    #[cfg(feature = "v5")]
    (
        StandardToCustomPcc,
        StandardToCustomPccTag,
        StandardToCustomPcc,
        0x73326370
    ),
    #[cfg(feature = "v5")]
    (SurfaceMap, SurfaceMapTag, SurfaceMap, 0x736D6170),
    #[cfg(feature = "v5")]
    (UcrBg, UcrBgTag, UcrBg, 0x62666420),
    #[cfg(feature = "v5")]
    (MakeAndModel, MakeAndModelTag, MakeAndModel, 0x6D6D6F64),
    #[cfg(feature = "v5")]
    (
        EmbeddedV5Profile,
        EmbeddedV5ProfileTag,
        EmbeddedV5Profile,
        0x49434335
    ),
);
