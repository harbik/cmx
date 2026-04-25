// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2026, Harbers Bik LLC

//#![allow(dead_code, unused_imports)]

/*!
`cmx` is a Rust library for reading, writing, and constructing [ICC color profiles][icc-spec]
(versions 2.0–5.0).  ICC profiles describe how color values produced by a device (camera,
display, printer) relate to a standard reference color space, making them essential for
accurate color reproduction across devices and applications.

[icc-spec]: https://www.color.org/specification/ICC.1-2022-05.pdf

# Quick Start

## Read a profile from disk

```no_run
use cmx::profile::Profile;

let profile = Profile::read("profile.icc")?;
println!("Color space : {:?}", profile.data_color_space());
println!("PCS         : {:?}", profile.pcs());
println!("Version     : {:?}", profile.version()?);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Read header fields

All header fields are available as typed accessors on every profile type — [`profile::Profile`],
[`profile::RawProfile`], [`profile::DisplayProfile`], and all other device-class wrappers.
They decode the 128-byte ICC header into Rust values:

```no_run
use cmx::profile::Profile;

let profile = Profile::read("profile.icc")?;

let (major, minor)     = profile.version()?;           // e.g. (4, 0)
let color_space        = profile.data_color_space();   // Option<ColorSpace>
let pcs                = profile.pcs();                // Option<Pcs>
let intent             = profile.rendering_intent();   // RenderingIntent
let created            = profile.creation_date()?;     // DateTime<Utc>
let platform           = profile.primary_platform();   // Option<Platform>
let cmm                = profile.cmm();                // Option<Cmm>
let manufacturer       = profile.manufacturer();       // Option<Signature>
let (embedded, excl)   = profile.flags();             // (bool, bool)
let illuminant         = profile.pcs_illuminant();    // [f64; 3]
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Read tag values

Tag data is accessible via `.tags()` on every profile type — [`profile::Profile`],
[`profile::RawProfile`], [`profile::DisplayProfile`], and all other device-class wrappers.
It returns an `IndexMap` keyed by [`tag::TagSignature`], with tags in the order they appear
in the profile's tag table:

```no_run
use cmx::profile::Profile;
use cmx::tag::TagSignature;
use cmx::tag::tagdata::parametric_curve::ParametricCurveType;

let profile = Profile::read("profile.icc")?;

// Iterate every tag in the profile
for (sig, record) in profile.tags() {
    println!("{sig}: {} bytes", record.tag.len());
}

// Look up a specific tag and get a serializable representation
if let Some(record) = profile.tags().get(&TagSignature::MediaWhitePoint) {
    let parsed = record.tag.to_parsed(); // -> ParsedTag (implements serde::Serialize)
}

// Type-specific read: parametric curve parameters (gamma, a, b, c, d, e, f)
if let Some(record) = profile.tags().get(&TagSignature::RedTRC) {
    if let Some(curve_data) = record.tag.data().as_parametric_curve() {
        let curve = ParametricCurveType::from(curve_data);
        let [g, a, b, c, d, e, f] = curve.values();
        println!("gamma = {g}");
    }
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

[`tag::ParsedTag`] is a `serde::Serialize`-able enum covering all supported tag types.
For the quickest way to inspect all tags at once, serialize the whole profile to TOML
(see [Dump a profile as TOML](#dump-a-profile-as-toml) below).

## Dump a profile as TOML

The [`fmt::Display`](std::fmt::Display) implementation on every profile type serialises it
to TOML — the same output produced by the `cmx` CLI tool:

```no_run
use cmx::profile::Profile;

let profile = Profile::read("profile.icc")?;
println!("{profile}");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The output looks like:

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

[wtpt]
xyz = [0.894592, 1.0, 0.954422]

[rTRC]
g = 2.60001

[chad]
matrix = [
    [1.073822, 0.038803, -0.036896],
    [0.055573, 0.963989, -0.014343],
    [-0.004272, 0.005295, 0.862778]
]
```

## Build a profile from scratch

The consuming builder API sets tags one by one and computes the profile ID at the end:

```rust
use chrono::{DateTime, TimeZone};
use cmx::tag::tags::*;
use cmx::profile::DisplayProfile;

let display_p3_example = DisplayProfile::new()
    // set creation date — current date/time is used if omitted
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
    .with_profile_id(); // compute and embed the MD5 profile ID

// Serialise to bytes without touching the filesystem
let bytes = display_p3_example.to_bytes().unwrap();
assert_eq!(bytes.len(), 524);
```

## Modify an existing profile

Read a profile, change a tag, and write it back using the builder API:

```no_run
use cmx::profile::Profile;
use cmx::tag::tags::CopyrightTag;

Profile::read("input.icc")?
    .with_tag(CopyrightTag)
        .as_text(|t| t.set_text("Copyright 2025 Acme Corp."))
    .write("output.icc")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Use [`profile::Profile::tags_mut`] to remove tags — something the builder API does not support:

```no_run
use cmx::profile::Profile;
use cmx::tag::TagSignature;

let mut profile = Profile::read("input.icc")?;
profile.tags_mut().remove(&TagSignature::MakeAndModel);
profile.write("output.icc")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Modules

| Module | Contents |
| --- | --- |
| [`profile`] | `Profile` enum and per-device-class types (`DisplayProfile`, `InputProfile`, …) |
| [`tag`] | Tag signatures, tag data types, and the `TagSetter` builder |
| [`header`] | ICC 128-byte header fields and accessors |
| [`signatures`] | ICC 4-byte signature enums (`ColorSpace`, `DeviceClass`, `RenderingIntent`, …) |
| [`error`] | [`Error`] type returned by public API functions |

# ICC Profile Concepts

An ICC profile is a binary file with three sections:

1. **128-byte header** — fixed fields: device class, color space, PCS, version, creation date, etc.
2. **Tag table** — a list of `(signature, offset, size)` entries pointing into the data block.
3. **Tag data** — the actual payload for each tag (matrices, curves, look-up tables, text, …).

## Device Classes

The ICC specification defines eight device classes.  This crate provides a dedicated type for each:

| Type | ICC class code | Typical use |
| --- | --- | --- |
| [`profile::DisplayProfile`] | `mntr` | Monitors, projectors |
| [`profile::InputProfile`] | `scnr` | Cameras, scanners |
| [`profile::OutputProfile`] | `prtr` | Printers |
| [`profile::DeviceLinkProfile`] | `link` | Direct device-to-device transforms |
| [`profile::AbstractProfile`] | `abst` | Abstract color transforms |
| [`profile::ColorSpaceProfile`] | `spac` | Color space definitions |
| [`profile::NamedColorProfile`] | `nmcl` | Named color palettes |
| [`profile::SpectralProfile`] | `cenc` | Spectral data (ICC v5) |

All types wrap a [`profile::RawProfile`] which holds the raw binary data and
preserves unknown tags verbatim, guaranteeing lossless round-trips.

## Profile Connection Space (PCS)

Profiles connect device-specific color values to a common reference color space called the
**Profile Connection Space (PCS)**.  Two PCS values are defined by the ICC specification:

- `XYZ` — CIE 1931 XYZ, used by most display and output profiles.
- `Lab` — CIELAB (L\*a\*b\*), used by some output and abstract profiles.

## Rendering Intents

The rendering intent controls how out-of-gamut colors are handled during color conversion.
Four intents are defined:

| Intent | Typical use |
| --- | --- |
| Perceptual | Photographic images — compresses the gamut smoothly |
| Relative Colorimetric | Graphics — clips and maps the source white point |
| Saturation | Business graphics — maximises saturation |
| Absolute Colorimetric | Proofing — preserves absolute colorimetric values |

## Tags

Tags are identified by a 4-byte signature (e.g. `rXYZ`, `rTRC`, `desc`).  Each tag carries a
payload of a specific ICC type — an XYZ triplet, a tone-reproduction curve, a text string, etc.
All well-known tag signatures are re-exported from [`tag::tags`].

Tag types supported for reading and writing include:

| ICC type | Rust type | Common tags |
| --- | --- | --- |
| `XYZ` | `XYZArrayData` | `rXYZ`, `gXYZ`, `bXYZ`, `wtpt` |
| `para` | `ParametricCurveData` | `rTRC`, `gTRC`, `bTRC` |
| `curv` | `CurveData` | `rTRC`, `gTRC`, `bTRC` |
| `mluc` | `MultiLocalizedUnicodeData` | `desc` |
| `desc` | `TextDescriptionData` | `desc` |
| `sf32` | `S15Fixed16ArrayData` | `chad` |
| `sig` | `SignatureData` | `tech` |
| `text` | `TextData` | `cprt` |
| `mft1` | `Lut8Data` | `A2B0`, `B2A0` |
| `mft2` | `Lut16Data` | `A2B0`, `B2A0` |

Tags not yet parsed are stored as `RawData` and written back verbatim — no data is lost.

# Key Types

| Type | Description |
| --- | --- |
| [`S15Fixed16`] | ICC s15Fixed16 fixed-point number used in matrices and XYZ values |
| [`profile::Profile`] | Parsed profile, dispatched to one of eight device-class variants |
| [`profile::TagSetter`] | Consuming builder returned by `with_tag(…)` |
| [`tag::TagSignature`] | 4-byte tag identifier; 70+ known signatures plus `Unknown(u32)` |
| [`error::Error`] | Top-level error type |

# Lossless Round-trips

Any tag not recognised by this crate is preserved as raw bytes and written back verbatim.
Reading a profile and re-serialising it produces byte-identical output.

# CLI Tool

The `cmx` binary prints any ICC profile as TOML:

```bash
cargo install cmx
cmx profile.icc               # print TOML to stdout
cmx profile.icc -o out.toml   # write TOML to a file
```

# Installation

Add the library to your project:

```bash
cargo add cmx
```

Full API documentation is on [docs.rs/cmx](https://docs.rs/cmx).

# Roadmap

- [x] Parse full ICC profiles (versions 2.x–5.0)
- [x] Lossless round-trips — unknown tags preserved verbatim
- [x] Conversion to human-readable TOML format
- [x] Builder-style API for constructing ICC profiles
- [x] Support for the primary ICC tag types
- [ ] Read TOML color profiles and convert back to binary ICC
- [ ] Support all ICC tag types
- [ ] Spectral data and ICC v5 color management
*/

pub mod error;
pub mod header;
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

/// An ICC `s15Fixed16Number`: a signed 32-bit fixed-point value with 15 integer bits and
/// 16 fractional bits, stored in big-endian byte order.
///
/// The value `v` is encoded as `round(v × 65536)`.  The representable range is roughly
/// `[−32768, 32767.99998]`.  This type is used throughout ICC profiles for XYZ tristimulus
/// values, chromatic-adaptation matrices, and parametric curve parameters.
///
/// Conversion to and from `f64` is provided via the standard [`From`] trait.  Values are
/// rounded to 5 decimal places on conversion to avoid floating-point noise.
#[derive(FromBytes, IntoBytes, Unaligned, KnownLayout, Immutable, Debug, Clone, Copy)]
#[repr(C)]
pub struct S15Fixed16(I32<BigEndian>);

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
