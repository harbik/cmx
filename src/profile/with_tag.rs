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

#[derive(Debug, Clone, Copy, Default)]
pub struct CurveKind;

impl TagDataKind for CurveKind {
    type Data = crate::tag::tagdata::CurveData;
    fn as_ref(td: &crate::tag::tagdata::TagData) -> Option<&Self::Data> {
        if let crate::tag::tagdata::TagData::Curve(c) = td {
            Some(c)
        } else {
            None
        }
    }
    fn as_mut(td: &mut crate::tag::tagdata::TagData) -> Option<&mut Self::Data> {
        if let crate::tag::tagdata::TagData::Curve(c) = td {
            Some(c)
        } else {
            None
        }
    }
    fn wrap(data: Self::Data) -> crate::tag::tagdata::TagData {
        crate::tag::tagdata::TagData::Curve(data)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ParametricCurveKind;

impl TagDataKind for ParametricCurveKind {
    type Data = crate::tag::tagdata::ParametricCurveData;
    fn as_ref(td: &crate::tag::tagdata::TagData) -> Option<&Self::Data> {
        if let crate::tag::tagdata::TagData::ParametricCurve(c) = td {
            Some(c)
        } else {
            None
        }
    }
    fn as_mut(td: &mut crate::tag::tagdata::TagData) -> Option<&mut Self::Data> {
        if let crate::tag::tagdata::TagData::ParametricCurve(c) = td {
            Some(c)
        } else {
            None
        }
    }
    fn wrap(data: Self::Data) -> crate::tag::tagdata::TagData {
        crate::tag::tagdata::TagData::ParametricCurve(data)
    }
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

    /// Convenience: get CurveData if present for this signature.
    pub fn curve<S: Into<TagSignature>>(&self, tag: S) -> Option<&crate::tag::tagdata::CurveData> {
        self.tag_data(tag).and_then(|td| {
            if let crate::tag::tagdata::TagData::Curve(c) = td {
                Some(c)
            } else {
                None
            }
        })
    }

    /// Convenience: get mutable CurveData if present for this signature.
    pub fn curve_mut<S: Into<TagSignature>>(
        &mut self,
        tag: S,
    ) -> Option<&mut crate::tag::tagdata::CurveData> {
        self.tag_data_mut(tag).and_then(|td| {
            if let crate::tag::tagdata::TagData::Curve(c) = td {
                Some(c)
            } else {
                None
            }
        })
    }

    /// Get or insert a CurveData for a signature and return a mutable reference.
    pub fn ensure_curve_mut<S: Into<TagSignature> + Copy>(
        &mut self,
        tag: S,
    ) -> &mut crate::tag::tagdata::CurveData {
        self.ensure_tag_mut::<CurveKind, _>(tag)
    }

    /// Convenience: get ParametricCurveData if present for this signature.
    pub fn parametric_curve<S: Into<TagSignature>>(
        &self,
        tag: S,
    ) -> Option<&crate::tag::tagdata::ParametricCurveData> {
        self.tag_data(tag).and_then(|td| {
            if let crate::tag::tagdata::TagData::ParametricCurve(c) = td {
                Some(c)
            } else {
                None
            }
        })
    }

    /// Convenience: get mutable ParametricCurveData if present for this signature.
    pub fn parametric_curve_mut<S: Into<TagSignature>>(
        &mut self,
        tag: S,
    ) -> Option<&mut crate::tag::tagdata::ParametricCurveData> {
        self.tag_data_mut(tag).and_then(|td| {
            if let crate::tag::tagdata::TagData::ParametricCurve(c) = td {
                Some(c)
            } else {
                None
            }
        })
    }

    /// Get or insert a ParametricCurveData for a signature and return a mutable reference.
    pub fn ensure_parametric_curve_mut<S: Into<TagSignature> + Copy>(
        &mut self,
        tag: S,
    ) -> &mut crate::tag::tagdata::ParametricCurveData {
        self.ensure_tag_mut::<ParametricCurveKind, _>(tag)
    }
}
