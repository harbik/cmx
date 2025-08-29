// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

mod parsed_header;
use num::FromPrimitive;
pub use parsed_header::Header;

use chrono::{DateTime, Datelike, Timelike};
use zerocopy::{
    BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Ref, Unaligned, U16, U32, U64,
};

use crate::{
    error::{Error, HeaderParseError},
    format_hex_with_spaces, is_printable_ascii_bytes,
    profile::RawProfile,
    signatures::{Cmm, ColorSpace, DeviceClass, Pcs, Platform, Signature},
    tag::{GamutCheck, Interpolate, Quality, RenderingIntent},
    S15Fixed16,
};

fn validate_version(major: u8, minor: u8) -> Result<(u8, u8), Error> {
    match (major, minor) {
        (2, 0) => Ok((2, 0)),
        (2, 1) => Ok((2, 1)),
        (2, 2) => Ok((2, 2)),
        (2, 3) => Ok((2, 3)),
        (2, 4) => Ok((2, 4)),
        (4, 0) => Ok((4, 0)),
        (4, 2) => Ok((4, 2)),
        (4, 3) => Ok((4, 3)),
        (4, 4) => Ok((4, 4)),
        (5, 0) => Ok((5, 0)),
        _ => Err(
            HeaderParseError::new(format!("Invalid ICC profile version: V{major}.{minor}")).into(),
        ),
    }
}

#[derive(FromBytes, IntoBytes, Unaligned, KnownLayout, Immutable, Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct HeaderLayout {
    pub profile_size: U32<BigEndian>,
    pub cmm: U32<BigEndian>,
    pub version: U32<BigEndian>,
    pub device_class: U32<BigEndian>,
    pub color_space: U32<BigEndian>,
    pub pcs: U32<BigEndian>,
    pub creation_year: U16<BigEndian>,
    pub creation_month: U16<BigEndian>,
    pub creation_day: U16<BigEndian>,
    pub creation_hours: U16<BigEndian>,
    pub creation_minutes: U16<BigEndian>,
    pub creation_seconds: U16<BigEndian>,
    pub file_signature: U32<BigEndian>,
    pub primary_platform: U32<BigEndian>,
    pub flags: U32<BigEndian>,
    pub manufacturer: U32<BigEndian>,
    pub model: U32<BigEndian>,
    pub attributes: U64<BigEndian>,
    pub rendering_intent: U32<BigEndian>,
    pub pcs_illuminant: [S15Fixed16; 3],
    pub creator: U32<BigEndian>,
    pub profile_id: [u8; 16],
    pub reserved: [u8; 28],
}

impl RawProfile {
    // Returns a reference to the ICC profile header, from an zerocopy overlay.
    // Unwrap justificiation:
    //
    // - The header is a 128-byte array that contains metadata about the profile.
    // - The byte array has already been validated to have a size of 128 bytes,
    //   and to have a valid ICC profile signature.
    pub(crate) fn header(&self) -> Ref<&[u8], HeaderLayout> {
        Ref::<&[u8], HeaderLayout>::from_bytes(self.header.as_slice()).unwrap()
    }

    // Returns a mutual reference to the ICC profile header, from an zerocopy overlay.
    // Unwrap justificiation:
    //
    // - The header is a 128-byte array that contains metadata about the profile.
    // - The byte array has already been validated to have a size of 128 bytes,
    //   and to have a valid ICC profile signature.
    pub(crate) fn header_mut(&mut self) -> &mut HeaderLayout {
        let mut_ref = Ref::<&mut [u8], HeaderLayout>::from_bytes(&mut self.header).unwrap();
        Ref::into_mut(mut_ref)
    }

    /// Returns the size of the profile in bytes, as specified in the header.
    /// This size includes the header, tag table, and all tag data.
    /// It does not include any padding that may be present at the end of the profile.
    pub fn profile_size(&self) -> usize {
        let header = self.header();
        header.profile_size.get() as usize
    }

    /// Updates the size of the profile in bytes. This is not necessarily the same as the size of the
    /// ".icc" file on disk, as the file may contain padding.
    pub fn with_profile_size(mut self, size: usize) -> Self {
        self.header_mut().profile_size = U32::new(size as u32);
        self
    }

