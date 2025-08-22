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

mod tag_setter;
mod with_tag;
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

    // Add mutable access to the underlying RawProfile for enum variants.
    fn as_raw_profile_mut(&mut self) -> &mut RawProfile {
        match self {
            Profile::Input(p) => &mut p.0,
            Profile::Display(p) => &mut p.0,
            Profile::Output(p) => &mut p.0,
            Profile::DeviceLink(p) => &mut p.0,
            Profile::Abstract(p) => &mut p.0,
            Profile::ColorSpace(p) => &mut p.0,
            Profile::NamedColor(p) => &mut p.0,
            Profile::Spectral(p) => &mut p.0,
            Profile::Raw(p) => p,
        }
    }

    // Read-only getters delegated to RawProfile.
    pub fn version(&self) -> Result<(u8, u8), crate::Error> {
        self.as_raw_profile().version()
    }
    pub fn profile_size(&self) -> usize {
        self.as_raw_profile().profile_size()
    }
    pub fn flags(&self) -> (bool, bool) {
        self.as_raw_profile().flags()
    }
    pub fn data_color_space(&self) -> Option<crate::signatures::ColorSpace> {
        self.as_raw_profile().data_color_space()
    }
    pub fn primary_platform(&self) -> Option<crate::signatures::Platform> {
        self.as_raw_profile().primary_platform()
    }
    pub fn manufacturer(&self) -> crate::signatures::Signature {
        self.as_raw_profile().manufacturer()
    }
    pub fn model(&self) -> crate::signatures::Signature {
        self.as_raw_profile().model()
    }

    // Consuming builders: forward to the matching wrapper and re-wrap.
    pub fn with_version(self, major: u8, minor: u8) -> Result<Self, crate::Error> {
        Ok(match self {
            Profile::Input(p) => Profile::Input(p.with_version(major, minor)?),
            Profile::Display(p) => Profile::Display(p.with_version(major, minor)?),
            Profile::Output(p) => Profile::Output(p.with_version(major, minor)?),
            Profile::DeviceLink(p) => Profile::DeviceLink(p.with_version(major, minor)?),
            Profile::Abstract(p) => Profile::Abstract(p.with_version(major, minor)?),
            Profile::ColorSpace(p) => Profile::ColorSpace(p.with_version(major, minor)?),
            Profile::NamedColor(p) => Profile::NamedColor(p.with_version(major, minor)?),
            Profile::Spectral(p) => Profile::Spectral(p.with_version(major, minor)?),
            Profile::Raw(p) => Profile::Raw(p.with_version(major, minor)?),
        })
    }

    pub fn with_creation_date(self, date: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        match self {
            Profile::Input(p) => Profile::Input(p.with_creation_date(date)),
            Profile::Display(p) => Profile::Display(p.with_creation_date(date)),
            Profile::Output(p) => Profile::Output(p.with_creation_date(date)),
            Profile::DeviceLink(p) => Profile::DeviceLink(p.with_creation_date(date)),
            Profile::Abstract(p) => Profile::Abstract(p.with_creation_date(date)),
            Profile::ColorSpace(p) => Profile::ColorSpace(p.with_creation_date(date)),
            Profile::NamedColor(p) => Profile::NamedColor(p.with_creation_date(date)),
            Profile::Spectral(p) => Profile::Spectral(p.with_creation_date(date)),
            Profile::Raw(p) => Profile::Raw(p.with_creation_date(date)),
        }
    }

    // Consuming builder entry for tags, returning TagSetter bound to this enum.
    pub fn with_tag<S: Into<crate::tag::TagSignature> + Copy>(
        self,
        signature: S,
    ) -> TagSetter<Self, S> {
        TagSetter::new(self, signature)
    }
}

// Helper: render a RawProfile as TOML (used by Display impls).
fn write_toml_from_raw(raw: &RawProfile, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let header = Header::from(raw);
    let tags: IndexMap<String, ParsedTag> = raw
        .tags
        .iter()
        .map(|(sig, entry)| (sig.to_string(), entry.tag.to_parsed()))
        .collect();
    let parsed_profile = ParsedProfile { header, tags };
    match toml::to_string(&parsed_profile) {
        Ok(s) => write!(f, "{s}"),
        Err(_) => Err(fmt::Error),
    }
}

/// A display implementation for `Profile` that serializes the profile to a TOML string.
impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_toml_from_raw(self.as_raw_profile(), f)
    }
}

// Implement Display for all wrapper profile types by delegating to the helper.
macro_rules! impl_display_for_wrappers {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl fmt::Display for $ty {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write_toml_from_raw(&self.0, f)
                }
            }
        )+
    };
}

impl_display_for_wrappers!(
    InputProfile,
    DisplayProfile,
    OutputProfile,
    DeviceLinkProfile,
    AbstractProfile,
    ColorSpaceProfile,
    NamedColorProfile,
    SpectralProfile,
);

/// A fully parsed ICC profile represented in a structured format.
///
/// This is **mainly used for serialization to TOML**, providing a human-readable
/// representation of an ICC profile's header and tags.
/// For all other operations, use the Profile enums (`InputProfile`, `DisplayProfile`, etc.),
/// and use lazily parsed operations on their encapsulated `RawProfile`s directly.
#[derive(Serialize)]
pub struct ParsedProfile {
    #[serde(flatten)]
    pub header: Header,
    #[serde(flatten)]
    pub tags: IndexMap<String, ParsedTag>,
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
