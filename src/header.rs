use chrono::DateTime;
use chrono::{Datelike, Timelike};
use zerocopy::{
    BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Ref, Unaligned, U16, U32, U64,
};

use crate::profile::Profile;

use crate::error::Error;
use crate::tags::{ColorSpace, Platform};
use crate::tags::Cmm;
use crate::tags::DeviceClass;
use crate::tags::Pcs;
use crate::tags::Tag;
        
fn validate_version(major: u8, minor: u8) -> Result<(u8, u8), Error> {
    match (major, minor) {
        (2, 0) => Ok((2, 0)),
        (4, 0) => Ok((4, 0)),
        (4, 2) => Ok((4, 2)),
        (4, 3) => Ok((4, 3)),
        (5, 0) => Ok((5, 0)),
        _ => Err(Error::HeaderParseError("Invalid ICC profile version".to_string())),
    }
}

#[derive(FromBytes, IntoBytes, Unaligned, KnownLayout, Immutable, Debug, Clone, Copy)]
#[repr(C)]
pub struct IccHeader {
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
    pub pcs_illuminant: [u8; 12],
    pub creator: U32<BigEndian>,
    pub profile_id: [u8; 16],
    pub reserved: [u8; 28],
}

impl Profile {
    pub fn header(&self) -> Result<Ref<&[u8], IccHeader>, Error> {
        Ref::<&[u8], IccHeader>::from_bytes(self.header.as_slice())
            .map_err(|e| Error::HeaderParseError(e.to_string()))
    }

    pub fn header_mut(&mut self) -> Result<&mut IccHeader, Error> {
        let mut_ref = Ref::<&mut [u8], IccHeader>::from_bytes(&mut self.header)
            .map_err(|e| Error::HeaderParseError(e.to_string()))?;
        Ok(Ref::into_mut(mut_ref))
    }

    /// Returns the size of the profile in bytes, as specified in the header.
    /// This size includes the header, tag table, and all tag data.
    /// It does not include any padding that may be present at the end of the profile.
    pub fn profile_size(&self) -> usize {
        let header = self.header().unwrap();
        header.profile_size.get() as usize
    }

    /// Updates the size of the profile in bytes. This is not necessarily the same as the size of the
    /// ".icc" file on disk, as the file may contain padding.
    pub fn with_profile_size(mut self, size: usize) -> Result<Self, Error> {
        self.header_mut()?.profile_size = U32::new(size as u32);
        Ok(self)
    }

