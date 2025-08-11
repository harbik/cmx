use serde::Serialize;

use crate::tag::{
    tag_value::{
        chromaticity::ChromaticityTypeToml, curve::CurveTypeToml, raw::RawTypeToml,
        text_description::TextDescriptionTypeToml, xyz::XYZArrayTypeToml,
    },
    TagTraits, TagValue,
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
/// - Allow seamless conversion from the internal `TagValue` type via `From<&TagValue>` and helper methods
///   like `TagValue::to_toml()` and bulk conversion utilities.
/// - Keep TOML output free of enum tags (thanks to `untagged`), so each tag serializes to the
///   shape of its underlying type.
///
/// Extensibility:
/// - When introducing a new tag kind, add a new variant that wraps its TOML representation and
///   update the `From<&TagValue>` match accordingly.
/// - Because the enum is `untagged`, ensure new variants have unambiguous shapes to avoid
///   potential collisions during (future) deserialization.
///
#[derive(Serialize)]
#[serde(untagged)]
pub enum TagToml {
    XYZArray(XYZArrayTypeToml),
    Curve(CurveTypeToml),
    TextDescription(TextDescriptionTypeToml),
    Chromaticity(ChromaticityTypeToml),
    Raw(RawTypeToml),
    // Fallback when no dedicated TOML format is implemented for a tag variant.
    // Kept minimal and unambiguous for untagged serialization.
    UnsupportedTag {
        unsupported_tag: String,
        type_signature: Option<String>,
    },
    // ... add a variant for every tag type you want to serialize
}

/// Converts a `TagValue`, which is enum collection of encapsulated TagTypes,
/// into a `TagToml` representation. This is used for serializing
/// the tag into a TOML format, using the `Display` trait for `RawProfile`.
///
/// This requires each TagType to implement `From<&TagType> for TagTypeToml`, which
/// converts the raw bytes into a serializable format, specific for each tag.
///
impl From<&TagValue> for TagToml {
    fn from(tag: &TagValue) -> Self {
        match tag {
            // This uses From<&XYZType> for XYZTypeToml in turn, which
            // converts the raw bytes into a serializable format.
            TagValue::XYZArray(xyz) => TagToml::XYZArray(xyz.into()),
            TagValue::Curve(curve) => TagToml::Curve(curve.into()),
            TagValue::TextDescription(text_desc) => TagToml::TextDescription(text_desc.into()),
            TagValue::Chromaticity(chromaticity) => TagToml::Chromaticity(chromaticity.into()),
            // Raw is used for unknown types, and which serializes the raw bytes
            // as a hex string.
            TagValue::Raw(raw) => TagToml::Raw(raw.into()),
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
impl super::TagValue {
    /// Converts the tag into a serializable TOML representation.
    pub fn to_toml(&self) -> TagToml {
        TagToml::from(self)
    }
}
impl super::TagToml {
    /// Converts a vector of tags into a vector of serializable TOML representations.
    pub fn from_tags(tags: &[super::TagValue]) -> Vec<TagToml> {
        tags.iter().map(TagToml::from).collect()
    }
}
impl super::TagValue {
    /// Converts a vector of tags into a vector of serializable TOML representations.
    pub fn to_toml_vec(tags: &[super::TagValue]) -> Vec<TagToml> {
        TagToml::from_tags(tags)
    }
}
