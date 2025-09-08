// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::{
    profile::Profile,
    tag::{bradford::Bradford, tags::*},
};

use super::RawProfile;

/// Represents an ICC input profile, which is a specific type of ICC profile used for input devices such as scanners and cameras.
/// This struct wraps a `RawProfile` and provides methods to convert from a generic `Profile`.
///
/// # Required Tags for all Input Profiles
///
/// - `desc`: Profile Description tag
/// - `cprt`: Copyright tag
/// - `dmwp`: Device Media white point tag
/// - `chad`: Media chromatic adaptation tag, if the white point of the data is different than that of the PCS
///  
/// # N-component LUT-based Input Profiles
/// For input profiles that use LUTs (Look-Up Tables) for color transformation, the following tags are required:
///
/// - `AtoB0`: A to B LUT tag for the first color component
///
/// Opinionally, the following tags can be present:
///
/// - `AtoB1`: A to B LUT tag for the second color component
/// - `AtoB2`: A to B LUT tag for the third color component
/// - `BtoA0`: B to A LUT tag for the first color component
/// - `BtoA1`: B to A LUT tag for the second color component
/// - `BtoA2`: B to A LUT tag for the third color component
/// - `DToB0`: Device to B LUT tag for the first color component
/// - `DToB1`: Device to B LUT tag for the second color component
/// - `DToB2`: Device to B LUT tag for the third color component
/// - `DToB3`: Device to B LUT tag for the fourth color component
/// - `BToD0`: B to Device LUT tag for the first color component
/// - `BToD1`: B to Device LUT tag for the second color component
/// - `BToD2`: B to Device LUT tag for the third color component
/// - `BToD3`: B to Device LUT tag for the fourth color component
///
/// # Three-component matrix-based Input Profiles
/// Only the XYZ PCS (Profile Connection Space) is supported three-component matrix-based input profiles.
/// For input profiles that use matrices for color transformation, the following tags are required:
///
/// - `rXYZ`: Red to XYZ matrix tag
/// - `gXYZ`: Green to XYZ matrix tag
/// - `bXYZ`: Blue to XYZ matrix tag
/// - `rTRC`: Red transfer function tag
/// - `gTRC`: Green transfer function tag
/// - `bTRC`: Blue transfer function tag
///
/// Optionally, the following tags can be present:
///
/// - `gamt`: Gamma tag
///
/// # Monochrome Input Profiles
///
/// For monochrome input profiles, the following tags are required:
///
/// - `kTRC`: Monochrome transfer function tag
///
/// Optionally, the following tags can be present:
///
/// - `AtoB0`: A to B LUT tag for the first color component
/// - `AtoB1`: A to B LUT tag for the second color component
/// - `AtoB2`: A to B LUT tag for the third color component
/// - `BtoA0`: B to A LUT tag for the first color component
/// - `BtoA1`: B to A LUT tag for the second color component
/// - `BtoA2`: B to A LUT tag for the third color component
///
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct InputProfile(pub(crate) RawProfile);

impl Default for InputProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl InputProfile {
    /// Creates a new, empty, `InputProfile` with
    ///
    /// - the default `RawProfile` with
    ///   - the ICC profile signature
    ///   - version set to 4.3
    ///   - the current date
    /// - `DeviceClass` set to `Input`
    pub fn new() -> Self {
        Self(
            Self(RawProfile::default())
                .0
                .with_device_class(crate::signatures::DeviceClass::Input),
        )
    }

    pub fn new_nlut() -> Self {
        todo!()
    }

    pub fn new_monochrome() -> Self {
        todo!()
    }
}

impl TryFrom<Profile> for InputProfile {
    type Error = crate::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        if let Profile::Input(input_profile) = profile {
            Ok(input_profile)
        } else {
            Err(Self::Error::IsNotA("Input Profile"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag::{
        tagdata::parametric_curve::ParametricCurveType, tags::RedTRCTag, TagSignature,
    };

    #[test]
    fn test_input_profile_from_display_p3() {
        let input_profile = InputProfile::from_rgb_space(colorimetry::rgb::RgbSpace::DisplayP3);
        let bytes = input_profile.to_bytes().unwrap();
        let input_profile_2 = InputProfile::try_from(Profile::from_bytes(&bytes).unwrap()).unwrap();
        let ts: TagSignature = RedTRCTag.into();
        let t = input_profile_2.0.tags.get(&ts).unwrap();
        let parametric_curve_data = t.tag.data().as_parametric_curve().unwrap();
        let parametric_curve_values: ParametricCurveType = parametric_curve_data.into();
        assert_eq!(
            parametric_curve_values.values().as_slice(),
            [2.39999, 0.94786, 0.05214, 0.07739, 0.04045, 0.0, 0.0].as_slice()
        );

        println!("{input_profile_2}");
    }

    #[test]
    fn test_input_profile_from_srgb() {
        let input_profile = InputProfile::from_rgb_space(colorimetry::rgb::RgbSpace::Adobe);
        let bytes = input_profile.to_bytes().unwrap();
        let input_profile_2 = InputProfile::try_from(Profile::from_bytes(&bytes).unwrap()).unwrap();
        let ts: TagSignature = RedTRCTag.into();
        let t = input_profile_2.0.tags.get(&ts).unwrap();
        let parametric_curve_data = t.tag.data().as_parametric_curve().unwrap();
        let parametric_curve_values: ParametricCurveType = parametric_curve_data.into();
        assert_eq!(
            parametric_curve_values.values().as_slice(),
            [2.19922, 0., 0., 0., 0., 0., 0.].as_slice()
        );

        println!("{input_profile_2}");
    }
}
