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

pub trait IsTextDescriptionTag {}
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

pub trait IsLut8DataTag {}
pub trait IsLut16DataTag {}
pub trait IsLutAtoBDataTag {}
pub trait IsLutBtoADataTag {}

/*
Allows for a ergonomic way to set tag data in a `RawProfile`.
Example usage:
let mut profile = RawProfile::new()
    // set profile header data
    .with_profile_version(4, 4)
    .with_creation_date(None)
    .add_tag(ChromaticityTag)
        .with_data(|data| { // all umbiguous tags can use this method
            // use data to set the ChromaticityData
            data.set_standard(Primaries::ITU);
        }) // This returns &mut RawProfile, so we can chain...
    .add_tag(ProfileDescriptionTag)
        .as_multi_localized_unicode(|mlu| {
            // use mlu to set the data for the MultiLocalizedUnicode tag
        }) // This returns &mut RawProfile, so we can chain...
    /*
        alternatively
        .as_text_description(|text| {
            // use text to set the data for the TextDescription tag
        })
    */
    .add_tag(RedTRCTag)
        .as_curve(|curve| {
            // ...
        }); // ...and so on.
*/

/// A helper for safely setting the data for a specific tag signature.
/// It is generic over the signature type `S` to enable compile-time checks.
pub struct TagSetter<'a, S: 'a> {
    profile: &'a mut RawProfile,
    tag: S,
}

impl<'a, S: Into<TagSignature> + Copy> TagSetter<'a, S> {
    pub fn new(profile: &'a mut RawProfile, tag: S) -> Self {
        Self { profile, tag }
    }

    /// Sets the tag's data as a `CurveData`.
    /// This method is only available if the signature implements `IsCurveTag`.
    pub fn as_curve<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: IsCurveTag,
        F: FnOnce(&mut CurveData),
    {
        let curve = self.profile.ensure_curve_mut(self.tag.into());
        configure(curve);
        self.profile
    }

    pub fn as_parametric_curve<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: IsParametricCurveTag,
        F: FnOnce(&mut ParametricCurveData),
    {
        let para_curve = self.profile.ensure_parametric_curve_mut(self.tag.into());
        configure(para_curve);
        self.profile
    }

    pub fn as_signature<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: IsSignatureTag,
        F: FnOnce(&mut crate::tag::tagdata::SignatureData),
    {
        let signature = self.profile.ensure_signature_mut(self.tag.into());
        configure(signature);
        self.profile
    }
}
/*
    /// correct data type automatically.
    pub fn with_data<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: UnambiguousTag, // This method is only available for unambiguous tags!
        F: FnOnce(&mut <S as UnambiguousTagData>::TagData),
    {
        let mut data = <S as UnambiguousTagData>::TagData::default();
        configure(&mut data);
        let tag = S::new_tag(data);
        // as this is a new tag, it did not get assigned an offset and length yet.
        configure(&mut data);
        let tag = S::new_tag(data);
        // as this is a new tag, it did not get assigned an offset and length yet.
        self.profile
            .tags
            .insert(self.signature.into(), RawTag::new(0, 0, tag));
        self.profile
    }


    pub fn as_multi_localized_unicode<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: IsMultiLocalizedUnicodeTag,
        F: FnOnce(&mut MultiLocalizedUnicodeData),
    {
        let mut mlu = MultiLocalizedUnicodeData::default();
        configure(&mut mlu);
        let mlu_tag = TagData::MultiLocalizedUnicode(mlu);
        self.profile
            .tags
            .insert(self.signature.into(), RawTag::new(0, 0, mlu_tag));
        self.profile // Return a mutable reference to the profile itself
    }
}

 */

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
        // Further assertions can be added to verify the profile state.
    }
}
