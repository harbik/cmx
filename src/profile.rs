// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

mod raw_profile;
use std::fmt;

use indexmap::IndexMap;
pub use raw_profile::RawProfile;

mod input_profile;
pub use input_profile::InputProfile;

mod display_profile;
pub use display_profile::DisplayProfile;

mod output_profile;
pub use output_profile::OutputProfile;

mod device_link_profile;
pub use device_link_profile::DeviceLinkProfile;

mod abstract_profile;
pub use abstract_profile::AbstractProfile;

mod color_space_profile;
pub use color_space_profile::ColorSpaceProfile;

mod named_color_profile;
pub use named_color_profile::NamedColorProfile;

mod spectral_profile;
use serde::Serialize;
pub use spectral_profile::SpectralProfile;

/// delegates methods from the RawProfile to all Profiles.
mod delegate;

mod with_tag;
mod tag_setter;
pub use tag_setter::TagSetter;

mod checksum;
use crate::{header::Header, tag::ParsedTag};

pub use {checksum::md5checksum, checksum::set_profile_id};

#[derive(Debug)]
pub enum Profile {
    Input(InputProfile),
    Display(DisplayProfile),
    Output(OutputProfile),
    DeviceLink(DeviceLinkProfile),
    Abstract(AbstractProfile),
    ColorSpace(ColorSpaceProfile),
    NamedColor(NamedColorProfile),
    Spectral(SpectralProfile),
    Raw(RawProfile),
}

impl Profile {
    fn as_raw_profile(&self) -> &RawProfile {
        match self {
            Profile::Input(p) => &p.0,
            Profile::Display(p) => &p.0,
            Profile::Output(p) => &p.0,
            Profile::DeviceLink(p) => &p.0,
            Profile::Abstract(p) => &p.0,
            Profile::ColorSpace(p) => &p.0,
            Profile::NamedColor(p) => &p.0,
            Profile::Spectral(p) => &p.0,
            Profile::Raw(p) => p,
        }
    }
}


/// A serde-friendly TOML façade for an ICC profile.
///
/// This struct is used to:
/// - Print a profile to TOML (currently used by `RawProfile`'s `Display` impl to write to
///   standard output).
/// - Eventually deserialize a profile from TOML as an alternative to reading a binary ICC file.
///
/// Layout:
/// - `header`: Stored as `IccHeaderToml`, providing a structured view of the 128-byte ICC header.
/// - `tags` (flattened): An `IndexMap` from stringified tag signatures to `TagToml`. The map is
///   flattened into the top level of the TOML document (i.e., each tag appears as its own top-level
///   TOML key next to the `header` table). `IndexMap` preserves insertion order to retain the
///   original tag order for readability and compatibility.
///
/// TagData representation:
/// - Every tag type implements its own `TagDataToml`.
/// - `TagToml` is an enum that encapsulates all tag-type-specific TOML representations, allowing the
///   `tags` map to hold heterogeneous tag values while remaining serializable/deserializable.
///
/// Round-tripping notes:
/// - On serialization, tags are emitted in their insertion order.
#[derive(Serialize)]
pub struct ParsedProfile {
    #[serde(flatten)]
    pub header: Header,
    #[serde(flatten)]
    pub tags: IndexMap<String, ParsedTag>,
}

/// A display implementation for `RawProfile` that serializes the profile to a TOML string.
impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // See header::icc_header_toml.rs for the IccHeaderToml struct
        // and its conversion from RawProfile.
        let header = Header::from(self.as_raw_profile());

        // Convert tags to a flattened IndexMap<String, ParsedTag>, using the tag signature as the
        // key, and converting each RawTag's tag to a ParsdTag using its From<TagData>
        // implementation, which needs to be implemented by each TagType individually.
        // RawTag is a struct that contains the offset, size, and the "TagData" enum,
        // which encapsulates the tag data.
        let tags: IndexMap<String, ParsedTag> = self
            .as_raw_profile()
            .tags
            .iter()
            .map(|(sig, entry)| {
                (sig.to_string(), entry.tag.to_parsed())
            })
            .collect();

        let parsed_profile = ParsedProfile { header, tags };

        // serialize the ParsedProfile to a TOML string.
        match toml::to_string(&parsed_profile) {
            Ok(s) => write!(f, "{s}"),
            Err(_) => Err(fmt::Error),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::profile::RawProfile;

    #[test]
    fn print_rgb() -> Result<(), Box<dyn std::error::Error>> {
        let profile = include_bytes!("../tests/profiles/sRGB.icc");
        let raw_profile = RawProfile::from_bytes(profile).unwrap();
        println!("{}", crate::profile::Profile::Raw(raw_profile));
        Ok(())
    }

    #[test]
    fn print_display_p3() -> Result<(), Box<dyn std::error::Error>> {
        let profile = include_bytes!("../tests/profiles/Display P3.icc");
        let raw_profile = RawProfile::from_bytes(profile).unwrap();
        println!("{}", crate::profile::Profile::Raw(raw_profile));
        Ok(())
    }
}
