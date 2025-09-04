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

    /// Creates an input profile from a Colorimetry::RgbSpace.
    ///
    /// This type of profile is typically used to embed in image files,
    /// such as a PNG.
    ///
    /// # Example
    /// See `colorimetry-plot::examples::display_p3_gamut.rs` how this is used to show a full
    /// DisplayP3 gamut in a CIE 1931 chromaticity diagram, if viewed on a high gamut display, using
    /// a modern Web Browser with support of managed color workflows.
    pub fn from_rgb_space(rgb_space: colorimetry::rgb::RgbSpace) -> InputProfile {
        let input_profile = Self::new();
        let pcs_illuminant = input_profile.pcs_illuminant(); // always D50
        let obs = colorimetry::observer::Observer::Cie1931;
        let pcs_illuminant_xyz = colorimetry::xyz::XYZ::new(pcs_illuminant, obs);
        let media_white_xyz = rgb_space.white_point(obs).set_illuminance(1.0).values();
        let m_rgb = obs.calc_rgb2xyz_matrix_with_alt_white(rgb_space, Some(pcs_illuminant_xyz));
        let r_xyz = m_rgb.column(0);
        let g_xyz = m_rgb.column(1);
        let b_xyz = m_rgb.column(2);
        let bradford = Bradford::new(media_white_xyz, pcs_illuminant).as_matrix();
        let gamma_values = rgb_space.gamma().values();

        input_profile
            .with_rendering_intent(crate::tag::RenderingIntent::RelativeColorimetric)
            .with_tag(ProfileDescriptionTag)
            .as_text_description(|text| {
                text.set_ascii("CMX_P3");
            })
            .with_tag(CopyrightTag)
            .as_text(|text| {
                text.set_text("CC0");
            })
            .with_tag(MediaWhitePointTag)
            .as_xyz_array(|xyz| {
                xyz.set(media_white_xyz);
            })
            .with_tag(RedMatrixColumnTag)
            .as_xyz_array(|xyz| {
                xyz.set(r_xyz.as_slice().try_into().unwrap());
            })
            .with_tag(GreenMatrixColumnTag)
            .as_xyz_array(|xyz| {
                xyz.set(g_xyz.as_slice().try_into().unwrap());
            })
            .with_tag(BlueMatrixColumnTag)
            .as_xyz_array(|xyz| {
                xyz.set(b_xyz.as_slice().try_into().unwrap());
            })
            .with_tag(ChromaticAdaptationTag)
            .as_sf15_fixed_16_array(|array| {
                let bradford_array: [f64; 9] = bradford.as_slice().try_into().unwrap();
                array.set(bradford_array);
            })
            .with_tag(RedTRCTag)
            .as_parametric_curve(|para| {
                para.set_parameters_slice(gamma_values);
            })
            .with_tag(BlueTRCTag)
            .as_parametric_curve(|para| {
                para.set_parameters_slice(gamma_values);
            })
            .with_tag(GreenTRCTag)
            .as_parametric_curve(|para| {
                para.set_parameters_slice(gamma_values);
            })
            .with_profile_id()
    }

    #[allow(unused)]
    /// Creates a new Three-component matrix-based ("RGB") InputProfile
    /// using one of RGB color spaces as defined in the Rust `Colorimetry` library.
    pub fn new_with_rgbspace(rgb_space: colorimetry::rgb::RgbSpace) -> Self {
        todo!()
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
