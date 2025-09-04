// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! This module provides a delegation pattern for RawProfile methods to all the Device Profile structs.

use delegate::delegate;
use std::path::Path;

macro_rules! delegate_raw_profile_methods {
    ($($profile:ident),*) => {
        $(
            impl super::$profile {
                delegate! {
                    to self.0 {
                        pub fn apple_flags(&self) -> (crate::tag::Quality, crate::tag::Interpolate, crate::tag::GamutCheck);
                        pub fn cmm(&self) -> Option<crate::signatures::Cmm>;
                        pub fn creation_date(&self) -> chrono::DateTime<chrono::Utc>;
                        pub fn data_color_space(&self) -> Option<crate::signatures::ColorSpace>;
                        pub fn flags(&self) -> (bool, bool);
                        pub fn manufacturer(&self) -> Option<crate::signatures::Signature>;
                        pub fn model(&self) -> crate::signatures::Signature;
                        pub fn pcs(&self) -> Option<crate::signatures::Pcs>;
                        pub fn pcs_illuminant(&self) -> [f64; 3];
                        pub fn primary_platform(&self) -> Option<crate::signatures::Platform>;
                        pub fn profile_size(&self) -> usize;
                        pub fn profile_id(&self) -> [u8; 16];
                        pub fn version(&self) -> Result<(u8, u8), crate::Error>;
                        pub fn write<P: AsRef<Path>>(self, path: P) -> Result<(), Box<dyn std::error::Error>>;
                    }
                }

                pub fn from_bytes(
                    bytes: &[u8],
                ) -> Result<Self, Box<dyn std::error::Error>> {
                    Ok(Self(crate::profile::RawProfile::from_bytes(bytes)?))
                }

                pub fn to_bytes(
                    self,
                ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
                    self.0.into_bytes()
                }

                pub fn with_version(
                    self,
                    major: u8,
                    minor: u8,
                ) -> Result<Self, crate::Error> {
                    Ok(Self(self.0.with_version(major, minor)?))
                }

                pub fn with_creation_date(
                    self,
                    date: impl Into<chrono::DateTime<chrono::Utc>>,
                ) -> Self {
                    Self(self.0.with_creation_date(date))
                }

                pub fn with_now_as_creation_date(
                    self,
                ) -> Self {
                    Self(self.0.with_now_as_creation_date())
                }

                pub fn with_tag<S: Into<crate::tag::TagSignature> + Copy>(
                    self,
                    tag: S,
                ) -> super::TagSetter<Self, S> {
                    super::TagSetter::new(self, tag)
                }

                pub fn with_primary_platform(
                    self,
                    platform: crate::signatures::Platform
                ) -> Self {
                    Self(self.0.with_primary_platform(platform))
                }

                pub fn with_cmm(
                    self,
                    cmm: crate::signatures::Cmm) -> Result<Self, crate::Error> {
                        Ok(Self(self.0.with_cmm(cmm)?))
                }

                pub fn with_manufacturer(mut self, manufacturer: &str) -> Self {
                    self.0 = self.0.with_manufacturer(manufacturer);
                    self
                }

                pub fn with_data_color_space(
                    self,
                    color_space: crate::signatures::ColorSpace
                ) -> Self {
                    Self(self.0.with_data_color_space(color_space))
                }

                pub fn with_rendering_intent(
                    self,
                    intent: crate::tag::RenderingIntent
                ) -> Self {
                    Self(self.0.with_rendering_intent(intent))
                }

                pub fn with_creator(
                    self,
                    creator:  &str
                ) -> Self {
                    Self(self.0.with_creator(creator))
                }

                pub fn without_profile_id(self) -> Self {
                    Self(self.0.without_profile_id())
                }

                pub fn with_profile_id(self) -> Self {
                    Self(self.0.with_profile_id())
                }
            }
        )*
    };
}

delegate_raw_profile_methods!(
    InputProfile,
    DisplayProfile,
    OutputProfile,
    DeviceLinkProfile,
    AbstractProfile,
    ColorSpaceProfile,
    NamedColorProfile,
    SpectralProfile
);
