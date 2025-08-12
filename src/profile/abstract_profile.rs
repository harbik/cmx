// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::profile::Profile;

use super::RawProfile;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AbstractProfile(pub(crate) RawProfile);

impl TryFrom<Profile> for AbstractProfile {
    type Error = crate::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Profile::Abstract(abstract_profile) = profile {
            Ok(abstract_profile)
        } else {
            Err(Self::Error::IsNotA("AbstractProfile"))
        }
    }
}
