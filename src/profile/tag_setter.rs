// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC
#![allow(unused)]

use crate::{
    profile::RawProfile,
    tag::{
        tagdata::{CurveData, ParametricCurveData, TagData},
        ProfileTagRecord, Tag, TagSignature,
    },
};

// Provide a way to access the inner RawProfile from wrappers and Profile enum.
pub trait HasRawProfile {
    fn raw(&self) -> &RawProfile;
    fn raw_mut(&mut self) -> &mut RawProfile;
}

// Implement for RawProfile itself.
impl HasRawProfile for RawProfile {
    fn raw(&self) -> &RawProfile {
        self
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        self
    }
}

// Implement for all wrapper profiles.
impl HasRawProfile for super::InputProfile {
    fn raw(&self) -> &RawProfile {
        &self.0
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        &mut self.0
    }
}
impl HasRawProfile for super::DisplayProfile {
    fn raw(&self) -> &RawProfile {
        &self.0
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        &mut self.0
    }
}
impl HasRawProfile for super::OutputProfile {
    fn raw(&self) -> &RawProfile {
        &self.0
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        &mut self.0
    }
}
impl HasRawProfile for super::DeviceLinkProfile {
    fn raw(&self) -> &RawProfile {
        &self.0
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        &mut self.0
    }
}
impl HasRawProfile for super::AbstractProfile {
    fn raw(&self) -> &RawProfile {
        &self.0
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        &mut self.0
    }
}
impl HasRawProfile for super::ColorSpaceProfile {
    fn raw(&self) -> &RawProfile {
        &self.0
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        &mut self.0
    }
}
impl HasRawProfile for super::NamedColorProfile {
    fn raw(&self) -> &RawProfile {
        &self.0
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        &mut self.0
    }
}
impl HasRawProfile for super::SpectralProfile {
    fn raw(&self) -> &RawProfile {
        &self.0
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        &mut self.0
    }
}

// Implement for the enum Profile.
impl HasRawProfile for super::Profile {
    fn raw(&self) -> &RawProfile {
        self.as_raw_profile()
    }
    fn raw_mut(&mut self) -> &mut RawProfile {
        super::Profile::as_raw_profile_mut(self)
    }
}

pub trait IsMultiLocalizedUnicodeTag {}

pub trait IsCurveTag {}
impl IsCurveTag for crate::tag::tags::BlueTRCTag {}
impl IsCurveTag for crate::tag::tags::GreenTRCTag {}
impl IsCurveTag for crate::tag::tags::RedTRCTag {}
impl IsCurveTag for crate::tag::tags::GrayTRCTag {}

pub trait IsParametricCurveTag {}
impl IsParametricCurveTag for crate::tag::tags::BlueTRCTag {}
impl IsParametricCurveTag for crate::tag::tags::GreenTRCTag {}
impl IsParametricCurveTag for crate::tag::tags::RedTRCTag {}
impl IsParametricCurveTag for crate::tag::tags::GrayTRCTag {}

pub trait IsSignatureTag {}
impl IsSignatureTag for crate::tag::tags::ColorimetricIntentImageStateTag {}
impl IsSignatureTag for crate::tag::tags::TechnologyTag {}
#[cfg(feature = "v5")]
impl IsSignatureTag for crate::tag::tags::SaturationRenderingIntentGamutTag {}
impl IsSignatureTag for crate::tag::tags::PerceptualRenderingIntentGamutTag {}

pub trait IsXYZArrayTag {}
impl IsXYZArrayTag for crate::tag::tags::RedMatrixColumnTag {}
impl IsXYZArrayTag for crate::tag::tags::GreenMatrixColumnTag {}
impl IsXYZArrayTag for crate::tag::tags::BlueMatrixColumnTag {}
impl IsXYZArrayTag for crate::tag::tags::LuminanceTag {}
impl IsXYZArrayTag for crate::tag::tags::MediaWhitePointTag {}
impl IsXYZArrayTag for crate::tag::tags::MediaBlackPointTag {}

