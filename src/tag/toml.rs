// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::tag::{
    tagdata::{
        chromaticity::Chromaticity, curve::CurveTypeToml, lut8::Lut8TypeToml, measurement::Measurement, multi_localized_unicode::MultiLocalizedUnicodeTypeToml, parametric_curve::ParametricCurveTypeToml, raw::RawTypeToml, s15fixed16array::S15Fixed16ArrayTypeToml, text::TextTypeToml, text_description::TextDescriptionTypeToml, xyz::XYZArrayTypeToml
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
pub enum TagToml {
    Chromaticity(Chromaticity),
    Curve(CurveTypeToml),
    Lut8(Lut8TypeToml),
    Measurement(Measurement),
    MultiLocalizedUnicode(MultiLocalizedUnicodeTypeToml),
    ParametricCurve(ParametricCurveTypeToml),
    S15Fixed16Array(S15Fixed16ArrayTypeToml),
    Text(TextTypeToml),
    TextDescription(TextDescriptionTypeToml),
    XYZArray(XYZArrayTypeToml),

    Raw(RawTypeToml),
    // Fallback when no dedicated TOML format is implemented for a tag variant.
    // Kept minimal and unambiguous for untagged serialization.
    UnsupportedTag {
        unsupported_tag: String,
        type_signature: Option<String>,
    },
    // ... add a variant for every tag type you want to serialize
}

/// Converts a `TagData`, which is enum collection of encapsulated TagTypes,
/// into a `TagToml` representation. This is used for serializing
/// the tag into a TOML format, using the `Display` trait for `RawProfile`.
///
/// This requires each TagType to implement `From<&TagType> for TagTypeToml`, which
/// converts the raw bytes into a serializable format, specific for each tag.
///
impl From<&TagData> for TagToml {
    fn from(tag: &TagData) -> Self {
        match tag {
            TagData::Chromaticity(chromaticity) => TagToml::Chromaticity(chromaticity.into()),
            TagData::Curve(curve) => TagToml::Curve(curve.into()),
            TagData::Lut8(lut8) => TagToml::Lut8(lut8.into()),
            TagData::Measurement(measurement) => {
                TagToml::Measurement(measurement.into())
            }
            TagData::MultiLocalizedUnicode(mluc) => TagToml::MultiLocalizedUnicode(mluc.into()),
            TagData::ParametricCurve(para) => TagToml::ParametricCurve(para.into()),
            TagData::S15Fixed16Array(values) => TagToml::S15Fixed16Array(values.into()),
            TagData::Text(text) => TagToml::Text(text.into()),
            TagData::TextDescription(text_desc) => TagToml::TextDescription(text_desc.into()),
            TagData::XYZArray(xyz) => TagToml::XYZArray(xyz.into()),
            TagData::Raw(raw) => TagToml::Raw(raw.into()),
            // Graceful fallback: don't panic, just emit a small structured note.
            _ => {
                let type_signature = tag
                    .as_slice()
                    .get(0..4)
                    .map(|b| String::from_utf8_lossy(b).to_string());
                TagToml::UnsupportedTag {
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
    pub fn to_toml(&self) -> TagToml {
        TagToml::from(self)
    }
}
impl super::TagToml {
    /// Converts a vector of tags into a vector of serializable TOML representations.
    pub fn from_tags(tags: &[super::TagData]) -> Vec<TagToml> {
        tags.iter().map(TagToml::from).collect()
    }
}
impl super::TagData {
    /// Converts a vector of tags into a vector of serializable TOML representations.
    pub fn to_toml_vec(tags: &[super::TagData]) -> Vec<TagToml> {
        TagToml::from_tags(tags)
    }
}
