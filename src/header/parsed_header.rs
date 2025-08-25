// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use crate::{
    is_empty_or_none, is_zero, round_to_precision, signatures::Signature, tag::RenderingIntent,
};

/*
// Profile Flags (4-byte field)
pub mod profile_flags {
    pub const EMBEDDED: u32 = 1 << 0;         // Bit 0 - Profile is embedded
    pub const NOT_INDEPENDENT: u32 = 1 << 1;  // Bit 1 - Profile can't be used independently
}
 */

// Device Attributes (8-byte field)
pub mod device_attributes {
    // Byte 7 (LSB)
    pub const TRANSPARENCY: u64 = 1 << 0; // Bit 0 - Transparency flag
    pub const MATTE: u64 = 1 << 1; // Bit 1 - Matte finish flag
    pub const NEGATIVE: u64 = 1 << 2; // Bit 2 - Media polarity is negative
    pub const BLACK_AND_WHITE: u64 = 1 << 3; // Bit 3 - Media is black and white

    #[cfg(feature = "v5")]
    pub const DEVICE_MEDIA: u64 = 1 << 4; // Bit 4 - Device represents media
    #[cfg(feature = "v5")]
    pub const BCS_INTENT: u64 = 1 << 5; // Bit 5 - BCS intent override
}

#[derive(serde::Serialize)]
pub struct Header {
    profile_size: u32,
    cmm: Option<String>,
    version: String,
    #[serde(skip_serializing_if = "is_empty_or_none")]
    device_class: String,
    color_space: Option<String>,
    pcs: String,
    creation_datetime: String,
    primary_platform: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    embedded: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    use_embedded_only: bool,
    manufacturer: Option<String>,
    #[serde(skip_serializing_if = "is_empty_or_none")]
    model: String,
    #[serde(skip_serializing_if = "is_empty_or_none")]
    attributes: String,
    rendering_intent: String,
    pcs_illuminant: (f64, f64, f64),
    creator: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    profile_id: String,
}

impl From<&super::RawProfile> for Header {
    fn from(raw_profile: &super::RawProfile) -> Self {
        let header = raw_profile.header();
        let (major, minor) = raw_profile.version().unwrap();
        let version = format!("{major}.{minor}");
        let (embedded, use_embedded_only) = raw_profile.flags();
        let profile_id_raw = raw_profile.profile_id();
        let profile_id = if profile_id_raw.iter().all(is_zero) {
            String::new()
        } else {
            hex::encode(profile_id_raw)
        };

        let model = if header.model == 0 {
            String::new()
        } else {
            Signature(header.model.get()).to_string()
        };

        let manufacturer = raw_profile.manufacturer().map(|m| m.to_string());

        use self::device_attributes::*;

        // In your header parsing code:
        let attrs_u64 = header.attributes.get();
        let mut attr_strings = Vec::new();

        // Check each flag and add descriptive strings if set
        if attrs_u64 & TRANSPARENCY != 0 {
            attr_strings.push("Transparency");
        }
        if attrs_u64 & MATTE != 0 {
            attr_strings.push("Matte");
        }
        if attrs_u64 & NEGATIVE != 0 {
            attr_strings.push("Negative");
        }
        if attrs_u64 & BLACK_AND_WHITE != 0 {
            attr_strings.push("BlackAndWhite");
        }

        // Add v5 flags if feature enabled
        #[cfg(feature = "v5")]
        {
            if attrs_u64 & DEVICE_MEDIA != 0 {
                attr_strings.push("DeviceMedia");
            }
            if attrs_u64 & BCS_INTENT != 0 {
                attr_strings.push("BCSIntent");
            }
        }

        let attributes_string = if attr_strings.is_empty() {
            String::new()
        } else {
            attr_strings.join(", ")
        };

        Header {
            profile_size: raw_profile.profile_size() as u32,
            cmm: raw_profile.cmm().map(|c| c.to_string()),
            version,
            device_class: raw_profile.device_class().to_string(),
            color_space: raw_profile.data_color_space().map(|c| c.to_string()),
            pcs: raw_profile.pcs().unwrap().to_string(),
            creation_datetime: raw_profile.creation_date().to_string(),
            primary_platform: raw_profile.primary_platform().map(|c| c.to_string()),
            embedded,
            use_embedded_only,
            manufacturer,
            model,
            attributes: attributes_string,
            rendering_intent: RenderingIntent::from(header.rendering_intent.get()).to_string(),
            pcs_illuminant: (
                round_to_precision(f64::from(header.pcs_illuminant[0]), 4),
                round_to_precision(f64::from(header.pcs_illuminant[1]), 4),
                round_to_precision(f64::from(header.pcs_illuminant[2]), 4),
            ),
            creator: raw_profile.creator().map(|c| c.to_string()),
            profile_id,
        }
    }
}
