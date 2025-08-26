# CMX: Rust Spectral Color Management Library
<!-- cargo-rdme start -->

This crate provides utilities for working with ICC color profiles
and integrates with the Colorimetry Library.

### Use Cases
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
<details><summary><strong>Create ICC profiles programmatically</strong></summary>
You can also use the `cmx` library to create ICC profiles programmatically in Rust.
The library provides a builder-style API for constructing profiles,
allowing you to set various tags and properties.

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
Not all ICC tag types are supported yet, but please submit a pull request, or an issue, on our [GitHub CMX repo](https://github.com/harbik/cmx) if you need additional tags to be supported.

</details>



### Installation

Install the `cmx` tool using Cargo:

```bash
cargo install cmx
```

To use the `cmx` library in your Rust project:

```bash
cargo add cmx
```

Documentation is available at [docs.rs/cmx](https://docs.rs/cmx).

### Roadmap

- [x] Parse full ICC profiles
- [x] Convert to TOML format
- [ ] Support more ICC tags and color models
- [ ] Add builder-style API for constructing ICC profiles
- [ ] Enable spectral data and advanced color management
- [ ] Provide utilities for profile conversion and manipulation

### Overview

Although the ICC specification is broad and complex, this crate aims
to provide a robust foundation for working with ICC profiles in Rust.

It supports parsing, constructing, and changing of the primary ICC-defined tags,
as well as some commonly used non-standard tags.

Even tags that cannot yet be parsed are still preserved when reading
and serializing profiles, ensuring no data loss.

The long-term goal is to fully support advanced ICC color management,
including spectral data and extended color models, while maintaining
compatibility with existing profiles.

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
