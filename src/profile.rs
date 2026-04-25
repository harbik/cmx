// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2026, Harbers Bik LLC

use std::fmt;

mod raw_profile;
pub use raw_profile::{ProfileTagRecord, RawProfile};

use indexmap::IndexMap;

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

/// A parsed ICC color profile, represented as one of the eight device classes
/// defined by the ICC specification, plus a catch-all [`Profile::Raw`] variant
/// for profiles whose device-class field is not recognised.
///
/// Use [`Profile::read`] or [`Profile::from_bytes`] to parse an existing profile.
/// Use the device-class constructors (e.g. [`DisplayProfile::new`]) and the builder
/// API when constructing a new profile from scratch.
///
/// # Device classes
///
/// | Variant | ICC class | Typical use |
/// |---|---|---|
/// | [`Profile::Input`] | `scnr` | Scanners, cameras |
/// | [`Profile::Display`] | `mntr` | Monitors, displays |
/// | [`Profile::Output`] | `prtr` | Printers |
/// | [`Profile::DeviceLink`] | `link` | Direct device-to-device conversion |
/// | [`Profile::Abstract`] | `abst` | Abstract color transform |
/// | [`Profile::ColorSpace`] | `spac` | Color space definition |
/// | [`Profile::NamedColor`] | `nmcl` | Named color palettes |
/// | [`Profile::Spectral`] | `cenc` | Spectral data (ICC v5) |
/// | [`Profile::Raw`] | — | Unrecognised device class |
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
    /// Unwrap into the underlying [`RawProfile`], consuming `self`.
    fn into_raw_profile(self) -> RawProfile {
        match self {
            Profile::Input(p) => p.0,
            Profile::Display(p) => p.0,
            Profile::Output(p) => p.0,
            Profile::DeviceLink(p) => p.0,
            Profile::Abstract(p) => p.0,
            Profile::ColorSpace(p) => p.0,
            Profile::NamedColor(p) => p.0,
            Profile::Spectral(p) => p.0,
            Profile::Raw(p) => p,
        }
    }

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

    /// Return a mutable reference to the underlying [`RawProfile`].
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

    // -----------------------------------------------------------------------
    // Read-only accessors — all delegate to the underlying RawProfile.
    // -----------------------------------------------------------------------

    /// Returns the rendering intent stored in the profile header.
    pub fn rendering_intent(&self) -> crate::tag::RenderingIntent {
        self.as_raw_profile().rendering_intent()
    }

    /// Returns the ICC profile version as `(major, minor)`.
    /// Errors if the version stored in the header is not one of the values
    /// recognised by this crate (2.x, 4.x, 5.0).
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
    pub fn manufacturer(&self) -> Option<crate::signatures::Signature> {
        self.as_raw_profile().manufacturer()
    }
    pub fn model(&self) -> crate::signatures::Signature {
        self.as_raw_profile().model()
    }

    /// Parse an ICC profile from a raw byte slice, returning the appropriate
    /// device-class variant.  Unknown device classes are returned as
    /// [`Profile::Raw`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = RawProfile::from_bytes(bytes)?;
        Ok(raw.into_class_profile())
    }

    /// Read an ICC profile from a file on disk, returning the appropriate
    /// device-class variant.
    pub fn read(path: impl AsRef<std::path::Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = RawProfile::read(path)?;
        Ok(raw.into_class_profile())
    }

    /// Serialise the profile to a byte vector ready for writing to disk or
    /// embedding in an image.  Recalculates tag offsets and, if
    /// [`with_profile_id`](RawProfile::with_profile_id) was called, the MD5
    /// profile ID.
    pub fn into_bytes(self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.into_raw_profile().into_bytes()
    }

    /// Serialise the profile and write it to the given path.
    pub fn write(
        self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.into_raw_profile().write(path)
    }

    /// Returns the raw 16-byte profile ID (MD5 checksum of the profile bytes
    /// with the ID field zeroed).  All zeros means no ID has been computed.
    pub fn profile_id(&self) -> [u8; 16] {
        self.as_raw_profile().profile_id()
    }

    pub fn profile_id_as_hex_string(&self) -> String {
        self.as_raw_profile().profile_id_as_hex_string()
    }

    pub fn creation_date(&self) -> Result<chrono::DateTime<chrono::Utc>, crate::Error> {
        self.as_raw_profile().creation_date()
    }
    pub fn pcs_illuminant(&self) -> [f64; 3] {
        self.as_raw_profile().pcs_illuminant()
    }
    pub fn pcs(&self) -> Option<crate::signatures::Pcs> {
        self.as_raw_profile().pcs()
    }
    pub fn cmm(&self) -> Option<crate::signatures::Cmm> {
        self.as_raw_profile().cmm()
    }

    /// Returns a reference to the profile's tag map, keyed by [`crate::tag::TagSignature`].
    ///
    /// Each [`ProfileTagRecord`] holds the tag's offset, size, and parsed [`crate::tag::Tag`] data.
    /// Tags are stored in insertion order, which matches the order they appear in the
    /// original profile's tag table.
    pub fn tags(&self) -> &IndexMap<crate::tag::TagSignature, ProfileTagRecord> {
        self.as_raw_profile().tags()
    }

    /// Returns a mutable reference to the profile's tag map, keyed by [`crate::tag::TagSignature`].
    ///
    /// Allows inserting, replacing, or removing tags directly.  Note that removing
    /// required tags (e.g. `MediaWhitePoint` from a display profile) will produce a
    /// profile that does not conform to the ICC specification.
    pub fn tags_mut(&mut self) -> &mut IndexMap<crate::tag::TagSignature, ProfileTagRecord> {
        self.as_raw_profile_mut().tags_mut()
    }

    // -----------------------------------------------------------------------
    // Consuming builders — forward to the matching device-class wrapper and re-wrap.
    // -----------------------------------------------------------------------

    /// Set the ICC version.  Returns an error for any version not recognised
    /// by this crate.  See [`RawProfile::with_version`] for supported values.
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

    pub fn with_creation_date(self, date: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
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

    /// Begin setting the data for a specific tag, returning a [`TagSetter`]
    /// that provides type-safe `as_*` methods for the chosen tag.
    ///
    /// # Example
    /// ```rust
    /// use cmx::profile::DisplayProfile;
    /// use cmx::tag::tags::MediaWhitePointTag;
    ///
    /// let profile = DisplayProfile::new()
    ///     .with_tag(MediaWhitePointTag)
    ///     .as_xyz_array(|xyz| xyz.set([0.9505, 1.0, 1.0890]));
    /// ```
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
