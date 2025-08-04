use serde::{Serialize};

use crate::profile::Profile;

use super::RawProfile;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DisplayProfile(pub(crate) RawProfile);

impl TryFrom<Profile> for DisplayProfile {
    type Error = crate::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Profile::Display(display_profile) = profile {
            Ok(display_profile)
        } else {
            Err(Self::Error::IsNotA("Display Profile"))
        }
    }
}