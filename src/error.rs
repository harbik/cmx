// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2024-2025, Harbers Bik LLC

use crate::signatures::Signature;

/// The top-level error type returned by most public API functions in this crate.
///
/// This enum is non-exhaustive — new variants may be added in future releases
/// without a semver-breaking change.
#[derive(thiserror::Error, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// A lower-level error produced while parsing the 128-byte ICC profile header.
    /// Carries a human-readable description of the specific field that failed.
    #[error(transparent)]
    HeaderParseError(#[from] HeaderParseError),

    /// A general string-parsing error, e.g. when converting a tag field to a
    /// Rust `String` fails.
    #[error(transparent)]
    ParseError(#[from] ParseError),

    /// The CMM (Color Management Module) signature field contains a value that
    /// is not recognised by this crate.
    #[error(transparent)]
    InvalidCmmError(#[from] InvalidCmmError),

    /// A CMM value was rejected for a reason described by the contained string.
    #[error("Invalid CMM: {0}")]
    InvalidCmm(&'static str),

    /// The Profile Connection Space (PCS) tag carries an unrecognised 4-byte
    /// signature.  Valid values are `XYZ ` and `Lab `.
    #[error("Invalid ICC Profile signature: {0}")]
    InvalidPcsTag(Signature),

    /// Returned by `TryFrom` conversions between `Profile` and a specific
    /// device-class wrapper (e.g. `DisplayProfile`) when the profile's
    /// `DeviceClass` field does not match the target type.
    #[error("Is not a {0}")]
    IsNotA(&'static str),

    /// The bytes passed to [`RawProfile::from_bytes`](crate::profile::RawProfile::from_bytes)
    /// do not begin with the mandatory `acsp` file signature or otherwise fail the minimum
    /// structural checks for a valid ICC profile.
    #[error("This is not a valid ICC profile")]
    InvalidICCProfile,

    /// The creation date/time fields in the ICC profile header contain
    /// out-of-range values (e.g. month 13, hour 25).  The inner string
    /// contains the raw date as read from the header.
    ///
    /// Returned by [`RawProfile::creation_date`](crate::profile::RawProfile::creation_date).
    #[error("Invalid date/time in ICC profile header: {0}")]
    InvalidDate(String),

    /// [`ParametricCurveData::set_parameters_slice`](crate::tag::tagdata::ParametricCurveData::set_parameters_slice)
    /// was called with a slice whose length is not one of the five ICC-defined counts
    /// (1, 3, 4, 5, or 7).  Each count corresponds to a specific parametric curve function
    /// type in the ICC specification (Table 68 in ICC.1:2022).
    #[error("Unsupported parametric curve parameter count: {0} (must be 1, 3, 4, 5, or 7)")]
    UnsupportedParameterCount(usize),

    /// [`RawProfile::into_bytes`](crate::profile::RawProfile::into_bytes) produced a
    /// serialised profile whose size exceeds 2^32 − 1 bytes.  The ICC specification stores
    /// the profile size in a 32-bit field, so larger profiles cannot be represented.
    #[error("Profile exceeds maximum ICC size of 4 GiB ({0} bytes)")]
    ProfileTooLarge(usize),
}

/// Generates a newtype error struct wrapping a `String`, with a `new()` constructor
/// and a `From<&str>` impl for convenient construction from string literals.
///
/// Used internally to create [`HeaderParseError`], [`ParseError`], and
/// [`InvalidCmmError`].
macro_rules! define_string_error {
    ($name:ident, $msg:literal) => {
        #[derive(thiserror::Error, Debug, PartialEq)]
        #[error($msg)]
        pub struct $name(pub String);

        impl $name {
            pub fn new<T: Into<String>>(msg: T) -> Self {
                $name(msg.into())
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                $name(s.to_string())
            }
        }
    };
}

define_string_error!(HeaderParseError, "Could not parse ICC Profile header: {0}");
define_string_error!(ParseError, "String parse error: {0}");
define_string_error!(InvalidCmmError, "Invalid CMM: {0}");
