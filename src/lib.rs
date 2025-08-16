// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//#![allow(dead_code, unused_imports)]
//! This crate provides a set of utilities for working with ICC Color Profiles
//! and the Colorimetry Library.
//!
//! The main functionality at this stage it to parse ICC profiles, and convert them
//! to TOML format using the cmx command line tool:
//!
//! ```bash
//! cmx profile.icc -o profile.toml
//!  ```
//!
//! Every ICC profile tag is converted to a key in the TOML file, with the tag's
//! values serialized to key-value pairs.
//! The values are all given as single line output, so that the TOML file is
//! human-readable and easy to inspect.
//!
//! Install the `cmx` tool using Cargo:
//!
//! ```bash
//! cargo install cmx
//! ```
//!
//! To use the `cmx` library, run the following command in your Rust project:
//!
//! ```bash
//! cargo add cmx
//! ```
//!
//! Its documentation is available at [docs.rs/cmx](https://docs.rs/cmx).
//!
//! # Roadmap
//!
//! - [X] Implement a full ICC profile parser
//! - [X] Convert to TOML file format
//! - [ ] Parse TOML files back to ICC profiles
//! - [ ] Create ICC profiles using the [`Colorimetry`](https://docs.rs/colorimetry/latest/colorimetry/) library features
//!
//! The intention is to fully support advanced ICC Color management,
//! with the ability to use spectral data, and advanced color models,
//! while maintaining compatibility with existing ICC profiles.
//!

pub mod error;
pub mod header;
pub mod language;
pub mod profile;
pub mod signatures;
pub mod tag;

pub use error::Error;
use num::Zero;

/// Rounds a floating-point value to the specified precision.
/// For example, round_to_precision(1.23456, 100.0) returns 1.23.
pub(crate) fn round_to_precision(value: f64, precision: i32) -> f64 {
    let multiplier = 10f64.powi(precision);
    (value * multiplier).round() / multiplier
}

// Add this helper function
// Make the helper function generic
fn is_zero<T: Zero>(n: &T) -> bool {
    n.is_zero()
}

// Add this function at module level
fn is_empty_or_none(s: &String) -> bool {
    s.is_empty() || s == "none"
}


fn from_s15fixed16(v: i32) -> f64 {
    round_to_precision(v as f64 / 65536.0, 6)
}

fn format_hex_with_spaces(data: &[u8]) -> String {
    let hex = hex::encode(data);
    
    // Split into chunks of 8 characters and join with spaces
    hex.as_bytes()
        .chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<&str>>()
        .join(" ")
}