// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2024-2025, Harbers Bik LLC

use crate::signatures::Signature;

#[derive(thiserror::Error, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    HeaderParseError(#[from] HeaderParseError),

    #[error(transparent)]
    ParseError(#[from] ParseError),

    #[error(transparent)]
    InvalidCmmError(#[from] InvalidCmmError),

    #[error("Invalid CMM: {0}")]
    InvalidCmm(&'static str),

    #[error("Invalid ICC Profile signature: {0}")]
    InvalidPcsTag(Signature),
    #[error("Is not a {0}")]
    IsNotA(&'static str),
    #[error("This is not a valid ICC profile")]
    InvalidICCProfile,
}

/// Generates a `new()` constructor and a `From<&str>` impl
/// for an error wrapper around a `String`.
///
/// Usage:
///
/// ```
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
