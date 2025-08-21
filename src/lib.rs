// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//#![allow(dead_code, unused_imports)]

//! This crate provides utilities for working with ICC color profiles
//! and integrates with the Colorimetry Library.
//!
//! ## Use Cases
//! <details><summary><strong>Parsing ICC profiles and conversion to TOML format for analysis</strong></summary>
//! After installing the library, you can parse an ICC profile and convert it to a TOML format using the `cmx` command-line tool:
//!
//! ```bash
//! cmx profile.icc -o profile.toml
//! ```
//!
//! Each ICC profile tag is mapped to a key in the TOML file, with the
//! corresponding values serialized as key-value pairs.
//! All values are written as single-line entries to ensure the TOML output
//! remains human-readable and easy to inspect.
//!
//! Example of a parsed ICC profile in TOML format:
//!
//! ```toml
//! profile_size = 548
//! cmm = "Apple"
//! version = "4.0"
//! device_class = "Display"
//! color_space = "RGB"
//! pcs = "XYZ"
//! creation_datetime = "2015-10-14 13:08:56 UTC"
//! primary_platform = "Apple"
//! manufacturer = "APPL"
//! rendering_intent = "Perceptual"
//! pcs_illuminant = [0.9642, 1.0, 0.8249]
//! creator = "appl"
//! profile_id = "53410ea9facdd9fb57cc74868defc33f"
//!
//! [desc]
//! ascii = "SMPTE RP 431-2-2007 DCI (P3)"
//!
//! [cprt]
//! text = "Copyright Apple Inc., 2015"
//!
//! [wtpt]
//! xyz = [0.89459228515625, 1.0, 0.9544219970703125]
//!
//! [rXYZ]
//! xyz = [0.4861602783203125, 0.2266845703125, -0.0008087158203125]
//!
//! [gXYZ]
//! xyz = [0.3238525390625, 0.7103271484375, 0.0432281494140625]
//!
//! [bXYZ]
//! xyz = [0.1541900634765625, 0.06298828125, 0.782470703125]
//!
//! [chad]
//! matrix = [
//!     [1.073822, 0.038803, -0.036896],
//!     [0.055573, 0.963989, -0.014343],
//!     [-0.004272, 0.005295, 0.862778]
//! ]
//!
//! [rTRC]
//! g = 2.6
//!
//! [bTRC]
//! g = 2.6
//!
//! [gTRC]
//! g = 2.6
//!
//!  ```
//! </details>
//! <details><summary><strong>Creating ICC profiles programmatically</strong></summary>
//! You can also use the `cmx` library to create ICC profiles programmatically in Rust.
//! The library provides a builder-style API for constructing profiles,
//! allowing you to set various tags and properties.
//! //! Example of creating a simple ICC profile:
//! //! ```rust
//! use cmx::profile::DisplayProfile;
//! use cmx::tag::tags::{ChromaticityTag, ProfileDescriptionTag};
//! use cmx::tag::tagdata::{ChromaticityData, MultiLocalizedUnicodeData};   
//! let profile = DisplayProfile::new()
//!     .with_profile_version(4, 4)
//!     .with_creation_date(None)
//!     .with_tag(ChromaticityTag)
//!     .with_data(|data| {
//!         data.set_standard(cmx::tag::Primaries::ITU);
//!     })
//!     .with_tag(ProfileDescriptionTag)
//!         .as_multi_localized_unicode(|mlu| {
//!             mlu.set_ascii("My Display Profile");
//!             mlu.set_language("en");
//!             mlu.set_text("This is a custom display profile");
//!     })
//!
//!
//!
//! ## Installation
//!
//! Install the `cmx` tool using Cargo:
//!
//! ```bash
//! cargo install cmx
//! ```
//!
//! To use the `cmx` library in your Rust project:
//!
//! ```bash
//! cargo add cmx
//! ```
//!
//! Documentation is available at [docs.rs/cmx](https://docs.rs/cmx).
//!
//! ## Roadmap
//!
//! - [x] Parse full ICC profiles
//! - [x] Convert to TOML format
//! - [ ] Support more ICC tags and color models
//! - [ ] Add builder-style API for constructing ICC profiles
//! - [ ] Enable spectral data and advanced color management
//! - [ ] Provide utilities for profile conversion and manipulation
//!
//! ## Overview
//!
//! Although the ICC specification is broad and complex, this crate aims
//! to provide a robust foundation for working with ICC profiles in Rust.
//!
//! It supports parsing, constructing, and changing of the primary ICC-defined tags,
//! as well as some commonly used non-standard tags.
//!
//! Even tags that cannot yet be parsed are still preserved when reading
//! and serializing profiles, ensuring no data loss.
//!
//! The long-term goal is to fully support advanced ICC color management,
//! including spectral data and extended color models, while maintaining
//! compatibility with existing profiles.