    /// Returns the Color Management Module, which is a tag that primarily indicates the tool used
    /// to create the profile, but also is a suggestion for the CMM to use when interpreting the
    /// profile.  This is just a suggestion, and can be ignored if processed by a different CMM. In
    /// rare cases, the profile might contain non-standard tags, which require a specific CMM to
    /// interpret them correctly.  However, such a use is discouraged, as the ICC's intention is to
    /// improve color reproduction quality between devices and media, and not to create
    /// vendor-specific profiles.
    ///
    pub fn cmm(&self) -> Option<Cmm> {
        let header = self.header();
        let tag = Signature(header.cmm.get());
        Cmm::from_u32(tag.0)
    }

    /// Changes, or sets it, when creating a new profile, the Color Management Module (CMM) of the profile.
    pub fn with_cmm(mut self, cmm: Cmm) -> Result<Self, Error> {
        let tag = Signature::from(cmm as u32);
        self.header_mut().cmm = U32::new(tag.0);
        Ok(self)
    }

    /// Returns the version of the ICC profile.
    /// Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let (major, minor) = profile.version().unwrap();
    /// assert_eq!(major, 4);
    /// assert_eq!(minor, 0);
    /// ```
    pub fn version(&self) -> Result<(u8, u8), Error> {
        let header = self.header();
        let version = header.version.get();
        let major = (version >> 24) as u8;
        let minor = ((version >> 20) & 0x0F) as u8;
        validate_version(major, minor)
    }

    /// Sets the version of the ICC profile.
    /// This method allows you to specify the major and minor version numbers.
    ///
    /// # Error:
    ///
    /// - If the version is not one of the valid versions (2.0, 4.0, 4.2, 4.3, or 5.0), it will return an error.
    /// # Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_version(4, 3).unwrap();
    /// let (major, minor) = updated_profile.version().unwrap();
    /// assert_eq!(major, 4);
    /// assert_eq!(minor, 3);
    /// ```
    pub fn with_version(mut self, major: u8, minor: u8) -> Result<Self, Error> {
        let (major, minor) = validate_version(major, minor)?;
        let version_u32 = ((major as u32) << 24) | ((minor as u32) << 20);
        self.header_mut().version = U32::new(version_u32);
        Ok(self)
    }

    /// Returns the device class of the profile, which indicates the type of device the profile is associated with.
    /// Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::DeviceClass};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let device_class = profile.device_class();
    /// assert_eq!(device_class, DeviceClass::Display);
    /// ```
    pub fn device_class(&self) -> DeviceClass {
        let header = self.header();
        let d = header.device_class.get();
        DeviceClass::from_u32(d).unwrap_or_default()
    }

    /// Sets the device class of the profile.
    /// This method allows you to specify the device class using a `Signature`,
    /// such as Signature("mntr"), or using the `DeviceClass` enum: `DeviceClass::DisplayDevice``.
    /// Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::{Signature, DeviceClass}};
    /// use std::str::FromStr;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_device_class(DeviceClass::Display);
    /// let device_class = updated_profile.device_class();
    /// assert_eq!(device_class, DeviceClass::Display);
    /// ```
    pub fn with_device_class(mut self, device_class: DeviceClass) -> Self {
        self.header_mut().device_class = U32::new(device_class as u32);
        self
    }

    /// Returns the color space of the profile, which indicates the color space used by the profile.
    /// Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::ColorSpace};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let color_space = profile.data_color_space().unwrap();
    /// assert_eq!(color_space, ColorSpace::RGB);
    /// ```
    pub fn data_color_space(&self) -> Option<ColorSpace> {
        let header = self.header();
        let ncs = header.color_space.get();
        ColorSpace::from_u32(ncs)
    }

    /// Sets the color space of the profile, which indicates the color space of the data the profile
    /// is associated with, typically RGB, CMYK, or XYZ.
    /// This method allows you to specify the color space using a `Signature`, such as Signature::from_str("RGB "), or
    /// using the `colorspace` enum.
    /// Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::ColorSpace};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_data_color_space(ColorSpace::RGB);
    /// let color_space = updated_profile.data_color_space().unwrap();
    /// assert_eq!(color_space, ColorSpace::RGB);
    /// ```
    pub fn with_data_color_space(mut self, color_space: ColorSpace) -> Self {
        self.header_mut().color_space = U32::new(color_space as u32);
        self
    }

