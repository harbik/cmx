// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC
#! [allow(unused)]

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
impl IsCurveTag for crate::tag::tags::BlueTRC {}
impl IsCurveTag for crate::tag::tags::GreenTRC {}
impl IsCurveTag for crate::tag::tags::RedTRC {}
impl IsCurveTag for crate::tag::tags::GrayTRC {}

pub trait IsParametricCurveTag {}
impl IsParametricCurveTag for crate::tag::tags::BlueTRC {}
impl IsParametricCurveTag for crate::tag::tags::GreenTRC {}
impl IsParametricCurveTag for crate::tag::tags::RedTRC {}
impl IsParametricCurveTag for crate::tag::tags::GrayTRC {}

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
    signature: S,
}

impl<'a, S: Into<TagSignature> + Copy> TagSetter<'a, S> {
    pub fn new(profile: &'a mut RawProfile, signature: S) -> Self {
        Self { profile, signature }
    }

    /// Sets the tag's data as a `CurveData`.
    /// This method is only available if the signature implements `IsCurveTag`.
    pub fn as_curve<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: IsCurveTag,
        F: FnOnce(&mut CurveData),
    {
        let curve = self.profile.ensure_curve_mut(self.signature.into());
        configure(curve);
        self.profile
    }

    pub fn as_parametric_curve<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: IsParametricCurveTag,
        F: FnOnce(&mut ParametricCurveData),
    {
        let para_curve = self.profile.ensure_parametric_curve_mut(self.signature.into());
        configure(para_curve);
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
            .with_tag(RedTRC)
            .as_curve(|curve| {
                curve.set_gamma(2.2);
            })
            .with_tag(GreenTRC)
            .as_curve(|curve| {
                curve.set_gamma(2.2);
            })
            .with_tag(BlueTRC)
            .as_curve(|curve| {
                curve.set_gamma(2.2);
            })
            .with_tag(GrayTRC)
            .as_parametric_curve(|para_curve| {
                para_curve.set_parameters([0.5]);
            });

        Ok(())
        // Further assertions can be added to verify the profile state.
    }
}


