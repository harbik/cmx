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
                        pub fn manufacturer(&self) -> crate::signatures::Signature;
                        pub fn model(&self) -> crate::signatures::Signature;
                        pub fn pcs(&self) -> Option<crate::signatures::Pcs>;
                        pub fn pcs_illuminant(&self) -> [f64; 3];
                        pub fn primary_platform(&self) -> Option<crate::signatures::Platform>;
                        pub fn profile_size(&self) -> usize;
                        pub fn version(&self) -> Result<(u8, u8), crate::Error>;
                        pub fn to_file<P: AsRef<Path>>(self, path: P) -> Result<(), Box<dyn std::error::Error>>;
                    }
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
                    date: Option<chrono::DateTime<chrono::Utc>>,
                ) -> Self {
                    Self(self.0.with_creation_date(date))
                }

                pub fn with_tag<S: Into<crate::tag::TagSignature> + Copy>(
                    self,
                    signature: S,
                ) -> super::TagSetter<Self, S> {
                    super::TagSetter::new(self, signature)
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