    /// Returns the version of the ICC profile.
    /// Example:
    /// ```rust
    /// use cmx::profile::Profile;
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let (major, minor) = profile.version().unwrap();
    /// assert_eq!(major, 4);
    /// assert_eq!(minor, 0);
    /// ```
    pub fn version(&self) -> Result<(u8,u8), Error> {
        let header = self.header()?;
        let version = header.version.get();
        dbg!(version);
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
    /// use cmx::profile::Profile;
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_version(4, 3).unwrap();
    /// let (major, minor) = updated_profile.version().unwrap();
    /// assert_eq!(major, 4);
    /// assert_eq!(minor, 3);
    /// ```
    pub fn with_version(mut self, major: u8, minor: u8) -> Result<Self, Error> {
        let (major, minor) = validate_version(major, minor)?;
        let version_u32 = ((major as u32) << 24) | ((minor as u32) << 20);
        self.header_mut()?.version = U32::new(version_u32);
        Ok(self)
    }

    /// Returns the device class of the profile, which indicates the type of device the profile is associated with.
    /// Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::DeviceClass};
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let device_class = profile.device_class().unwrap();
    /// assert_eq!(device_class, DeviceClass::DisplayDevice);
    /// ```
    pub fn device_class(&self) -> Result<DeviceClass, Error> {
        let header = self.header()?;
        let d = header.device_class.get();
        let device_class = DeviceClass::new(Tag(d));
        Ok(device_class)
    }

    /// Sets the device class of the profile.
    /// This method allows you to specify the device class using a `Tag`,
    /// such as Tag("mntr"), or using the `DeviceClass` enum: `DeviceClass::DisplayDevice``.
    /// Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::{Tag, DeviceClass}};
    /// use std::str::FromStr;
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_device_class(Tag::from_str("mntr").unwrap()).unwrap();
    /// let device_class = updated_profile.device_class().unwrap();
    /// assert_eq!(device_class, DeviceClass::DisplayDevice);
    /// ```
    pub fn with_device_class(mut self, device_class: impl Into<Tag>) -> Result<Self, Error> {
        self.header_mut()?.device_class = U32::new(device_class.into().0);
        Ok(self)
    }

    /// Returns the color space of the profile, which indicates the color space used by the profile.
    /// Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::ColorSpace};
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let color_space = profile.color_space().unwrap();
    /// assert_eq!(color_space, ColorSpace::RGB);
    /// ```
    pub fn color_space(&self) -> Result<ColorSpace, Error> {
        let header = self.header()?;
        let ncs = header.color_space.get();
        Ok(ColorSpace::new(Tag(ncs)))
    }

    /// Sets the color space of the profile.
    /// This method allows you to specify the color space using a `Tag`, such as Tag::from_str("RGB "), or
    /// using the `colorspace` enum.
    /// Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::ColorSpace};
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_color_space(ColorSpace::RGB).unwrap();
    /// let color_space = updated_profile.color_space().unwrap();
    /// assert_eq!(color_space, ColorSpace::RGB);
    ///
    /// // or, using a Tag directly:
    /// use cmx::tags::Tag;
    /// let xyz_tag: Tag = "XYZ".parse().unwrap();
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_color_space(xyz_tag).unwrap();
    /// let color_space = updated_profile.color_space().unwrap();
    /// assert_eq!(color_space, ColorSpace::XYZ);
    /// ```
    pub fn with_color_space(mut self, color_space: impl Into<Tag>) -> Result<Self, Error> {
        self.header_mut()?.color_space = U32::new(color_space.into().0);
        Ok(self)
    }

    /// Returns the Profile Connection Space (PCS) of the profile, which indicates the color space used for
    /// the profile connection space.
    ///
    /// Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::Pcs};
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let pcs = profile.pcs().unwrap();
    /// assert_eq!(pcs, Pcs::XYZ);
    /// ```
    pub fn pcs(&self) -> Result<Pcs, Error> {
        let header = self.header()?;
        let pcs = header.pcs.get();
        Ok(Pcs::new(Tag(pcs))?)
    }

    /// Sets the Profile Connection Space (PCS) of the profile.
    /// This method allows you to specify the PCS using a `Tag`, such as Tag::from_str("XYZ "), or
    /// using the `Pcs` enum.
    /// Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::Pcs};
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_pcs(Pcs::XYZ).unwrap();
    /// let pcs = updated_profile.pcs().unwrap();
    /// assert_eq!(pcs, Pcs::XYZ);
    pub fn with_pcs(mut self, pcs: impl Into<Tag>) -> Result<Self, Error> {
        self.header_mut()?.pcs = U32::new(pcs.into().0);
        Ok(self)
    }

    pub fn creation_date(&self) -> Result<DateTime<chrono::Utc>, Error> {
        let header = self.header()?;
        let year = header.creation_year.get() as i32;
        let month = header.creation_month.get() as u32;
        let day = header.creation_day.get() as u32;
        let hour = header.creation_hours.get() as u32;
        let minute = header.creation_minutes.get() as u32;
        let second = header.creation_seconds.get() as u32;
        let naive = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|d| d.and_hms_opt(hour, minute, second))
            .ok_or_else(|| Error::HeaderParseError("Invalid date in ICC header".to_string()))?;
        Ok(DateTime::from_naive_utc_and_offset(naive, chrono::Utc))
    }

