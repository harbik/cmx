// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::tag::{
    tagdata::{
        chromaticity::ChromaticityType, curve::CurveType, lut8::Lut8Type, measurement::MeasurementType, multi_localized_unicode::MultiLocalizedUnicodeType, parametric_curve::ParametricCurveType, raw::RawType, s15fixed16array::S15Fixed16ArrayType, text::TextType, text_description::TextDescriptionType, xyz::XYZArrayDataToml
    },
    TagTraits, TagData,
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
pub enum TagType {
    Chromaticity(ChromaticityType),
    Curve(CurveType),
    Lut8(Lut8Type),
    Measurement(MeasurementType),
    MultiLocalizedUnicode(MultiLocalizedUnicodeType),
    ParametricCurve(ParametricCurveType),
    S15Fixed16Array(S15Fixed16ArrayType),
    Text(TextType),
    TextDescription(TextDescriptionType),
    XYZArray(XYZArrayDataToml),

    Raw(RawType),
    // Fallback when no dedicated TOML format is implemented for a tag variant.
    // Kept minimal and unambiguous for untagged serialization.
    UnsupportedTag {
        unsupported_tag: String,
        type_signature: Option<String>,
    },
    // ... add a variant for every tag type you want to serialize
}

/// Converts a `TagData`, which is enum collection of encapsulated TagDatas,
/// into a `TagToml` representation. This is used for serializing
/// the tag into a TOML format, using the `Display` trait for `RawProfile`.
///
/// This requires each TagData to implement `From<&TagData> for TagDataToml`, which
/// converts the raw bytes into a serializable format, specific for each tag.
///
impl From<&TagData> for TagType {
    fn from(tag: &TagData) -> Self {
        match tag {
            TagData::Chromaticity(chromaticity) => TagType::Chromaticity(chromaticity.into()),
            TagData::Curve(curve) => TagType::Curve(curve.into()),
            TagData::Lut8(lut8) => TagType::Lut8(lut8.into()),
            TagData::Measurement(measurement) => {
                TagType::Measurement(measurement.into())
            }
            TagData::MultiLocalizedUnicode(mluc) => TagType::MultiLocalizedUnicode(mluc.into()),
            TagData::ParametricCurve(para) => TagType::ParametricCurve(para.into()),
            TagData::S15Fixed16Array(values) => TagType::S15Fixed16Array(values.into()),
            TagData::Text(text) => TagType::Text(text.into()),
            TagData::TextDescription(text_desc) => TagType::TextDescription(text_desc.into()),
            TagData::XYZArray(xyz) => TagType::XYZArray(xyz.into()),
            TagData::Raw(raw) => TagType::Raw(raw.into()),
            // Graceful fallback: don't panic, just emit a small structured note.
            _ => {
                let type_signature = tag
                    .as_slice()
                    .get(0..4)
                    .map(|b| String::from_utf8_lossy(b).to_string());
                TagType::UnsupportedTag {
                    unsupported_tag: "No dedicated TOML format implemented for this tag"
                        .to_string(),
                    type_signature,
                }
            } // Add more matches for other tag types as needed
        }
    }
}
impl super::TagData {
    /// Converts the tag into a serializable TOML representation.
    pub fn to_toml(&self) -> TagType {
        TagType::from(self)
    }
}
impl super::TagType {
    /// Converts a vector of tags into a vector of serializable TOML representations.
    pub fn from_tags(tags: &[super::TagData]) -> Vec<TagType> {
        tags.iter().map(TagType::from).collect()
    }
}
impl super::TagData {
    /// Converts a vector of tags into a vector of serializable TOML representations.
    pub fn to_toml_vec(tags: &[super::TagData]) -> Vec<TagType> {
        TagType::from_tags(tags)
    }
}
