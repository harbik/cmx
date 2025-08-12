// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::profile::Profile;

use super::RawProfile;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ColorSpaceProfile(pub(crate) RawProfile);

impl TryFrom<Profile> for ColorSpaceProfile {
    type Error = crate::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Profile::ColorSpace(abstract_profile) = profile {
            Ok(abstract_profile)
        } else {
            Err(Self::Error::IsNotA("ColorSpace Profile"))
        }
    }
}
