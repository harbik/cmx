
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2024-2025, Harbers Bik LLC

use crate::signatures::Signature;

#[derive(thiserror::Error, Debug, PartialEq)]
#[non_exhaustive]

pub enum Error {
    #[error("Could not parse ICC Profile header (0) ")]
    HeaderParseError(String),
    #[error("String parse error: {0}")]
    ParseError(String),
    #[error("Invalid ICC Profile signature: {0}")]
    InvalidPcsTag(Signature),
    #[error("Is not a {0}")]
    IsNotA(&'static str),
    #[error("Invalid CMM: {0}")]
    InvalidCmm(&'static str),
    #[error("This is not a valid ICC profile")]
    InvalidICCProfile,
}