    /// Sets the creation date of the profile.
    /// This method allows you to specify the creation date using a `DateTime<chrono::Utc>`.
    /// Example:
    /// ```rust
    /// use cmx::profile::Profile;
    /// use chrono::{DateTime, Utc, Timelike};
    ///
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let creation_date = Utc::now().with_nanosecond(0).unwrap();
    /// let updated_profile = profile.with_creation_date(creation_date).unwrap();
    /// let date = updated_profile.creation_date().unwrap().with_nanosecond(0).unwrap();
    /// assert_eq!(date, creation_date);
    /// ```
    pub fn with_creation_date(
        mut self,
        date: DateTime<chrono::Utc>,
    ) -> Result<Self, Error> {
        let naive = date.naive_utc();
        let header = self.header_mut()?;
        header.creation_year = U16::new(naive.year() as u16);
        header.creation_month = U16::new(naive.month() as u16);
        header.creation_day = U16::new(naive.day() as u16);
        header.creation_hours = U16::new(naive.hour() as u16);
        header.creation_minutes = U16::new(naive.minute() as u16);
        header.creation_seconds = U16::new(naive.second() as u16);
        Ok(self)
    }

    /// Checks if the file signature of the profile is valid.
    /// This method verifies that the file signature matches the expected value for an ICC profile.
    /// If the signature is invalid, it returns an error.
    /// Example:
    /// ```rust
    /// use cmx::profile::Profile;
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// profile.check_file_signature().unwrap(); // should not return an error
    /// ```
    pub fn check_file_signature(&self) -> Result<(), Error> {
        let header = self.header()?;
        let signature = header.file_signature.get();
        if signature != 0x61637370 { // 'acsp' in ASCII
            return Err(Error::InvalidPcsTag(Tag(signature)));
        }
        Ok(())
    }

    /// Sets the file signature of the profile to a valid value.
    /// This method updates the file signature to the expected value for an ICC profile.
    /// Example:
    /// ```rust
    /// use cmx::profile::Profile;
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_valid_file_signature();
    /// assert!(updated_profile.check_file_signature().is_ok()); // should not return an error
    /// ```
    pub fn with_valid_file_signature(mut self) -> Self {
        self.header_mut().unwrap().file_signature = U32::new(0x61637370); // 'acsp' in ASCII
        self
    }

    /// Returns the primary platform of the profile, which indicates the platform for which the profile is intended.
    /// It is used for informational purposes and for compatibility tracking.
    /// Most Color Management Modules (CMMs) will ignore this tag, but it can be useful for applications that need to.
    ///
    /// # Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::Platform};
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let platform = profile.primary_platform();
    /// assert_eq!(platform, Platform::Apple); // or whatever the primary platform is for the profile
    /// ```
    /// # Notes:
    /// - The primary platform is not a strict requirement for ICC profiles, and many profiles may not have this tag set.
    /// - If the platform is not set, it will return a default value of `Platform::All`, with Tag "all ".
    /// 
    pub fn primary_platform(&self) -> Platform {
        let header = self.header().unwrap();
        let p = header.primary_platform.get();
        Platform::new(Tag(p))
    }

    /// Sets the primary platform of the profile.
    /// This method allows you to specify the primary platform using a `Platform` enum.
    /// If you pass the platform as `None`, it will set the primary platform to `Platform::None`,
    /// which will use 0 as the tag value.
    /// # Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::Platform};
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_primary_platform(Platform::Microsoft).unwrap();
    /// let platform = updated_profile.primary_platform();
    /// assert_eq!(platform, Platform::Microsoft);
    /// ```
    pub fn with_primary_platform(mut self, platform: Option<Platform>) -> Result<Self, Error> {
        let platform = platform.unwrap_or(Platform::None);
        let tag = Tag::from(platform);
        self.header_mut()?.primary_platform = U32::new(tag.0);
        Ok(self)
    }

