use serde::{Serialize};

use crate::profile::Profile;

use super::RawProfile;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OutputProfile(pub(crate) RawProfile);

impl TryFrom<Profile> for OutputProfile {
    type Error = crate::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Profile::Output(output_profile) = profile {
            Ok(output_profile)
        } else {
            Err(Self::Error::IsNotA("Output Profile"))
        }
    }
}