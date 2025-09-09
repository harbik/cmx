// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::{
    profile::Profile,
    tag::{bradford::Bradford, RenderingIntent},
};

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

impl DisplayProfile {
    /// Creates a new, empty, `InputProfile` with
    ///
    /// - the default `RawProfile` with
    ///   - the ICC profile signature
    ///   - version set to 4.3
    ///   - the current date
    /// - `DeviceClass` set to `Display`
    pub fn new() -> Self {
        Self(
            Self(RawProfile::default())
                .0
                .with_device_class(crate::signatures::DeviceClass::Display)
                .with_data_color_space(crate::signatures::ColorSpace::RGB),
        )
    }

    /// Creates an output profile from a Colorimetry::RgbSpace.
    ///
    /// This type of profile is typically used to embed in image files,
    /// such as a PNG.
    ///
    /// # Example
    /// See `colorimetry-plot::examples::display_p3_gamut.rs` how this is used to show a full
    /// DisplayP3 gamut in a CIE 1931 chromaticity diagram, if viewed on a high gamut display, using
    /// a modern Web Browser with support of managed color workflows.
    #[rustfmt::skip]
    pub fn from_rgb_space(rgb_space: colorimetry::rgb::RgbSpace, rendering_intent: RenderingIntent) -> Self {
        let display_profile = Self::new();
        let pcs_illuminant = display_profile.pcs_illuminant(); // always D50
        let obs = colorimetry::observer::Observer::Cie1931;
        let pcs_illuminant_xyz = colorimetry::xyz::XYZ::new(pcs_illuminant, obs);
        let media_white_xyz = rgb_space.white_point(obs).set_illuminance(1.0).values();
        let m_rgb = obs.calc_rgb2xyz_matrix_with_alt_white(rgb_space, Some(pcs_illuminant_xyz));
        let r_xyz = m_rgb.column(0);
        let g_xyz = m_rgb.column(1);
        let b_xyz = m_rgb.column(2);
        let bradford = Bradford::new(media_white_xyz, pcs_illuminant).as_matrix();
        let gamma_values = rgb_space.gamma().values();

        use crate::tag::tags::*;

        display_profile
            .with_rendering_intent(rendering_intent)
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

    pub fn cmx_p3(rendering_intent: RenderingIntent) -> Self {
        use crate::tag::tags::*;
        DisplayProfile::new()
            .with_rendering_intent(rendering_intent)
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
                    xyz.set([0.950455, 1.00000, 1.08905]);
                })
            .with_tag(RedMatrixColumnTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.515121, 0.241196, -0.001053]);
                })
            .with_tag(GreenMatrixColumnTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.291977, 0.692245, 0.041885]);
                })
            .with_tag(BlueMatrixColumnTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.157104, 0.066574, 0.784073]);
                })
            .with_tag(RedTRCTag)
                .as_parametric_curve(|para| {
                    para.set_parameters([2.39999, 0.94786, 0.05214, 0.07739, 0.04045]);
                })
            .with_tag(BlueTRCTag)
                .as_parametric_curve(|para| {
                    para.set_parameters([2.39999, 0.94786, 0.05214, 0.07739, 0.04045]);
                })
            .with_tag(GreenTRCTag)
                .as_parametric_curve(|para| {
                    para.set_parameters([2.39999, 0.94786, 0.05214, 0.07739, 0.04045]);
                })
            .with_tag(ChromaticAdaptationTag)
                .as_sf15_fixed_16_array(|array| {
                    array.set([
                         1.047882, 0.022919, -0.050201,
                         0.029587, 0.990479, -0.017059,
                        -0.009232, 0.015076,  0.751678
                    ]);
                })
    }

}

impl Default for DisplayProfile {
    fn default() -> Self {
        Self::new()
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
        let input_profile = DisplayProfile::from_rgb_space(
            colorimetry::rgb::RgbSpace::DisplayP3,
            RenderingIntent::RelativeColorimetric,
        );
        let bytes = input_profile.to_bytes().unwrap();
        let display_profile_2 =
            DisplayProfile::try_from(Profile::from_bytes(&bytes).unwrap()).unwrap();
        let ts: TagSignature = RedTRCTag.into();
        let t = display_profile_2.0.tags.get(&ts).unwrap();
        let parametric_curve_data = t.tag.data().as_parametric_curve().unwrap();
        let parametric_curve_values: ParametricCurveType = parametric_curve_data.into();
        assert_eq!(
            parametric_curve_values.values().as_slice(),
            [2.39999, 0.94786, 0.05214, 0.07739, 0.04045, 0.0, 0.0].as_slice()
        );

        println!("{display_profile_2}");
    }

    #[test]
    fn test_display_profile_from_srgb() {
        let display_profile = DisplayProfile::from_rgb_space(
            colorimetry::rgb::RgbSpace::Adobe,
            RenderingIntent::RelativeColorimetric,
        );
        let bytes = display_profile.to_bytes().unwrap();
        let display_profile_2 =
            DisplayProfile::try_from(Profile::from_bytes(&bytes).unwrap()).unwrap();
        let ts: TagSignature = RedTRCTag.into();
        let t = display_profile_2.0.tags.get(&ts).unwrap();
        let parametric_curve_data = t.tag.data().as_parametric_curve().unwrap();
        let parametric_curve_values: ParametricCurveType = parametric_curve_data.into();
        assert_eq!(
            parametric_curve_values.values().as_slice(),
            [2.19922, 0., 0., 0., 0., 0., 0.].as_slice()
        );

        println!("{display_profile_2}");
    }
}
