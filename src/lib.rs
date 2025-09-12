// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//#![allow(dead_code, unused_imports)]

/*!
This crate provides utilities for working with ICC color profiles
and integrates with the Colorimetry Library.

## Use Cases
<details><summary><strong>Parsing ICC profiles and conversion to TOML format for analysis</strong></summary>
After installing the library, you can parse an ICC profile and convert it to a TOML format using the `cmx` command-line tool:

```bash
cmx profile.icc -o profile.toml
```

Each ICC profile tag is mapped to a key in the TOML file, with the
corresponding values serialized as key-value pairs.
All values are written as single-line entries to ensure the TOML output
remains human-readable and easy to inspect.

Example of a parsed ICC profile in TOML format:

```toml
profile_size = 548
cmm = "Apple"
version = "4.0"
device_class = "Display"
color_space = "RGB"
pcs = "XYZ"
creation_datetime = "2015-10-14 13:08:56 UTC"
primary_platform = "Apple"
manufacturer = "APPL"
rendering_intent = "Perceptual"
pcs_illuminant = [0.9642, 1.0, 0.8249]
creator = "appl"
profile_id = "53410ea9facdd9fb57cc74868defc33f"

[desc]
ascii = "SMPTE RP 431-2-2007 DCI (P3)"

[cprt]
text = "Copyright Apple Inc., 2015"

[wtpt]
xyz = [0.894592, 1.0, 0.954422]

[rXYZ]
xyz = [0.48616, 0.226685, -0.000809]

[gXYZ]
xyz = [0.323853, 0.710327, 0.043228]

[bXYZ]
xyz = [0.15419, 0.062988, 0.782471]

[rTRC]
g = 2.60001

[chad]
matrix = [
    [1.073822, 0.038803, -0.036896],
    [0.055573, 0.963989, -0.014343],
    [-0.004272, 0.005295, 0.862778]
]

[bTRC]
g = 2.60001

[gTRC]
g = 2.60001

 ```
</details>

<details><summary><strong>Generate ICC profiles</strong></summary>

You can also use the `cmx` library to create ICC profiles from scratch, or read existing
profiles and change them, using Rust.

The library provides a builder-style API for constructing, or read and change profiles,
allowing you to set or change various tags and properties.

Here is an example for creating a Display P3 ICC profile:

```rust
use chrono::{DateTime, TimeZone};
use cmx::tag::tags::*;
use cmx::profile::DisplayProfile;
let display_p3_example = DisplayProfile::new()
    // set creation date, if omitted, the current date and time are used
    .with_creation_date(chrono::Utc.with_ymd_and_hms(2025, 8, 28, 0, 0, 0).unwrap())
    .with_tag(ProfileDescriptionTag)
        .as_text_description(|text| {
            text.set_ascii("Display P3");
        })
    .with_tag(CopyrightTag)
        .as_text(|text| {
            text.set_text("CC0");
        })
    .with_tag(MediaWhitePointTag)
        .as_xyz_array(|xyz| {
            xyz.set([0.950455, 1.00000, 1.08905]);
        })
    .with_tag(RedMatrixColumnTag)
        .as_xyz_array(|xyz| {
            xyz.set([0.515121, 0.241196, -0.001053]);
        })
    .with_tag(GreenMatrixColumnTag)
        .as_xyz_array(|xyz| {
            xyz.set([0.291977, 0.692245, 0.041885]);
        })
    .with_tag(BlueMatrixColumnTag)
        .as_xyz_array(|xyz| {
            xyz.set([0.157104, 0.066574, 0.784073]);
        })
    .with_tag(RedTRCTag)
        .as_parametric_curve(|para| {
            para.set_parameters([2.39999, 0.94786, 0.05214, 0.07739, 0.04045]);
        })
    .with_tag(BlueTRCTag)
        .as_parametric_curve(|para| {
            para.set_parameters([2.39999, 0.94786, 0.05214, 0.07739, 0.04045]);
        })
    .with_tag(GreenTRCTag)
        .as_parametric_curve(|para| {
            para.set_parameters([2.39999, 0.94786, 0.05214, 0.07739, 0.04045]);
        })
    .with_tag(ChromaticAdaptationTag)
        .as_sf15_fixed_16_array(|array| {
            array.set([
                 1.047882, 0.022919, -0.050201,
                 0.029587, 0.990479, -0.017059,
                -0.009232, 0.015076,  0.751678
            ]);
        })
    .with_profile_id() // calculate and add profile ID to the profile
    ;

display_p3_example.write("tmp/display_p3_example.icc").unwrap();
let display_p3_read_back = cmx::profile::Profile::read("tmp/display_p3_example.icc").unwrap();
assert_eq!(
    display_p3_read_back.profile_id_as_hex_string(),
    "617028e1 e1014e15 91f178a9 fb8efc92"
);
assert_eq!(display_p3_read_back.profile_size(), 524);
```

Not all ICC tag types are supported yet, but please submit a pull request, or an issue, on our
[GitHub CMX repo](https://github.com/harbik/cmx) if you want additional tag types to be supported.

However, you can use the `as_raw` method to set raw data for tags that are not yet supported.

</details>



## Installation

Install the `cmx` tool using Cargo:

```bash
cargo install cmx
```

To use the `cmx` library in your Rust project:

```bash
cargo add cmx
```

Documentation is available at [docs.rs/cmx](https://docs.rs/cmx).

## Roadmap

- [x] Parse full ICC profiles
- [x] Convert to TOML format
- [x] Add builder-style API for constructing ICC profiles
- [x] Support basic ICC Type tags and color models
- [ ] Read TOML Color profiles and convert to binary ICC profiles
- [ ] Utilities for commandline profile conversion and manipulation
- [ ] Calibration and profiling tools
- [ ] X-Rite I1 Profiler support
- [ ] Support all ICC Type tags
- [ ] Enable spectral data and advanced color management



## Overview

Although the ICC specification is broad and complex, this crate aims
to provide a robust foundation for working with ICC profiles in Rust.

It supports parsing, constructing, and changing of the primary ICC-defined tags,
as well as some commonly used non-standard tags.

Even tags that cannot yet be parsed are still preserved when reading
and serializing profiles, ensuring no data loss.

The long-term goal is to fully support advanced ICC color management,
including spectral data and extended color models, while maintaining
compatibility with existing profiles.
*/

