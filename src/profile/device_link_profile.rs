// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::profile::Profile;

use super::RawProfile;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DeviceLinkProfile(pub(crate) RawProfile);

impl TryFrom<Profile> for DeviceLinkProfile {
    type Error = crate::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Profile::DeviceLink(device_link_profile) = profile {
            Ok(device_link_profile)
        } else {
            Err(Self::Error::IsNotA("DeviceLinkProfile"))
        }
    }
}

impl DeviceLinkProfile {
    /// Creates a new, empty, `InputProfile` with
    ///
    /// - the default `RawProfile` with
    ///   - the ICC profile signature
    ///   - version set to 4.3
    ///   - the current date
    /// - `DeviceClass` set to `DeviceLink`
    pub fn new() -> Self {
        Self(
            Self(RawProfile::default())
                .0
                .with_device_class(crate::signatures::DeviceClass::DeviceLink),
        )
    }
}

impl Default for DeviceLinkProfile {
    fn default() -> Self {
        Self::new()
    }
}