    /// Returns the Profile Connection Space (PCS) of the profile, which indicates the color space used for
    /// the profile connection space.
    ///
    /// # Error:
    /// If the PCS tag is not valid, it will return an error.
    ///
    /// Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Pcs};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let pcs = profile.pcs().unwrap();
    /// assert_eq!(pcs, Pcs::XYZ);
    /// ```
    pub fn pcs(&self) -> Option<Pcs> {
        let header = self.header();
        let pcs = header.pcs.get();
        // TODO: PCS field can be any color space in the device link profile.
        Pcs::from_u32(pcs)
    }

    /// Sets the Profile Connection Space (PCS) of the profile.
    /// Valid PCS values are `Pcs::XYZ`, `Pcs::Lab`, and `Pcs::Luv`.
    /// Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Pcs};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_pcs(Pcs::XYZ);
    /// let pcs = updated_profile.pcs().unwrap();
    /// assert_eq!(pcs, Pcs::XYZ);
    /// ```
    pub fn with_pcs(mut self, pcs: Pcs) -> Self {
        self.header_mut().pcs = U32::new(pcs as u32);
        self
    }

    /// Returns the Profile Connection Space Illuminant of the profile,
    /// which is typically D50 for color profiles.
    pub fn pcs_illuminant(&self) -> [f64; 3] {
        let header = self.header();
        let xyz = header.pcs_illuminant;
        [f64::from(xyz[0]), f64::from(xyz[1]), f64::from(xyz[2])]
    }

    pub fn with_pcs_illuminant(mut self, illuminant: [f64; 3]) -> Self {
        let header = self.header_mut();
        header.pcs_illuminant = [
            S15Fixed16::from(illuminant[0]),
            S15Fixed16::from(illuminant[1]),
            S15Fixed16::from(illuminant[2]),
        ];
        self
    }

