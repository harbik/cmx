// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC
#![allow(unused)]

use crate::{
    profile::{ProfileTagRecord, RawProfile},
    tag::{
        self,
        tagdata::{CurveData, ParametricCurveData, TagData},
        Tag, TagSignature,
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
impl IsMultiLocalizedUnicodeTag for crate::tag::tags::CopyrightTag {}
impl IsMultiLocalizedUnicodeTag for crate::tag::tags::ProfileDescriptionTag {}
impl IsMultiLocalizedUnicodeTag for crate::tag::tags::DeviceMfgDescTag {}
impl IsMultiLocalizedUnicodeTag for crate::tag::tags::DeviceModelDescTag {}
impl IsMultiLocalizedUnicodeTag for crate::tag::tags::ViewingCondDescTag {}
#[cfg(feature = "v5")]
impl IsMultiLocalizedUnicodeTag for crate::tag::tags::ScreeningDescTag {}

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

pub trait IsS15Fixed16ArrayTag {}
impl IsS15Fixed16ArrayTag for crate::tag::tags::ChromaticAdaptationTag {}

pub trait IsLut8DataTag {}
pub trait IsLut16DataTag {}
pub trait IsLutAtoBDataTag {}
pub trait IsLutBtoADataTag {}

pub trait IsRawTag {}

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
        let text = self.profile.raw_mut().ensure_text_mut(self.tag.into());
        configure(text);
        self.profile
    }

    pub fn as_sf15_fixed_16_array<F>(mut self, configure: F) -> P
    where
        S: IsS15Fixed16ArrayTag,
        F: FnOnce(&mut crate::tag::tagdata::S15Fixed16ArrayData),
    {
        let array = self
            .profile
            .raw_mut()
            .ensure_s15_fixed_16_array_mut(self.tag.into());
        configure(array);
        self.profile
    }

    /// Sets the tag's data as raw bytes.
    /// This is used for non-ICC or manufacturer private tags, with unknown data formats, It is the
    /// caller's responsibility to ensure the data is valid for the intended use.
    /// It can also be used to set the raw data for known tags, but this is not recommended,
    /// as it bypasses the type safety provided by the other methods.
    pub fn as_raw<F>(mut self, configure: F) -> P
    where
        F: FnOnce(&mut crate::tag::tagdata::RawData),
    {
        let sig: TagSignature = self.tag.into();
        let raw = self.profile.raw_mut().ensure_raw_mut(self.tag.into());

        // If the tag has no data yet (is a new tag), initialize it with the tag signature and
        // reserved bytes.  This method is intended for use with unknown tags.  If a new tag is
        // being created using this method, it will use the tag signature also as the type
        // signature.  If the tag already exists, it may already have data, in which case we do not
        // overwrite it.
        if raw.0.is_empty() {
            // Initialize with the tag signature and reserved bytes.
            let mut initial_data = Vec::with_capacity(8);
            initial_data.extend_from_slice(&sig.to_u32().to_be_bytes());
            initial_data.extend_from_slice(&[0u8; 4]); // Reserved bytes
            raw.0 = initial_data;
        }
        configure(raw);
        self.profile
    }

    pub fn as_multi_localized_unicode<F>(mut self, configure: F) -> P
    where
        S: IsMultiLocalizedUnicodeTag,
        F: FnOnce(&mut crate::tag::tagdata::MultiLocalizedUnicodeData),
    {
        let mlu = self
            .profile
            .raw_mut()
            .ensure_multi_localized_unicode_mut(self.tag.into());
        configure(mlu);
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
