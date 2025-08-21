// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! This module provides a delegation pattern for RawProfile methods to all the Device Profile structs.

use delegate::delegate;

macro_rules! delegate_raw_profile_methods {
    ($($profile:ident),*) => {
        $(
            impl super::$profile {
                delegate! {
                    to self.0 {
                        pub fn version(&self) -> Result<(u8, u8), crate::Error>;
                        pub fn profile_size(&self) -> usize;
                        pub fn flags(&self) -> (bool, bool);
                        pub fn data_color_space(&self) -> crate::signatures::ColorSpace;
                        pub fn primary_platform(&self) -> crate::signatures::Platform;
                        pub fn manufacturer(&self) -> crate::signatures::Signature;
                        pub fn model(&self) -> crate::signatures::Signature;
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

                pub fn with_tag<'a, S: Into<crate::tag::TagSignature> + Copy>(
                    &'a mut self,
                    signature: S,
                ) -> super::TagSetter<'a, Self, S> {
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
