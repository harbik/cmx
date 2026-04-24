# CMX: Rust Spectral Color Management Library
<!-- cargo-rdme start -->

`cmx` is a Rust library for reading, writing, and constructing [ICC color profiles][icc-spec]
(versions 2.0–5.0).  ICC profiles describe how color values produced by a device (camera,
display, printer) relate to a standard reference color space, making them essential for
accurate color reproduction across devices and applications.

[icc-spec]: https://www.color.org/specification/ICC.1-2022-05.pdf

## Quick Start

### Read a profile from disk

```rust
use cmx::profile::Profile;

let profile = Profile::read("profile.icc")?;
println!("Color space : {:?}", profile.data_color_space());
println!("PCS         : {:?}", profile.pcs());
println!("Version     : {:?}", profile.version()?);
```

### Dump a profile as TOML

The [`fmt::Display`](std::fmt::Display) implementation on every profile type serialises it
to TOML — the same output produced by the `cmx` CLI tool:

```rust
use cmx::profile::Profile;

let profile = Profile::read("profile.icc")?;
println!("{profile}");
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

### Build a profile from scratch

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

### Modify an existing profile

Read a profile, change a tag, and write it back:

```rust
use cmx::profile::Profile;
use cmx::tag::tags::CopyrightTag;

Profile::read("input.icc")?
    .with_tag(CopyrightTag)
        .as_text(|t| t.set_text("Copyright 2025 Acme Corp."))
    .write("output.icc")?;
```

## Modules

| Module | Contents |
|---|---|
| [`profile`] | `Profile` enum and per-device-class types (`DisplayProfile`, `InputProfile`, …) |
| [`tag`] | Tag signatures, tag data types, and the `TagSetter` builder |
| [`header`] | ICC 128-byte header fields and accessors |
| [`signatures`] | ICC 4-byte signature enums (`ColorSpace`, `DeviceClass`, `RenderingIntent`, …) |
| [`error`] | [`Error`] type returned by public API functions |

## ICC Profile Concepts

An ICC profile is a binary file with three sections:

1. **128-byte header** — fixed fields: device class, color space, PCS, version, creation date, etc.
2. **Tag table** — a list of `(signature, offset, size)` entries pointing into the data block.
3. **Tag data** — the actual payload for each tag (matrices, curves, look-up tables, text, …).

### Device Classes

The ICC specification defines eight device classes.  This crate provides a dedicated type for each:

| Type | ICC class code | Typical use |
|---|---|---|
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

### Profile Connection Space (PCS)

Profiles connect device-specific color values to a common reference color space called the
**Profile Connection Space (PCS)**.  Two PCS values are defined by the ICC specification:

- `XYZ` — CIE 1931 XYZ, used by most display and output profiles.
- `Lab` — CIELAB (L\*a\*b\*), used by some output and abstract profiles.

### Rendering Intents

The rendering intent controls how out-of-gamut colors are handled during color conversion.
Four intents are defined:

| Intent | Typical use |
|---|---|
| Perceptual | Photographic images — compresses the gamut smoothly |
| Relative Colorimetric | Graphics — clips and maps the source white point |
| Saturation | Business graphics — maximises saturation |
| Absolute Colorimetric | Proofing — preserves absolute colorimetric values |

### Tags

Tags are identified by a 4-byte signature (e.g. `rXYZ`, `rTRC`, `desc`).  Each tag carries a
payload of a specific ICC type — an XYZ triplet, a tone-reproduction curve, a text string, etc.
All well-known tag signatures are re-exported from [`tag::tags`].

Tag types supported for reading and writing include:

| ICC type | Rust type | Common tags |
|---|---|---|
| `XYZ` | `XYZArrayData` | `rXYZ`, `gXYZ`, `bXYZ`, `wtpt` |
| `para` | `ParametricCurveData` | `rTRC`, `gTRC`, `bTRC` |
| `curv` | `CurveData` | `rTRC`, `gTRC`, `bTRC` |
| `mluc` | `MultiLocalizedUnicodeData` | `desc` |
| `desc` | `TextDescriptionData` | `desc` |
| `sf32` | `S15Fixed16ArrayData` | `chad` |
| `sig ` | `SignatureData` | `tech` |
| `text` | `TextData` | `cprt` |
| `mft1` | `Lut8Data` | `A2B0`, `B2A0` |
| `mft2` | `Lut16Data` | `A2B0`, `B2A0` |

Tags not yet parsed are stored as `RawData` and written back verbatim — no data is lost.

## Key Types

| Type | Description |
|---|---|
| [`S15Fixed16`] | ICC s15Fixed16 fixed-point number used in matrices and XYZ values |
| [`profile::Profile`] | Parsed profile, dispatched to one of eight device-class variants |
| [`profile::TagSetter`] | Consuming builder returned by `with_tag(…)` |
| [`tag::TagSignature`] | 4-byte tag identifier; 70+ known signatures plus `Unknown(u32)` |
| [`error::Error`] | Top-level error type |

## Lossless Round-trips

Any tag not recognised by this crate is preserved as raw bytes and written back verbatim.
Reading a profile and re-serialising it produces byte-identical output.

## CLI Tool

The `cmx` binary prints any ICC profile as TOML:

```bash
cargo install cmx
cmx profile.icc               # print TOML to stdout
cmx profile.icc -o out.toml   # write TOML to a file
```

## Installation

Add the library to your project:

```bash
cargo add cmx
```

Full API documentation is on [docs.rs/cmx](https://docs.rs/cmx).

## Roadmap

- [x] Parse full ICC profiles (versions 2.x–5.0)
- [x] Lossless round-trips — unknown tags preserved verbatim
- [x] Conversion to human-readable TOML format
- [x] Builder-style API for constructing ICC profiles
- [x] Support for the primary ICC tag types
- [ ] Read TOML color profiles and convert back to binary ICC
- [ ] Support all ICC tag types
- [ ] Spectral data and ICC v5 color management

<!-- cargo-rdme end -->

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