pub trait IsTextDescriptionTag {}
impl IsTextDescriptionTag for crate::tag::tags::ProfileDescriptionTag {}
impl IsTextDescriptionTag for crate::tag::tags::CopyrightTag {}
impl IsTextDescriptionTag for crate::tag::tags::DeviceMfgDescTag {}
impl IsTextDescriptionTag for crate::tag::tags::DeviceModelDescTag {}
#[cfg(feature = "v5")]
impl IsTextDescriptionTag for crate::tag::tags::ScreeningDescTag {}
impl IsTextDescriptionTag for crate::tag::tags::ViewingCondDescTag {}

pub trait IsTextTag {}
impl IsTextTag for crate::tag::tags::CopyrightTag {}
impl IsTextTag for crate::tag::tags::CharTargetTag {}

pub trait IsLut8DataTag {}
pub trait IsLut16DataTag {}
pub trait IsLutAtoBDataTag {}
pub trait IsLutBtoADataTag {}

/// A helper for safely setting the data for a specific tag signature.
/// It is generic over the signature type `S` to enable compile-time checks,
/// and generic over the profile type `P` to return the correct P for chaining.
pub struct TagSetter<P: HasRawProfile, S> {
    profile: P,
    tag: S,
}

impl<P, S> TagSetter<P, S>
where
    P: HasRawProfile,
    S: Into<TagSignature> + Copy,
{
    pub fn new(profile: P, tag: S) -> Self {
        Self { profile, tag }
    }

    /// Sets the tag's data as a `CurveData`.
    /// This method is only available if the signature implements `IsCurveTag`.
    pub fn as_curve<F>(mut self, configure: F) -> P
    where
        S: IsCurveTag,
        F: FnOnce(&mut CurveData),
    {
        let curve = self.profile.raw_mut().ensure_curve_mut(self.tag.into());
        configure(curve);
        self.profile
    }

    pub fn as_parametric_curve<F>(mut self, configure: F) -> P
    where
        S: IsParametricCurveTag,
        F: FnOnce(&mut ParametricCurveData),
    {
        let para_curve = self
            .profile
            .raw_mut()
            .ensure_parametric_curve_mut(self.tag.into());
        configure(para_curve);
        self.profile
    }

    pub fn as_signature<F>(mut self, configure: F) -> P
    where
        S: IsSignatureTag,
        F: FnOnce(&mut crate::tag::tagdata::SignatureData),
    {
        let signature = self.profile.raw_mut().ensure_signature_mut(self.tag.into());
        configure(signature);
        self.profile
    }

    pub fn as_xyz_array<F>(mut self, configure: F) -> P
    where
        S: IsXYZArrayTag,
        F: FnOnce(&mut crate::tag::tagdata::XYZArrayData),
    {
        let xyz = self.profile.raw_mut().ensure_xyz_array_mut(self.tag.into());
        configure(xyz);
        self.profile
    }

    pub fn as_text_description<F>(mut self, configure: F) -> P
    where
        S: IsTextDescriptionTag,
        F: FnOnce(&mut crate::tag::tagdata::TextDescriptionData),
    {
        let text_description = self
            .profile
            .raw_mut()
            .ensure_text_description_mut(self.tag.into());
        configure(text_description);
        self.profile
    }

    pub fn as_text<F>(mut self, configure: F) -> P
    where
        S: IsTextTag,
        F: FnOnce(&mut crate::tag::tagdata::TextData),
    {
        let text = self
            .profile
            .raw_mut()
            .ensure_text_mut(self.tag.into());
        configure(text);
        self.profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::InputProfile;
    use crate::tag::tags::*;
    #[test]
    fn test_tag_setter() -> Result<(), Box<dyn std::error::Error>> {
        let mut profile = InputProfile::new();

        profile
            .with_tag(RedTRCTag)
            .as_curve(|curve| {
                curve.set_gamma(2.2);
            })
            .with_tag(GreenTRCTag)
            .as_curve(|curve| {
                curve.set_gamma(2.2);
            })
            .with_tag(BlueTRCTag)
            .as_curve(|curve| {
                curve.set_gamma(2.2);
            })
            .with_tag(GrayTRCTag)
            .as_parametric_curve(|para_curve| para_curve.set_parameters([0.5]))
            .with_tag(TechnologyTag)
            .as_signature(|signature| {
                signature.set_signature("fscn");
            });

        Ok(())
    }
}