pub mod error;
pub mod header;
pub mod language;
pub mod profile;
pub mod signatures;
pub mod tag;

use std::fmt::Display;

pub use error::Error;
use num::Zero;

/// Rounds a floating-point value to the specified precision (decimal places).
/// Example: round_to_precision(1.23456, 2) -> 1.23
pub(crate) fn round_to_precision(value: f64, precision: i32) -> f64 {
    let multiplier = 10f64.powi(precision);
    (value * multiplier).round() / multiplier
}

/// Generic zero-check used by serde skip_serializing_if for many numeric fields.
pub(crate) fn is_zero<T: Zero>(n: &T) -> bool {
    n.is_zero()
}

/// Treats the string as "empty" if it is empty or equals "none" (case-sensitive),
/// used to suppress serialization for some optional fields.
pub(crate) fn is_empty_or_none(s: &String) -> bool {
    s.is_empty() || s == "none"
}

/// Convert ICC s15Fixed16Number to f64 by dividing by 65536.0.
/// This is a signed 32-bit fixed-point with 16 fractional bits.
pub(crate) fn s15fixed16(v: i32) -> f64 {
    round_to_precision(v as f64 / 65536.0, 6)
}

/// Convert ICC u1.15 fixed number (u16) to f64.
/// Range is [0, ~1.99997]. We scale by (65535 / 32768) and round for compact output.
pub(crate) fn u1_fixed15_number(v: u16) -> f64 {
    const SCALE: f64 = 0xFFFF as f64 / 0x8000 as f64;
    round_to_precision(v as f64 * SCALE, 6)
}

/// Render bytes as uppercase hex grouped into 4-byte (8-hex) chunks separated by spaces.
/// Example: [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC] -> "12345678 9ABC"
pub(crate) fn format_hex_with_spaces(data: &[u8]) -> String {
    let hex = hex::encode_upper(data);

    // Split into chunks of 8 characters and join with spaces
    hex.as_bytes()
        .chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<&str>>()
        .join(" ")
}

use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, I32};
#[derive(FromBytes, IntoBytes, Unaligned, KnownLayout, Immutable, Debug, Clone, Copy)]
#[repr(C)]
pub struct S15Fixed16(I32<BigEndian>);

/// A 15.16 fixed-point number, where the first 15 bits are the integer part and the last 16 bits are the fractional part.
/// This is used in ICC profiles to represent color values.
/// The value is stored as a 32-bit signed integer in big-endian format.
impl From<S15Fixed16> for f64 {
    fn from(value: S15Fixed16) -> Self {
        let s15 = value.0.get();
        s15 as f64 / 65536.0
    }
}

impl From<f64> for S15Fixed16 {
    fn from(value: f64) -> Self {
        let s15 = (value * 65536.0).round() as i32;
        S15Fixed16(I32::new(s15))
    }
}

impl From<S15Fixed16> for I32<BigEndian> {
    fn from(value: S15Fixed16) -> Self {
        value.0
    }
}

impl Display for S15Fixed16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", f64::from(*self))
    }
}