    /// Returns the flags of the profile, which indicate whether the profile is embedded and whether it should be used only when embedded.
    /// The flags are represented as a bitmask:
    /// - Bit 0 (0x00000001): Indicates if the profile is embedded.
    /// - Bit 1 (0x00000002): Indicates if the profile should be used only when embedded.
    /// # Example:
    /// ```rust
    /// use cmx::profile::Profile;
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let (embedded, use_embedded_only) = profile.flags();
    /// assert!(!embedded); // Not an embeded profile, as read from a file
    /// assert!(!use_embedded_only); // Not set to use embedded only
    /// ```
    pub fn flags(&self) -> (bool, bool) {
        let header = self.header().unwrap();
        let flags = header.flags.get();
        let embedded = (flags & 0x00000001) != 0; // bit 0
        let use_embedded_only = (flags & 0x00000002) != 0; // bit 1
        (embedded, use_embedded_only)
    }
    /// Sets the flags of the profile.
    /// This method allows you to specify whether the profile is embedded and whether it should be used only when embedded.
    /// The flags are represented as a bitmask:
    /// - Bit 0 (0x00000001): Indicates if the profile is embedded.
    /// - Bit 1 (0x00000002): Indicates if the profile should be used only when embedded.
    pub fn with_flags(mut self, embedded: bool, use_embedded_only: bool) -> Result<Self, Error> {
        let mut flags = 0x0;
        if embedded {
            flags |= 0x00000001; // set bit 0
        }
        if use_embedded_only {
            flags |= 0x00000002; // set bit 1
        }
        self.header_mut().unwrap().flags = U32::new(flags);
        Ok(self)
    }

    /// Returns the manufacturer of the profile, which is a tag that indicates the company or organization that created the profile.
    /// The manufacturer tag is a 4-character string, such as "APPL" for Apple, "MSFT" for Microsoft, etc.
    /// # Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::Tag};
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let manufacturer = profile.manufacturer().unwrap();
    /// assert_eq!(manufacturer.to_string(), "APPL"); // or whatever the manufacturer is for the profile
    /// ```
    pub fn manufacturer(&self) -> Result<Tag, Error> {
        let header = self.header()?;
        let m = header.manufacturer.get();
        Ok(Tag(m))
    }

     /// Sets the manufacturer of the profile.
     /// This method allows you to specify the manufacturer using a `Tag`, such as Tag::from_str("APPL"), or
     /// using a string that can be parsed into a `Tag`.
    /// If you pass `None`, it will set the manufacturer to a default value of `Tag(0)`.
    /// # Example:
    /// ```rust
    /// use cmx::{profile::Profile, tags::Tag};
    /// let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
    /// let updated_profile = profile.with_manufacturer(Tag::from_str("TEST").unwrap()).unwrap();
    /// let manufacturer = updated_profile.manufacturer().unwrap();
    /// assert_eq!(manufacturer.to_string(), "TEST");
    /// ```
     pub fn with_manufacturer(mut self, manufacturer: Option<impl Into<Tag>>) -> Self {
        let manufacturer = if let Some(m) = manufacturer {
            m.into()
        } else {
            Tag(0)
        };
        self.header_mut().unwrap().manufacturer = U32::new(manufacturer.0);
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
    pub fn cmm(&self) -> Cmm {
        let header = self.header().unwrap();
        let tag = Tag(header.cmm.get());
        Cmm::new(tag) // this can not fail, as unknown CMMs are handled in the Cmm enum
    }

    /// Changes, or sets it, when creating a new profile, the Color Management Module (CMM) of the profile.
    pub fn with_cmm(mut self, cmm: Cmm) -> Result<Self, Error> {
        let tag = Tag::from(cmm);
        self.header_mut()?.cmm = U32::new(tag.0);
        Ok(self)
    }


}

#[cfg(test)]
mod test{ 
    use crate::{profile::Profile, tags::Tag};

    #[test]
    fn test_header() {
        let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
        let header = profile.header().unwrap();
        let mfg = header.manufacturer.get();
        let mfg_str = Tag(mfg).to_string();
        assert_eq!(mfg_str,"APPL");
    }

    #[test]
    fn test_set_manufacturer() {
        let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
        let signature: Tag = "TEST".parse().unwrap();
        let updated_profile = profile.with_manufacturer(Some(signature));
        let mfg_new = updated_profile.manufacturer().unwrap();
        assert_eq!(mfg_new.to_string(), "TEST");
    }

}