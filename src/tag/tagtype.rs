// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::tag::{
    tagdata::{
        chromaticity::ChromaticityType, curve::CurveType, lut8::Lut8Type, measurement::MeasurementType, multi_localized_unicode::MultiLocalizedUnicodeType, parametric_curve::ParametricCurveType, raw::UnparsedType, s15fixed16array::S15Fixed16ArrayType, text::TextType, text_description::TextDescriptionType, xyz::XYZArrayType
    },
    TagData,
};

/// A TOML-serializable wrapper enum that captures all supported tag variants in a single type.
///
/// This enum acts as the serialization boundary for heterogeneous tags by wrapping each concrete
/// TOML-friendly tag representation (e.g., XYZ, Curve, TextDescription). It is marked
/// `#[serde(untagged)]`, which means no extra discriminant is added during serialization: each
/// variant is emitted as its inner structure, keeping the resulting TOML compact and intuitive.
///
/// Purpose:
/// - Provide a single, serializable sum type for all tag kinds when emitting TOML.
/// - Allow seamless conversion from the internal `TagData` type via `From<&TagData>` and helper methods
///   like `TagData::to_toml()` and bulk conversion utilities.
/// - Keep TOML output free of enum tags (thanks to `untagged`), so each tag serializes to the
///   shape of its underlying type.
///
/// Extensibility:
/// - When introducing a new tag kind, add a new variant that wraps its TOML representation and
///   update the `From<&TagData>` match accordingly.
/// - Because the enum is `untagged`, ensure new variants have unambiguous shapes to avoid
///   potential collisions during (future) deserialization.
///
#[derive(Serialize)]
#[serde(untagged)]
pub enum ParsedTag {
    Chromaticity(ChromaticityType),
    Curve(CurveType),
    Lut8(Lut8Type),
    Measurement(MeasurementType),
    MultiLocalizedUnicode(MultiLocalizedUnicodeType),
    ParametricCurve(ParametricCurveType),
    S15Fixed16Array(S15Fixed16ArrayType),
    Text(TextType),
    TextDescription(TextDescriptionType),
    XYZArray(XYZArrayType),

    // Graceful fallback for unrecognized or unsupported tags
    Unparsed(UnparsedType),
}

/// Converts a `TagData`, which is enum collection of encapsulated TagDatas,
/// into a `TagToml` representation. This is used for serializing
/// the tag into a TOML format, using the `Display` trait for `RawProfile`.
///
/// This requires each TagData to implement `From<&TagData> for TagDataToml`, which
/// converts the raw bytes into a serializable format, specific for each tag.
///
impl From<&TagData> for ParsedTag {
    fn from(tag: &TagData) -> Self {
        match tag {
            TagData::Chromaticity(chromaticity) => ParsedTag::Chromaticity(chromaticity.into()),
            TagData::Curve(curve) => ParsedTag::Curve(curve.into()),
            TagData::Lut8(lut8) => ParsedTag::Lut8(lut8.into()),
            TagData::Measurement(measurement) => {
                ParsedTag::Measurement(measurement.into())
            }
            TagData::MultiLocalizedUnicode(mluc) => ParsedTag::MultiLocalizedUnicode(mluc.into()),
            TagData::ParametricCurve(para) => ParsedTag::ParametricCurve(para.into()),
            TagData::S15Fixed16Array(values) => ParsedTag::S15Fixed16Array(values.into()),
            TagData::Text(text) => ParsedTag::Text(text.into()),
            TagData::TextDescription(text_desc) => ParsedTag::TextDescription(text_desc.into()),
            TagData::XYZArray(xyz) => ParsedTag::XYZArray(xyz.into()),
            
            // Graceful fallback: don't panic, just emit the unparsed data
            _ => ParsedTag::Unparsed(UnparsedType::from(tag))
        }
    }
}
