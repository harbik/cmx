
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC



use paste::paste;

use crate::tag::TagSignature;

use super::tag_value::*;


/// A trait for tag signatures that have only one valid data type.
pub trait UnambiguousTag {
    /// The single data type associated with this tag signature.
    type TagType: Default;

    /// A function to create the correct `TagData` enum variant from the TagData block data.
    fn new_tag(tag_type_instance: Self::TagType) -> TagData;
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
                fn new_tag(tag_type_instance: Self::TagType) -> TagData {
                    TagData::$tag_variant(tag_type_instance)
                }
            }
        }
    };
}

// Tags of type XYZType
impl_unambiguous_tag!(MediaWhitePointTag, XYZArray);
impl_unambiguous_tag!(MediaBlackPointTag, XYZArray);
impl_unambiguous_tag!(LuminanceTag, XYZArray);

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


// Marker traits (families of allowed inner types)
pub trait IsTextDescriptionTag {}
pub trait IsMultiLocalizedUnicodeTag {}
pub trait IsCurveTag {}
pub trait IsParametricCurveTag {}
pub trait IsLut8TypeTag {}
pub trait IsLut16TypeTag {}
pub trait IsLutAtoBTypeTag {}
pub trait IsLutBtoATypeTag {}

// Family helpers: dispatch by inner type signature (first 4 bytes).
#[inline]
fn inner_sig(bytes: &[u8]) -> Option<&[u8; 4]> {
    if bytes.len() >= 4 { Some(bytes[..4].try_into().ok()?) } else { None }
}

#[inline]
fn parse_trc_family<S: IsCurveTag + IsParametricCurveTag>(data: Vec<u8>) -> TagData {
    match inner_sig(&data) {
        Some(b"curv") => TagData::Curve(CurveType(data)),
        Some(b"para") => TagData::ParametricCurve(ParametricCurveType(data)),
        _ => TagData::Raw(RawType(data)),
    }
}

#[inline]
fn parse_desc_family<S: IsTextDescriptionTag + IsMultiLocalizedUnicodeTag>(data: Vec<u8>) -> TagData {
    match inner_sig(&data) {
        Some(b"desc") => TagData::TextDescription(TextDescriptionType(data)),
        Some(b"mluc") => TagData::MultiLocalizedUnicode(MultiLocalizedUnicodeType(data)),
        _ => TagData::Raw(RawType(data)),
    }
}

#[inline]
fn parse_preview_family<S: IsLut8TypeTag + IsLut16TypeTag>(data: Vec<u8>) -> TagData {
    match inner_sig(&data) {
        Some(b"mft1") => TagData::Lut8(Lut8Type(data)),
        Some(b"mft2") => TagData::Lut16(Lut16Type(data)),
        _ => TagData::Raw(RawType(data)),
    }
}

#[inline]
fn parse_atob_family<S: IsLutAtoBTypeTag>(data: Vec<u8>) -> TagData {
    match inner_sig(&data) {
        Some(b"mAB ") => TagData::LutAToB(LutAToBType(data)),
        _ => TagData::Raw(RawType(data)),
    }
}

#[inline]
fn parse_btoa_family<S: IsLutBtoATypeTag>(data: Vec<u8>) -> TagData {
    match inner_sig(&data) {
        Some(b"mBA ") => TagData::LutBToA(LutBToAType(data)),
        _ => TagData::Raw(RawType(data)),
    }
}

#[inline]
fn parse_gamut_family<S: IsLut8TypeTag + IsLut16TypeTag + IsLutBtoATypeTag>(data: Vec<u8>) -> TagData {
    match inner_sig(&data) {
        Some(b"mft1") => TagData::Lut8(Lut8Type(data)),
        Some(b"mft2") => TagData::Lut16(Lut16Type(data)),
        Some(b"mBA ") => TagData::LutBToA(LutBToAType(data)),
        _ => TagData::Raw(RawType(data)),
    }
}

