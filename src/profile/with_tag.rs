// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! This module provides a type-safe builder API for constructing ICC profiles.
//!
//! The core of the API is the `Profile::with_tag` method, which returns a `TagSetter`.
//! This `TagSetter` uses a system of "capability traits" to ensure that only valid
//! data types can be associated with a given `TagSignature` at compile time.

use crate::{
    profile::{RawProfile, TagSetter},
    tag::TagSignature,
};

// Kind markers and trait to unify ensure/get/get_mut for TagData variants
pub trait TagDataKind {
    type Data: Default;
    fn as_ref(td: &crate::tag::tagdata::TagData) -> Option<&Self::Data>;
    fn as_mut(td: &mut crate::tag::tagdata::TagData) -> Option<&mut Self::Data>;
    fn wrap(data: Self::Data) -> crate::tag::tagdata::TagData;
}

// Macro to declare a Kind marker + TagDataKind impl in one go
macro_rules! tag_kind {
    ($kind:ident, $variant:ident, $data:path) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $kind;
        impl TagDataKind for $kind {
            type Data = $data;
            fn as_ref(td: &crate::tag::tagdata::TagData) -> Option<&Self::Data> {
                if let crate::tag::tagdata::TagData::$variant(c) = td {
                    Some(c)
                } else {
                    None
                }
            }
            fn as_mut(td: &mut crate::tag::tagdata::TagData) -> Option<&mut Self::Data> {
                if let crate::tag::tagdata::TagData::$variant(c) = td {
                    Some(c)
                } else {
                    None
                }
            }
            fn wrap(data: Self::Data) -> crate::tag::tagdata::TagData {
                crate::tag::tagdata::TagData::$variant(data)
            }
        }
    };
}

// Replace the manual marker structs/impls
tag_kind!(CurveKind, Curve, crate::tag::tagdata::CurveData);
tag_kind!(
    ParametricCurveKind,
    ParametricCurve,
    crate::tag::tagdata::ParametricCurveData
);
tag_kind!(SignatureKind, Signature, crate::tag::tagdata::SignatureData);

// Add: macro to generate {get, get_mut, ensure_mut} accessors
macro_rules! tag_accessors {
    ($lower:ident, $lower_mut:ident, $ensure_mut:ident, $variant:ident, $data:path, $kind:ident) => {
        pub fn $lower<S: Into<TagSignature>>(&self, tag: S) -> Option<&$data> {
            self.tag_data(tag).and_then(|td| {
                if let crate::tag::tagdata::TagData::$variant(c) = td {
                    Some(c)
                } else {
                    None
                }
            })
        }
        pub fn $lower_mut<S: Into<TagSignature>>(&mut self, tag: S) -> Option<&mut $data> {
            self.tag_data_mut(tag).and_then(|td| {
                if let crate::tag::tagdata::TagData::$variant(c) = td {
                    Some(c)
                } else {
                    None
                }
            })
        }
        pub fn $ensure_mut<S: Into<TagSignature> + Copy>(&mut self, tag: S) -> &mut $data {
            self.ensure_tag_mut::<$kind, _>(tag)
        }
    };
}

impl RawProfile {
    pub fn with_tag<S: Into<TagSignature> + Copy>(&mut self, tag: S) -> TagSetter<'_, S> {
        TagSetter::new(self, tag)
    }

    /// Get a shared reference to the TagData for a signature.
    pub fn tag_data<S: Into<TagSignature>>(&self, tag: S) -> Option<&crate::tag::tagdata::TagData> {
        let sig = tag.into();
        self.tags.get(&sig).map(|rec| rec.tag.data())
    }

    /// Get a mutable reference to the TagData for a signature.
    pub fn tag_data_mut<S: Into<TagSignature>>(
        &mut self,
        tag: S,
    ) -> Option<&mut crate::tag::tagdata::TagData> {
        let sig = tag.into();
        self.tags.get_mut(&sig).map(|rec| rec.tag.data_mut())
    }

    /// Generic: get or insert a specific TagData kind and return a mutable reference.
    pub fn ensure_tag_mut<K, S>(&mut self, tag: S) -> &mut K::Data
    where
        K: TagDataKind,
        S: Into<TagSignature> + Copy,
    {
        let sig = tag.into();
        let rec = match self.tags.entry(sig) {
            indexmap::map::Entry::Occupied(o) => o.into_mut(),
            indexmap::map::Entry::Vacant(v) => {
                let tag = crate::tag::Tag::new(sig.to_u32(), K::wrap(Default::default()));
                v.insert(crate::tag::ProfileTagRecord::new(0, 0, tag))
            }
        };

        if K::as_ref(rec.tag.data()).is_none() {
            rec.tag = crate::tag::Tag::new(sig.to_u32(), K::wrap(Default::default()));
        }

        K::as_mut(rec.tag.data_mut()).expect("ensured kind must be present")
    }

    // Curve
    tag_accessors!(
        curve,
        curve_mut,
        ensure_curve_mut,
        Curve,
        crate::tag::tagdata::CurveData,
        CurveKind
    );

    // ParametricCurve
    tag_accessors!(
        parametric_curve,
        parametric_curve_mut,
        ensure_parametric_curve_mut,
        ParametricCurve,
        crate::tag::tagdata::ParametricCurveData,
        ParametricCurveKind
    );

    // Signature
    tag_accessors!(
        signature,
        signature_mut,
        ensure_signature_mut,
        Signature,
        crate::tag::tagdata::SignatureData,
        SignatureKind
    );
}
