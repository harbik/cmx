use crate::{profile::RawProfile, signatures::TagSignature, tags::{CurveType, IsCurveTag, IsMultiLocalizedUnicodeTag, IsTextDescriptionTag, MultiLocalizedUnicodeType, Tag, TagEntry, UnambiguousTag}};


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
    pub fn with_data<F>(self, configure: F)
    where
        S: UnambiguousTag, // This method is only available for unambiguous tags!
        F: FnOnce(&mut S::DataType),
    {
        let mut data = S::DataType::default();
        configure(&mut data);
        let tag = S::new_tag(data);
        // as this is a new tag, it did not get assigned an offset and length yet.
        self.profile.tags.insert(self.signature.into(), TagEntry::new(0, 0, tag));
    }

    /// Sets the tag's data as a `curveType`.
    /// This method is only available if the signature implements `IsCurveTag`.
    pub fn as_curve<F>(self, configure: F)
    where
        S: IsCurveTag, // The compile-time safety check!
        F: FnOnce(&mut CurveType),
    {
        let mut curve = CurveType::default();
        configure(&mut curve);
        let curve_tag = Tag::Curve(curve);
        self.profile.tags.insert(self.signature.into(), TagEntry::new(0, 0, curve_tag));
    }

    /// Sets the tag's data as a `multiLocalizedUnicodeType`.
    pub fn as_multi_localized_unicode<F>(self, configure: F)
    where
        S: IsMultiLocalizedUnicodeTag, // The check!
        F: FnOnce(&mut MultiLocalizedUnicodeType),
    {
        let mut mlu = MultiLocalizedUnicodeType::default();
        configure(&mut mlu);
        let mlu_tag = Tag::MultiLocalizedUnicode(mlu);
        self.profile.tags.insert(self.signature.into(), TagEntry::new(0, 0, mlu_tag));
    }
}