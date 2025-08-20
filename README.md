# CMX: Rust Spectral Color Management Library
<!-- cargo-rdme start -->

This crate provides utilities for working with ICC color profiles
and integrates with the Colorimetry Library.

### Use Cases
 <details>
<summary><strong>Parsing ICC profiles and conversion to TOML format for analysis</strong></summary>
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
xyz = [0.89459228515625, 1.0, 0.9544219970703125]

[rXYZ]
xyz = [0.4861602783203125, 0.2266845703125, -0.0008087158203125]

[gXYZ]
xyz = [0.3238525390625, 0.7103271484375, 0.0432281494140625]

[bXYZ]
xyz = [0.1541900634765625, 0.06298828125, 0.782470703125]

[chad]
matrix = [
    [1.073822, 0.038803, -0.036896], 
    [0.055573, 0.963989, -0.014343], 
    [-0.004272, 0.005295, 0.862778]
]

[rTRC]
g = 2.6

[bTRC]
g = 2.6

[gTRC]
g = 2.6

 ``` 
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