pub mod error;
pub mod header;
//pub mod language;
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
/// Example: [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC] -> "12345678 9abc"
/// This is used for displaying binary data in a human-readable format.
///
/// Example:
/// ```
///  let data = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
///  let formatted = cmx::format_hex_with_spaces(&data);
///  assert_eq!(formatted, "12345678 9abc");
/// ```
///
/// Note: The last chunk may be shorter than 8 characters if the data length is not a multiple of 4.
///
pub fn format_hex_with_spaces(data: &[u8]) -> String {
    let hex = hex::encode(data);

    // Split into chunks of 8 characters and join with spaces
    hex.as_bytes()
        .chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Parse a hex string with optional spaces into a byte vector.
/// Example: "12345678 9abc" -> [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]
/// This is used for converting human-readable hex strings back into binary data.
///
/// Example:
/// ```
///   let hex_str = "12345678 9abc";
///   let bytes = cmx::parse_hex_string(hex_str).unwrap();
///   assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]);
/// ```
///
/// Notes:
/// * The input string can contain spaces and is case-insensitive.
/// * Non-hex characters and whitespace are ignored.
///
pub fn parse_hex_string(s: &str) -> Result<Vec<u8>, hex::FromHexError> {
    let cleaned: String = s
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_ascii_hexdigit())
        .collect();
    hex::decode(cleaned)
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
        round_to_precision(s15 as f64 / 65536.0, 5)
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

impl From<I32<BigEndian>> for S15Fixed16 {
    fn from(value: I32<BigEndian>) -> Self {
        Self(value)
    }
}
impl Display for S15Fixed16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", f64::from(*self))
    }
}

fn pad_size(len: usize) -> usize {
    //    ((len + 3) / 4) * 4 - len
    len.div_ceil(4) * 4 - len
}

fn padded_size(len: usize) -> usize {
    //    ((len + 3) / 4) * 4
    len.div_ceil(4) * 4
}

pub fn is_printable_ascii_bytes(b: &[u8]) -> bool {
    b.iter().all(|&x| (0x20..=0x7E).contains(&x))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round_to_precision() {
        assert_eq!(round_to_precision(1.23456, 2), 1.23);
        assert_eq!(round_to_precision(1.23456, 3), 1.235);
        assert_eq!(round_to_precision(1.23456, 0), 1.0);
    }

    #[test]
    fn test_s15fixed16() {
        let value = S15Fixed16::from(1.5);
        assert_eq!(f64::from(value), 1.5);
        assert_eq!(value.to_string(), "1.5");
    }

    #[test]
    fn test_format_hex_with_spaces() {
        let data = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
        let formatted = format_hex_with_spaces(&data);
        assert_eq!(formatted, "12345678 9abc");
    }
    #[test]
    fn test_pad() {
        assert_eq!(pad_size(0), 0);
        assert_eq!(pad_size(1), 3);
        assert_eq!(pad_size(2), 2);
        assert_eq!(pad_size(3), 1);
        assert_eq!(pad_size(4), 0);
        assert_eq!(pad_size(5), 3);
        assert_eq!(pad_size(6), 2);
        assert_eq!(pad_size(7), 1);
        assert_eq!(pad_size(8), 0);
    }
}