// 'desc' family
impl IsTextDescriptionTag for ProfileDescriptionTag {}
impl IsMultiLocalizedUnicodeTag for ProfileDescriptionTag {}
impl IsTextDescriptionTag for DeviceMfgDescTag {}
impl IsMultiLocalizedUnicodeTag for DeviceMfgDescTag {}
impl IsTextDescriptionTag for DeviceModelDescTag {}
impl IsMultiLocalizedUnicodeTag for DeviceModelDescTag {}

// TRC family
impl IsCurveTag for RedTRCTag {}
impl IsParametricCurveTag for RedTRCTag {}
impl IsCurveTag for GreenTRCTag {}
impl IsParametricCurveTag for GreenTRCTag {}
impl IsCurveTag for BlueTRCTag {}
impl IsParametricCurveTag for BlueTRCTag {}
impl IsCurveTag for GrayTRCTag {}
impl IsParametricCurveTag for GrayTRCTag {}

// Preview LUTs
impl IsLut8TypeTag for Preview0Tag {}
impl IsLut16TypeTag for Preview0Tag {}
impl IsLut8TypeTag for Preview1Tag {}
impl IsLut16TypeTag for Preview1Tag {}
impl IsLut8TypeTag for Preview2Tag {}
impl IsLut16TypeTag for Preview2Tag {}

// AToB / BToA families
impl IsLutAtoBTypeTag for AToB0Tag {}
impl IsLutAtoBTypeTag for AToB1Tag {}
impl IsLutAtoBTypeTag for AToB2Tag {}
impl IsLutBtoATypeTag for BToA0Tag {}
impl IsLutBtoATypeTag for BToA1Tag {}
impl IsLutBtoATypeTag for BToA2Tag {}

// Gamut family
impl IsLut8TypeTag for GamutTag {}
impl IsLut16TypeTag for GamutTag {}
impl IsLutBtoATypeTag for GamutTag {}



// Use the family helpers inside the single factory.
impl TagData {
    pub fn parse(signature: TagSignature, data: Vec<u8>) -> Self {
        match signature {
            // Ambiguous families (marker-constrained)
            TagSignature::RedTRCTag => parse_trc_family::<RedTRCTag>(data),
            TagSignature::GreenTRCTag => parse_trc_family::<GreenTRCTag>(data),
            TagSignature::BlueTRCTag => parse_trc_family::<BlueTRCTag>(data),
            TagSignature::GrayTRCTag => parse_trc_family::<GrayTRCTag>(data),

            TagSignature::ProfileDescriptionTag => parse_desc_family::<ProfileDescriptionTag>(data),
            TagSignature::DeviceMfgDescTag => parse_desc_family::<DeviceMfgDescTag>(data),
            TagSignature::DeviceModelDescTag => parse_desc_family::<DeviceModelDescTag>(data),

            TagSignature::Preview0Tag => parse_preview_family::<Preview0Tag>(data),
            TagSignature::Preview1Tag => parse_preview_family::<Preview1Tag>(data),
            TagSignature::Preview2Tag => parse_preview_family::<Preview2Tag>(data),

            TagSignature::AToB0Tag => parse_atob_family::<AToB0Tag>(data),
            TagSignature::AToB1Tag => parse_atob_family::<AToB1Tag>(data),
            TagSignature::AToB2Tag => parse_atob_family::<AToB2Tag>(data),
            TagSignature::BToA0Tag => parse_btoa_family::<BToA0Tag>(data),
            TagSignature::BToA1Tag => parse_btoa_family::<BToA1Tag>(data),
            TagSignature::BToA2Tag => parse_btoa_family::<BToA2Tag>(data),

            TagSignature::GamutTag => parse_gamut_family::<GamutTag>(data),

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
            TagSignature::VcgtTag => Self::Vcgt(VcgtType(data)),
            TagSignature::VcgpTag => Self::Vcgp(VcgpType(data)),
            TagSignature::ViewingConditionsTag => {
                Self::ViewingConditions(ViewingConditionsType(data))
            }
            TagSignature::MediaWhitePointTag => Self::XYZArray(XYZArrayType(data)),

            // Fallback
            _ => Self::Raw(RawType(data)),
        }
    }
}