
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2024-2025, Harbers Bik LLC

#[derive(thiserror::Error, Debug, PartialEq)]
#[non_exhaustive]

pub enum Error {
    #[error("Could not parse ICC Profile header (0) ")]
    HeaderParseError(String),
    #[error("String parse error: {0}")]
    ParseError(String),
}

