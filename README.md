# CMX: Rust Spectral Color Management Library
<!-- cargo-rdme start -->

This crate provides a set of utilities for working with ICC Color Profiles
and the Colorimetry Library.

The main functionality at this stage it to parse ICC profiles, and convert them
to TOML format using the cmx command line tool:

```bash
cmx profile.icc -o profile.toml
 ```

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
- [ ] Create ICC profiles using the [`Colorimetry`] library features

<!-- cargo-rdme end -->

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license
  ([LICENSE-MIT](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
