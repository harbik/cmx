// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;

use crate::{profile::Profile, tag::{bradford::Bradford, RenderingIntent}};

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
}

impl Default for DisplayProfile {
    fn default() -> Self {
        Self::new()
    }
}
