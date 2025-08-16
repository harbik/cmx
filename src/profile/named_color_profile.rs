// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::profile::Profile;

use super::RawProfile;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NamedColorProfile(pub(crate) RawProfile);

impl TryFrom<Profile> for NamedColorProfile {
    type Error = crate::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Profile::NamedColor(named_color_profile) = profile {
            Ok(named_color_profile)
        } else {
            Err(Self::Error::IsNotA("NamedColorProfile"))
        }
    }
}

impl NamedColorProfile {
    /// Creates a new, empty, `InputProfile` with
    ///
    /// - the default `RawProfile` with
    ///   - the ICC profile signature
    ///   - version set to 4.3
    ///   - the current date
    /// - `DeviceClass` set to `NamedColor`
    pub fn new() -> Self {
        Self(
            Self(RawProfile::default())
                .0
                .with_device_class(crate::signatures::DeviceClass::NamedColor),
        )
    }
}

impl Default for NamedColorProfile {
    fn default() -> Self {
        Self::new()
    }
}
