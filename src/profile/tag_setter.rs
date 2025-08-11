use crate::{
    profile::RawProfile,
    tag::{
        tag_value::CurveType, 
        tag_value::MultiLocalizedUnicodeType, tag_value::TagValue, TagTable,
        TagSignature,
    },
};

// Marker traits (families of allowed inner types)
pub trait UnambiguousTag {}
pub trait IsTextDescriptionTag {}
pub trait IsMultiLocalizedUnicodeTag {}
pub trait IsCurveTag {}
pub trait IsParametricCurveTag {}
pub trait IsLut8TypeTag {}
pub trait IsLut16TypeTag {}
pub trait IsLutAtoBTypeTag {}
pub trait IsLutBtoATypeTag {}


/*
Allows for a ergonomic way to set tag data in a `RawProfile`.
Example usage:
let mut profile = RawProfile::new()
    // set profile header data
    .with_profile_version(4, 4)
    .with_creation_date(None)
    .add_tag(ChromaticityTag)
        .with_data(|data| { // all umbiguous tags can use this method
            // use data to set the ChromaticityType
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

    /// Configures the tag's data directly using a closure.
    ///
    /// This ergonomic method is only available for tag signatures that have a single,
    /// unambiguous data type. The closure receives a mutable reference to the
    /// correct data type automatically.
    pub fn with_data<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: UnambiguousTag, // This method is only available for unambiguous tags!
        F: FnOnce(&mut S::TagType),
    {
        let mut data = S::TagType::default();
        configure(&mut data);
        let tag = S::new_tag(data);
        // as this is a new tag, it did not get assigned an offset and length yet.
        self.profile
            .tags
            .insert(self.signature.into(), TagTable::new(0, 0, tag));
        self.profile
    }

    /// Sets the tag's data as a `curveType`.
    /// This method is only available if the signature implements `IsCurveTag`.
    pub fn as_curve<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: IsCurveTag, // The compile-time safety check!
        F: FnOnce(&mut CurveType),
    {
        let mut curve = CurveType::default();
        configure(&mut curve);
        let curve_tag = TagValue::Curve(curve);
        self.profile
            .tags
            .insert(self.signature.into(), TagTable::new(0, 0, curve_tag));
        self.profile
    }

    pub fn as_multi_localized_unicode<F>(self, configure: F) -> &'a mut RawProfile
    where
        S: IsMultiLocalizedUnicodeTag,
        F: FnOnce(&mut MultiLocalizedUnicodeType),
    {
        let mut mlu = MultiLocalizedUnicodeType::default();
        configure(&mut mlu);
        let mlu_tag = TagValue::MultiLocalizedUnicode(mlu);
        self.profile
            .tags
            .insert(self.signature.into(), TagTable::new(0, 0, mlu_tag));
        self.profile // Return a mutable reference to the profile itself
    }
}
