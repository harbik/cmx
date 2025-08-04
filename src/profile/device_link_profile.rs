use serde::{Serialize};

use crate::profile::Profile;

use super::RawProfile;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DeviceLinkProfile(pub(crate) RawProfile);

impl TryFrom<Profile> for DeviceLinkProfile{
    type Error = crate::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Profile::DeviceLink(device_link_profile) = profile {
            Ok(device_link_profile)
        } else {
            Err(Self::Error::IsNotA("DeviceLinkProfile"))
        }
    }
}