    /// Returns the creation date of the profile.
    /// This method extracts the creation date from the profile header and returns it as a `DateTime<chrono::Utc>`.
    ///
    /// # Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// use chrono::{DateTime, Datelike, Utc};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let creation_date = profile.creation_date();
    /// assert_eq!(creation_date.year(), 2017);
    /// assert_eq!(creation_date.month(), 7);
    /// assert_eq!(creation_date.day(), 7);
    /// ```
    pub fn creation_date(&self) -> DateTime<chrono::Utc> {
        let header = self.header();
        let year = header.creation_year.get() as i32;
        let month = header.creation_month.get() as u32;
        let day = header.creation_day.get() as u32;
        let hour = header.creation_hours.get() as u32;
        let minute = header.creation_minutes.get() as u32;
        let second = header.creation_seconds.get() as u32;
        let naive = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|d| d.and_hms_opt(hour, minute, second))
            .unwrap_or_else(|| panic!("Invalid date in ICC header: {year}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}"));
        // .expect(format!("Invalid date in ICC header: {year}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}").as_ref());
        DateTime::from_naive_utc_and_offset(naive, chrono::Utc)
    }

    /// Sets the creation date of the profile.
    /// This method allows you to specify the creation date using a `DateTime<chrono::Utc>`.
    /// If you pass `None`, it will set the creation date to the current date and time.
    ///
    pub fn with_creation_date(mut self, date: impl Into<DateTime<chrono::Utc>>) -> Self {
        let utc_date = date.into();
        let naive = utc_date.naive_utc();
        let header = self.header_mut();
        header.creation_year = U16::new(naive.year() as u16);
        header.creation_month = U16::new(naive.month() as u16);
        header.creation_day = U16::new(naive.day() as u16);
        header.creation_hours = U16::new(naive.hour() as u16);
        header.creation_minutes = U16::new(naive.minute() as u16);
        header.creation_seconds = U16::new(naive.second() as u16);
        self
    }

    pub fn with_now_as_creation_date(self) -> Self {
        let now = chrono::Utc::now().with_nanosecond(0).unwrap();
        self.with_creation_date(now)
    }

    /// Checks if the file signature of the profile is valid.
    /// This method verifies that the file signature matches the expected value for an ICC profile.
    /// If the signature is invalid, it returns an error.
    /// Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// profile.check_file_signature().expect("Not a valid ICC file"); // should not return an error
    /// ```
    pub fn check_file_signature(&self) -> Result<(), Error> {
        let header = self.header();
        let signature = header.file_signature.get();
        if signature != 0x61637370 {
            // 'acsp' in ASCII
            return Err(Error::InvalidICCProfile);
        }
        Ok(())
    }

    /// Sets the file signature of the profile to a valid value.
    /// This method updates the file signature to the expected value for an ICC profile.
    /// Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_valid_file_signature();
    /// assert!(updated_profile.check_file_signature().is_ok()); // should not return an error
    /// ```
    pub fn with_valid_file_signature(mut self) -> Self {
        self.header_mut().file_signature = U32::new(0x61637370); // 'acsp' in ASCII
        self
    }

    /// Returns the "primary platform" of the profile, the operating system for which the profile is primarily intended.
    /// This is represented by a `Platform` enum, e.g.:
    ///
    /// - `Platform::Apple`: For Apple platforms (macOS, iOS, etc.)
    /// - `Platform::Microsoft`: For Microsoft platforms (Windows, etc.)
    ///     
    /// Most Color Management Modules (CMMs) will ignore this tag, but it can be useful for applications that need to.
    ///
    /// # Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Platform};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let platform = profile.primary_platform().unwrap();
    /// assert_eq!(platform, Platform::Apple); // or whatever the primary platform is for the profile
    /// ```
    /// # Notes:
    /// - The primary platform is not a strict requirement for ICC profiles, and many profiles may not have this tag set.
    /// - If the platform is not set, it will return a default value of `Platform::All`, with Signature "all ".
    ///
    pub fn primary_platform(&self) -> Option<Platform> {
        let header = self.header();
        let p = header.primary_platform.get();
        Platform::from_u32(p)
    }

    /// Sets the primary platform of the profile.
    /// This method allows you to specify the primary platform using a `Platform` enum.
    /// The value is reset  you pass `Platform::All`.
    /// # Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Platform};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_primary_platform(Platform::Microsoft);
    /// let platform = updated_profile.primary_platform().unwrap();
    /// assert_eq!(platform, Platform::Microsoft);
    /// ```
    pub fn with_primary_platform(mut self, platform: Platform) -> Self {
        self.header_mut().primary_platform.set(platform as u32);
        self
    }

    /// Returns the flags of the profile, which indicate whether the profile is embedded and whether it should be used only when embedded.
    /// The flags are represented as a bitmask:
    /// - Bit 0 (0x00000001): Indicates if the profile is embedded.
    /// - Bit 1 (0x00000002): Indicates if the profile should be used only when embedded.
    /// # Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let (embedded, use_embedded_only) = profile.flags();
    /// assert!(!embedded); // Not an embeded profile, as read from a file
    /// assert!(!use_embedded_only); // Not set to use embedded only
    /// ```
    pub fn flags(&self) -> (bool, bool) {
        let header = self.header();
        let flags = header.flags.get();
        let embedded = (flags & 0x00000001) != 0; // bit 0
        let use_embedded_only = (flags & 0x00000002) != 0; // bit 1
        (embedded, use_embedded_only)
    }

    /// Clears the flags of the profile.
    pub fn with_cleared_flags(mut self) -> Self {
        self.header_mut().flags = U32::new(0x0);
        self
    }

    /// Sets the flags of the profile.
    /// This method allows you to specify whether the profile is embedded and whether it should be used only when embedded.
    /// The flags are represented as a bitmask:
    ///
    /// - Bit 0 (0x00000001): Indicates if the profile is embedded.
    /// - Bit 1 (0x00000002): Indicates if the profile should be used only when embedded.
    ///
    /// # Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_flags(true,false).unwrap();
    /// let (embedded, use_embedded_only) = updated_profile.flags();
    /// assert!(embedded); // Now set to embedded
    /// assert!(!use_embedded_only); // Not set to use embedded only
    ///  ```
    /// # Note:
    /// This bit can also be used as an indication, by the CMM, that the profile should not be copied and used
    /// for your own purposes, in addition to the copyright and license information in contained in the `CopyrightTag` "cprt" tag .
    pub fn with_flags(mut self, embedded: bool, use_embedded_only: bool) -> Result<Self, Error> {
        let mut flags = 0x0;
        if embedded {
            flags |= 0x00000001; // set bit 0
        }
        if use_embedded_only {
            flags |= 0x00000002; // set bit 1
        }
        self.header_mut().flags = U32::new(flags);
        Ok(self)
    }

    /// Gets the Apple (ColorSync) flags of the profile.
    /// These flags are defined by Apple and use the vendor specific bits in the header's `flags` field.
    /// ColorSync uses bits 16 through 19 for a quality and interpolation hint, and bit 19 as a hint
    /// that no gamut checking is required.
    /// Default values, with all bits  set to 0, are `normal` for quality, `true` for interpolation, and `true`
    /// for gamut checking.
    ///
    /// - Bit 16 and 17: Specifies a quality hint, represented in this library by the QualityHint enum,
    ///   which can be set using the `with_quality_hint` method, with values of `draft`, `normal`, and `high`.
    ///   Default is `normal`.
    /// - Bit 18: Indicates if interpolation is required. Default is `true`, meaning interpolation is recommended.
    /// - Bit 19: Indicates if gamut checking is required. Default is `true`, meaning gamut checking is recommended.
    ///
    /// # Note:
    /// These are Apple-specific flags, and are not part of the ICC specification.
    /// They are used by Apple's ColorSync to optimize color management.
    /// They can only be set when the `cmm` field is set to `Cmm::Apple`.
    ///
    /// # Example:
    ///  ```rust
    /// use cmx::profile::RawProfile;
    /// use cmx::tag::{Quality, Interpolate, GamutCheck};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let (quality, interpolate, gamut_check) = profile.apple_flags();
    /// assert_eq!(quality, Quality::Normal);
    /// assert_eq!(interpolate, Interpolate::True);
    /// assert_eq!(gamut_check, GamutCheck::True);
    /// ```
    pub fn apple_flags(&self) -> (Quality, Interpolate, GamutCheck) {
        let header = self.header();
        let flags = header.flags.get();
        let quality = ((flags >> 16) & 0x03) as u8; // bits 16 and 17
        let interpolate = ((flags >> 18) & 0x01) as u8; // bit 18
        let gamut_check = ((flags >> 19) & 0x01) as u8; // bit 19
        let quality_hint = match quality {
            0 => Quality::Normal,
            1 => Quality::Draft,
            2 => Quality::High,
            _ => Quality::Normal, // default to normal if unknown
        };
        let interpolate_hint = match interpolate {
            0 => Interpolate::True,
            1 => Interpolate::False,
            _ => Interpolate::True, // default to true if unknown
        };
        let gamut_check_hint = match gamut_check {
            0 => GamutCheck::True,
            1 => GamutCheck::False,
            _ => GamutCheck::True, // default to true if unknown
        };
        (quality_hint, interpolate_hint, gamut_check_hint)
    }

    /// Sets the Apple (ColorSync) flags of the profile.
    /// These flags are defined by Apple and use the vendor specific bits in the header's `flags` field.
    /// ColorSync uses bits 16 through 19 for a quality and interpolation hint, and bit 19 as a hint
    /// that no gamut checking is required.
    /// Default values, with all bits  set to 0, are `normal` for quality, `true` for interpolation, and `true`
    /// for gamut checking. By setting interpolation and gamut checking to false, rendering can be faster.
    ///
    /// - Bit 16 and 17: Specifies a quality hint, represented in this library by the QualityHint enum,
    ///   which can be set using the `with_quality_hint` method, with values of `draft`, `normal`, and `high`.
    ///   Default is `normal`.
    /// - Bit 18: Indicates if interpolation is required. Default is `true`, meaning interpolation is recommended.
    /// - Bit 19: Indicates if gamut checking is required. Default is `true`, meaning gamut checking is recommended.
    ///
    /// # Notes:
    /// - These are Apple-specific flags, and are not part of the ICC specification.
    ///   They are used by Apple's ColorSync to optimize color management.
    ///   They can only be set when the `cmm` field is set to `Cmm::Apple`.
    /// - If these flags are set, the CMM field will be set to 'APPL', as ColorSync
    ///   will ignore these flags if the CMM is not Apple.
    ///
    /// # Error:
    /// If the `cmm` field is not set to `Cmm::Apple`, this method will return an error.
    ///
    ///  # Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// use cmx::tag::{Quality, Interpolate, GamutCheck};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_apple_flags(Quality::High, Interpolate::False, GamutCheck::False);
    /// let (quality, interpolate, gamut_check) = updated_profile.apple_flags();
    /// assert_eq!(quality, Quality::High);
    /// assert_eq!(interpolate, Interpolate::False);
    /// assert_eq!(gamut_check, GamutCheck::False);
    /// ```
    pub fn with_apple_flags(
        mut self,
        quality: Quality,
        interpolate: Interpolate,
        gamut_check: GamutCheck,
    ) -> Self {
        let mut flags = self.header().flags.get();
        // Set the quality hint bits (16 and 17)
        flags |= (quality as u32) << 16; // set bits 16 and 17
                                         // Set the interpolation hint bit (18)
        flags |= (interpolate as u32) << 18; // set bit 18
                                             // Set the gamut check hint bit (19)
        flags |= (gamut_check as u32) << 19; // set bit 19
        let apple_cmm = Cmm::Apple as u32;
        self.header_mut().cmm.set(apple_cmm);
        self.header_mut().flags = U32::new(flags);
        self
    }

    /// Returns the manufacturer of the profile, which is a tag that indicates the company or organization that created the profile.
    /// The manufacturer tag is a 4-character string, such as "APPL" for Apple, "MSFT" for Microsoft, etc.
    /// # Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Signature};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let manufacturer = profile.manufacturer().unwrap();
    /// assert_eq!(manufacturer.to_string(), "APPL"); // or whatever the manufacturer is for the profile
    /// ```
    pub fn manufacturer(&self) -> Option<Signature> {
        let header = self.header();
        let m = header.manufacturer.get();
        let sig = Signature(m);
        if is_printable_ascii_bytes(sig.to_string().as_bytes()) {
            Some(sig)
        } else {
            None
        }
    }

    /// Sets the manufacturer of the profile.
    /// This method allows you to specify the manufacturer using a `Signature`, such as Signature::from_str("APPL"), or
    /// using a string that can be parsed into a `Signature`.
    /// If you pass `None`, it will set the manufacturer to a default value of `Signature(0)`.
    /// # Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Signature};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_manufacturer("TEST");
    /// let manufacturer = updated_profile.manufacturer().unwrap();
    /// assert_eq!(manufacturer.to_string(), "TEST");
    /// ```
    /// # Notes:
    /// - For a full list of manufacturers tag signatures, see the [ICC Manufacturer Registry](https://www.color.org/signatureRegistry/index.xalter).
    pub fn with_manufacturer(mut self, manufacturer: &str) -> Self {
        let manufacturer: Signature = manufacturer
            .parse()
            .unwrap_or_else(|_| Signature::default()); // Default to Signature(0) if parsing fails
        self.header_mut().manufacturer = U32::new(manufacturer.0);
        self
    }

    /// Returns the model of the profile, which is a tag that indicates the specific model of the device or software that created the profile.
    ///
    /// # Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Signature};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let model = profile.model();
    /// assert_eq!(model, Signature::default()); // or whatever the model is for the profile
    /// ```
    pub fn model(&self) -> Signature {
        let header = self.header();
        let m = header.model.get();
        Signature(m)
    }

    /// Sets the model of the profile.
    /// This method allows you to specify the model using a `Signature`, such as Signature::from_str("abcd"), or
    /// using a string that can be parsed into a `Signature`.
    /// If you pass `None`, it will set the model to a default value of `Signature(0)`.
    /// # Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Signature};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let test_tag: Signature = "abcd".parse().unwrap();
    /// let updated_profile = profile.with_model(Some(test_tag));
    /// let model = updated_profile.model();
    /// assert_eq!(model.to_string(), "abcd");
    /// ```
    pub fn with_model(mut self, model: Option<Signature>) -> Self {
        let tag = model.unwrap_or_default();
        self.header_mut().model = U32::new(tag.0);
        self
    }

    /// Returns the attributes of the profile, which are a bitmask indicating various properties of the profile.
    /// The attributes are represented as a 64-bit unsigned integer, where each bit represents a different attribute.
    /// # Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let attributes = profile.attributes();
    /// assert_eq!(attributes, 0); // or whatever the attributes are for the profile
    /// ```
    pub fn attributes(&self) -> u64 {
        let header = self.header();
        header.attributes.get()
    }

    /// Sets the attributes of the profile.
    /// This method allows you to specify the attributes using a 64-bit unsigned integer.
    /// # Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_attributes(0x0000000000000001); // Set the first attribute
    /// let attributes = updated_profile.attributes();
    /// assert_eq!(attributes, 0x0000000000000001); // or whatever the attributes are for the profile
    /// ```
    pub fn with_attributes(mut self, attributes: u64) -> Self {
        self.header_mut().attributes = U64::new(attributes);
        self
    }

    /// Returns the rendering intent of the profile, which indicates how colors should be rendered when converting between color spaces.
    /// The rendering intent is represented as a `Signature`, such as "perceptual", "relative colorimetric", "saturation", or "absolute colorimetric".
    /// # Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Signature, tag::RenderingIntent};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let rendering_intent = profile.rendering_intent();
    /// assert_eq!(rendering_intent, RenderingIntent::Perceptual); // or whatever the rendering intent is for the profile
    /// ```
    pub fn rendering_intent(&self) -> RenderingIntent {
        let header = self.header();
        let ri = header.rendering_intent.get();
        ri.into()
    }
    /// Sets the rendering intent of the profile.
    /// This method allows you to specify the rendering intent using a `Signature`, such as Signature::from_str("perceptual"), or
    /// using a string that can be parsed into a `Signature`.
    /// If you pass `None`, it will set the rendering intent to a default value of `Signature(0)`.
    /// # Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Signature, tag::RenderingIntent};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_rendering_intent(RenderingIntent::Perceptual);
    /// let rendering_intent = updated_profile.rendering_intent();
    /// assert_eq!(rendering_intent, RenderingIntent::Perceptual);
    /// ```
    pub fn with_rendering_intent(mut self, rendering_intent: RenderingIntent) -> Self {
        self.header_mut().rendering_intent = U32::new(rendering_intent as u32);
        self
    }

    pub fn creator(&self) -> Option<Signature> {
        let header = self.header();
        let c = header.creator.get();
        if c == 0 {
            None
        } else {
            Some(Signature(c))
        }
    }

    /// Sets the creator of the profile.
    /// This method allows you to specify the creator using a `Signature`, such as Signature::from_str("APPL"), or
    /// using a string that can be parsed into a `Signature`.
    /// If you pass `None`, it will set the creator to a default value of `Signature(0)`.
    /// # Example:
    /// ```rust
    /// use cmx::{profile::RawProfile, signatures::Signature};
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();  
    /// let updated_profile = profile.with_creator("TEST");
    ///     
    /// let creator = updated_profile.creator();
    /// assert_eq!(creator.unwrap().to_string(), "TEST");
    /// ```
    /// # Notes:
    /// - The creator tag is not a strict requirement for ICC profiles, and many profiles may not have this tag set.
    /// - If the creator is not set, it will return `None`.
    pub fn with_creator(mut self, creator: &str) -> Self {
        let signature: Signature = creator.parse().unwrap_or_default();
        self.header_mut().creator = U32::new(signature.0);
        self
    }

    /// Returns the profile ID of the profile, which is a unique identifier for the profile.
    /// The profile ID is a 16-byte array that is typically used to identify the profile in a unique way.
    /// # Example:
    /// ```rust
    /// use cmx::profile::RawProfile;
    /// let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
    /// let profile_id = profile.profile_id();
    /// assert_eq!(profile_id, [202, 26, 149, 130, 37, 127, 16, 77, 56, 153, 19, 213, 209, 234, 21, 130]); // or whatever the profile ID is for the profile
    /// ```
    pub fn profile_id(&self) -> [u8; 16] {
        let header = self.header();
        header.profile_id
    }

    pub fn profile_id_as_hex_string(&self) -> String {
        let profile_id = self.profile_id();
        format_hex_with_spaces(&profile_id)
    }

    /// Clears the profile ID of the profile, and indicates that the profile ID should not be included when creating a new profile.
    /// It is also used while calculating the profile ID.
    pub fn without_profile_id(mut self) -> Self {
        self.header_mut().profile_id = [0u8; 16];
        self
    }

    /// Request the profile ID to be included when creating a new profile.
    /// It will be calculated and set in the to_bytes() profile method, just before
    /// writing or embedding the profile.
    pub fn with_profile_id(mut self) -> Self {
        self.header_mut().profile_id = 1u128.to_be_bytes();
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{profile::RawProfile, signatures::Signature};

    #[test]
    fn test_header() {
        let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
        let header = profile.header();
        let mfg = header.manufacturer.get();
        let mfg_str = Signature(mfg).to_string();
        assert_eq!(mfg_str, "APPL");
    }

    #[test]
    fn test_set_manufacturer() {
        let profile = RawProfile::read("tests/profiles/Display P3.icc").unwrap();
        let updated_profile = profile.with_manufacturer("TEST");
        let mfg_new = updated_profile.manufacturer();
        assert_eq!(mfg_new.unwrap().to_string(), "TEST");
    }
}
