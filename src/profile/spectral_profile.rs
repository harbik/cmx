// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::profile::Profile;

use super::RawProfile;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SpectralProfile(pub(crate) RawProfile);

impl TryFrom<Profile> for SpectralProfile {
    type Error = crate::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Profile::Spectral(spectral_profile) = profile {
            Ok(spectral_profile)
        } else {
            Err(Self::Error::IsNotA("Spectral Profile"))
        }
    }
}
