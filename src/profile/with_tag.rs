// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! This module provides a type-safe builder API for constructing ICC profiles.
//!
//! The core of the API is the `Profile::` method, which returns a `TagSetter`.
//! This `TagSetter` uses a system of "capability traits" to ensure that only valid
//! data types can be associated with a given `TagSignature` at compile time.

use crate::{
    profile::{RawProfile, TagSetter}, tag::TagSignature,
};

impl RawProfile {
    pub fn with_tag<S: Into<TagSignature> + Copy>(
        &mut self,
        signature: S,
    ) -> TagSetter<'_, S> {
        TagSetter::new(self, signature) 
    }
}

/*
//! This module provides a type-safe builder API for constructing ICC profiles.
//!
//! The core of the API is the `Profile::with_tag` method, which returns a `TagSetter`.
//! This `TagSetter` uses a system of "capability traits" to ensure that only valid
//! data types can be associated with a given `TagSignature` at compile time.

use crate::profile::Profile;
use crate::signatures::{tag, TagSignature};
use crate::tags::{
    CurveType, MultiLocalizedUnicodeType, TagData, TextDescriptionType,
    // ... import other tag data types as you need them ...
};

// --------------------------------------------------------------------------------
// STEP 1: Define "Capability" Marker Traits
// These traits identify what a TagSignature is allowed to contain.
// --------------------------------------------------------------------------------

/// A marker trait for signatures that can be a `curveType`.
pub trait IsCurveTag {}

/// A marker trait for signatures that can be a `textDescriptionType`.
pub trait IsTextDescriptionTag {}

/// A marker trait for signatures that can be a `multiLocalizedUnicodeType`.
pub trait IsMultiLocalizedUnicodeTag {}

// ... define other marker traits for each possible data type ...

// --------------------------------------------------------------------------------
// STEP 2: Define the `UnambiguousTag` Trait for Ergonomics
// This trait is for tags that have only ONE valid data type.
// --------------------------------------------------------------------------------

/// A trait for tag signatures that have only one valid data type.
pub trait UnambiguousTag {
    /// The single data type associated with this tag signature.
    type DataType: Default;

    /// A function to create the correct `TagData` enum variant from the data.
    fn new_tag(data: Self::DataType) -> TagData;
}

/// A helper macro to reduce boilerplate when implementing `UnambiguousTag`.
macro_rules! impl_unambiguous_tag {
    // Takes the tag constant, its data type, and the corresponding TagData enum variant.
    ($tag_const:path, $data_type:ty, $tag_variant:ident) => {
        impl UnambiguousTag for $tag_const {
            type DataType = $data_type;
            fn new_tag(data: Self::DataType) -> TagData {
                TagData::$tag_variant(data)
            }
        }
    };
}

// --------------------------------------------------------------------------------
// STEP 3: Implement the Traits for Known TagData Signatures
// This is where you encode the ICC specification rules into the type system.
// --------------------------------------------------------------------------------

// --- Implementations for Unambiguous Tags ---
// impl_unambiguous_tag!(tag::RedTRC, CurveType, Curve);
// impl_unambiguous_tag!(tag::GreenTRC, CurveType, Curve);
// impl_unambiguous_tag!(tag::BlueTRC, CurveType, Curve);
// impl_unambiguous_tag!(tag::Copyright, TextDescriptionType, TextDescription);
// ... etc.

// --- Implementations for Ambiguous Tags ---
// The 'desc' tag can be either of these two types.
// impl IsTextDescriptionTag for tag::Desc {}
// impl IsMultiLocalizedUnicodeTag for tag::Desc {}

// The 'rTRC' tag can only be a curve.
// impl IsCurveTag for tag::RedTRC {}
// ... etc.

// --------------------------------------------------------------------------------
// STEP 4: The Generic `TagSetter` Struct
// This struct provides the type-safe methods for setting tag data.
// --------------------------------------------------------------------------------


// --------------------------------------------------------------------------------
// STEP 5: The Entry Point in the `Profile`
// --------------------------------------------------------------------------------

impl Profile {
    /// Begins the process of adding or replacing a tag in the profile.
    ///
    /// This returns a `TagSetter` helper struct, which provides type-safe methods
    /// (e.g., `.as_curve()`, `.with_data()`) to define the tag's data.
    pub fn with_tag<S: Into<TagSignature> + Copy>(&mut self, signature: S) -> TagSetter<'_, S> {
        TagSetter {
            profile: self,
            signature,
        }
    }
}

// --------------------------------------------------------------------------------
// STEP 6: Usage Examples
// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::profile::Profile;
    // use crate::signatures::tag;

    #[test]
    fn test_builder_api() {
        // let mut profile = Profile::new();

        // --- Example 1: Unambiguous TagData ---
        // The compiler knows `tag::RedTRC` is an `UnambiguousTag` whose `DataType`
        // is `CurveType`. The `.with_data()` method is available, and the closure
        // argument `curve` is correctly inferred as `&mut CurveType`.
        //
        // profile.with_tag(tag::RedTRC).with_data(|curve| {
        //     curve.set_gamma(1.8);
        // });

        // --- Example 2: Ambiguous TagData ---
        // The compiler knows `tag::Desc` does NOT implement `UnambiguousTag`, so
        // a call to `.with_data()` would be a compile error.
        //
        // Instead, the user must choose one of the valid `.as_...()` methods.
        //
        // profile.with_tag(tag::Desc).as_multi_localized_unicode(|mlu| {
        //     mlu.add_text("en-US", "My Profile Description");
        //     mlu.add_text("de-DE", "Meine Profilbeschreibung");
        // });
        //
        // --- Example 3: Compile-Time Error ---
        // This demonstrates the safety of the API. The compiler knows that
        // `tag::RedTRC` does not implement `IsTextDescriptionTag`, so the
        // `.as_text_description()` method is not available for it.
        //
        // The following line would fail to compile:
        //
        // profile.with_tag(tag::RedTRC).as_text_description(|text| {
        //     text.set_ascii("This is not allowed!");
        // });
    }
}
 */
