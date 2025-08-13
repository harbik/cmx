# CMX: Rust Spectral Color Management Library
<!-- cargo-rdme start -->

This crate provides a set of utilities for working with ICC Color Profiles
and the Colorimetry Library.

The main functionality at this stage it to parse ICC profiles, and convert them
to TOML format using the cmx command line tool:

```bash
cmx profile.icc -o profile.toml
 ```
Every ICC profile tag is converted to a key in the TOML file, with the tag's
values serialized to key-value pairs.
The values are all given as single line output, so that the TOML file is
human-readable and easy to inspect.

Install the `cmx` tool using Cargo:

```bash
cargo install cmx
```

To use the `cmx` library, run the following command in your Rust project:

```bash
cargo add cmx
```

Its documentation is available at [docs.rs/cmx](https://docs.rs/cmx).

## Roadmap

- [X] Implement a full ICC profile parser
- [X] Convert to TOML file format
- [ ] Parse TOML files back to ICC profiles
- [ ] Create ICC profiles using the [`Colorimetry`](https://docs.rs/colorimetry/latest/colorimetry/) library features

The intention is to fully support advanced ICC Color management,
with the ability to use spectral data, and advanced color models,
while maintaining compatibility with existing ICC profiles.

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
