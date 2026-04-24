# CMX — Rust ICC Color Profile Library

## What This Is

CMX is a Rust library for working with ICC color profiles (versions 2.0–5.0). It supports:

- **Parsing** binary ICC profiles from files or byte slices
- **Constructing** profiles from scratch via a builder-style API
- **Modifying** existing profiles programmatically
- **Converting** profiles to human-readable TOML format
- A **CLI tool** (`cmx`) to convert ICC profiles to TOML

It integrates with the [`colorimetry`](https://crates.io/crates/colorimetry) crate for color space
operations and uses `zerocopy` for memory-safe binary data handling.

---

## Build, Test, and Run

```bash
# Build
cargo build
cargo build --release

# Test (all integration tests live in tests/)
cargo test
cargo test --test roundtrip          # round-trip serialization
cargo test --test displayP3          # Display P3 profile construction

# CLI tool
cargo install --path .               # installs `cmx` binary
cmx profile.icc                      # print TOML to stdout
cmx profile.icc -o profile.toml      # write TOML to file

# Examples
cargo run --example primaries
cargo run --example set_relcolorimetric_intent -- path/to/profile.icc

# Docs
cargo doc --open
```

---

## Project Layout

```text
src/
  lib.rs                    # Library root, utility functions
  error.rs                  # Error types
  bin/cmx.rs                # CLI entry point
  header.rs / header/       # ICC header (128-byte structure)
  profile.rs / profile/     # Profile enum + per-device-class types
  signatures.rs / signatures/ # ICC 4-byte signature enums
  tag.rs / tag/             # Tag definitions, parsed tags
  tag/tagdata/              # Individual tag data types
tests/                      # Integration tests + test .icc files
examples/                   # Runnable example programs
cmx-icc/                    # WebAssembly / npm bindings (published as cmx-icc)
xtask/                      # Custom build and publish tasks (cargo xtask)
```

---

## Architecture

### Profile Type System

The `Profile` enum wraps eight device-class-specific structs, all backed by `RawProfile`:

| Struct | Device class |
| --- | --- |
| `DisplayProfile` | Monitors, projectors |
| `InputProfile` | Cameras, scanners |
| `OutputProfile` | Printers |
| `DeviceLinkProfile` | Device-to-device transforms |
| `AbstractProfile` | Abstract color transforms |
| `ColorSpaceProfile` | Color space definitions |
| `NamedColorProfile` | Named color palettes |
| `SpectralProfile` | Spectral data (future) |

Each wrapper enforces ICC spec constraints for that class. `RawProfile` holds the actual binary
data (bytes + `IndexMap` of tag records). `HasRawProfile` is the delegation trait.

### Builder / Tag Setter Pattern

Tags are set via a consuming builder chain:

```rust
let profile = DisplayProfile::new()
    .with_tag(ProfileDescriptionTag)
        .as_text_description(|t| t.set_ascii("My Profile"))
    .with_tag(MediaWhitePointTag)
        .as_xyz_array(|xyz| xyz.set([0.9505, 1.0, 1.0891]))
    .with_profile_id();  // computes MD5 checksum in place
```

`TagSetter<P, S>` is the intermediate builder type. It uses generic marker types and capability
traits to enforce at compile time which tag data types are valid for a given tag signature.

### Tag Data Types

`TagData` is the core enum covering all supported ICC tag payload types:
`CurveData`, `ParametricCurveData`, `XYZArrayData`, `TextData`, `TextDescriptionData`,
`MultiLocalizedUnicodeData`, `S15Fixed16ArrayData`, `ChromaticityData`, `MeasurementData`,
`Lut8Data`, `Lut16Data`, `SignatureData`, `RawData`, and more.

Unknown/vendor tags are preserved via `RawData` — round-trips are lossless.

### Key Types

- `S15Fixed16` — 15.16 fixed-point number (used for matrices and XYZ values)
- `Signature` — generic 4-byte ICC identifier parser
- `TagSignature` — 70+ known ICC tag type identifiers + `Unknown(u32)`
- `ColorSpace`, `DeviceClass`, `Pcs`, `Platform`, `Technology` — signature enums
- `RenderingIntent` — Perceptual, RelativeColorimetric, Saturation, AbsoluteColorimetric
- `Illuminant` — D50, D65, D93, F2, A, E, Daylight, BlackBody, etc.

### Binary Safety

Uses `zerocopy` for zero-copy overlay of the 128-byte ICC header — no unsafe code required.
`IndexMap` preserves tag insertion order from the source profile, which is critical for
deterministic round-trips and offset recalculation.

---

## Dependencies (Notable)

| Crate | Purpose |
| --- | --- |
| `colorimetry = "0.0.9"` | Color space definitions (RgbSpace, etc.) |
| `zerocopy = "0.8"` | Safe binary overlay for ICC header |
| `nalgebra = "0.33"` | Matrix math |
| `chrono = "0.4"` | DateTime in profile headers |
| `indexmap = "2"` | Order-preserving tag map |
| `serde` / `toml` | TOML serialization |
| `md5 = "0.8"` | Profile ID checksum |
| `thiserror = "2"` | Error types |
| `clap = "4"` | CLI arg parsing |
| `strum = "0.25"` | Enum utilities |
| `delegate = "0.13"` | Method delegation to RawProfile |

---

## Key Conventions

- **Lossless round-trips**: Unknown tags are stored as `RawData` and written back verbatim.
  Any test that reads a real ICC file and re-serializes it must produce byte-identical output.
- **Profile ID**: Call `.with_profile_id()` at the end of profile construction to compute and
  embed the MD5 checksum (fields 84–99 of the header, zeroed during computation per the ICC spec).
- **Tag data sharing**: Multiple tag signatures can reference the same offset in the file.
  The library detects and preserves this optimization.
- **Consuming builder**: `with_tag(...)` and `as_*` methods consume `self` and return a new value.
  Do not attempt to reuse intermediate `TagSetter` values.
- **Feature flags**: ICC v5 support is gated behind a `v5` feature flag.

---

## Common Entry Points

```rust
// Read an existing profile
let profile = Profile::read("path/to/profile.icc")?;

// Build a standard display profile
let p3 = DisplayProfile::cmx_display_p3(RenderingIntent::RelativeColorimetric);

// Serialize to bytes (for writing or embedding)
let bytes = profile.into_bytes()?;

// Write to disk
profile.write("output.icc")?;
```

---

## Testing Notes

Integration tests in `tests/` use real `.icc` files stored under `tests/profiles/`. When adding
new tag parsers or modifying serialization, always run the round-trip tests to confirm
byte-identical output. The `displayP3` test compares a programmatically constructed profile
against Apple's shipped Display P3 profile binary.

---

## Commit and PR Policy

### Direct commits to `main`

Permitted only for changes that carry no behavioral risk:

- Typo or grammar fixes in documentation or comments
- `CHANGELOG.md` updates (the release commit itself)
- `Cargo.toml` version bump (the release commit itself)
- CI / GitHub Actions configuration changes with no code impact
- Dependency-only bumps that pass all tests and change no public API

### Pull Requests (required)

Everything else goes through a PR so that CI runs and a second pair of eyes can review before the change lands on `main`:

- Any change to `src/` (new features, bug fixes, refactors, API changes)
- New or modified integration tests in `tests/`
- Changes to examples in `examples/`
- Any change that touches public API surface (adding/removing/renaming types, methods, or error variants)

Branch naming convention: `fix/<short-description>`, `feat/<short-description>`, `docs/<short-description>`.

---

## Release Process

Follow these steps in order.  All commands run from the repository root on a clean `main` branch.

### 1. Decide the new version number

This crate uses [Semantic Versioning](https://semver.org/).  While the version is `0.0.x`
(pre-1.0), any user-visible change — including new public API or bug fixes — warrants a
patch increment.  Use a minor increment (`0.1.0`) when the public API is considered stable
enough to declare a broader interface contract.

| Change type | Version bump |
| --- | --- |
| Bug fixes, internal refactors, docs | patch (`0.0.x → 0.0.x+1`) |
| New public API, new features | minor (`0.x.0 → 0.x+1.0`) |
| Breaking API changes | major (`x.0.0 → x+1.0.0`) |

### 2. Merge all pending PRs

Ensure the `main` branch is up to date and all intended changes are merged.

### 3. Update `CHANGELOG.md`

- Rename the `## [Unreleased]` heading to `## [X.Y.Z] - YYYY-MM-DD` (today's date).
- Add a fresh `## [Unreleased]` section above it (empty, ready for the next cycle).
- Review the entries: make sure every user-visible change since the previous release is listed.
  Compare against `git log vPREV..HEAD --oneline` to catch anything missing.

### 4. Bump the version in `Cargo.toml`

The version is defined once in `[workspace.package]` in the root `Cargo.toml`.
Both `cmx` (crates.io) and `cmx-icc` (npm) inherit it via `version.workspace = true`,
so a single edit keeps everything in lock-step.

```bash
# Edit the `version = "..."` field in [workspace.package]
```

Search the codebase for any hard-coded version strings in documentation and update them too:

```bash
grep -rn "0\.0\.X" --include="*.rs" --include="*.toml" --include="*.md"
```

### 5. Run the full test suite and quality checks

Use the xtask dry-run, which covers tests, doc-tests, clippy, cargo doc, and README in one step:

```bash
cargo xtask publish-crate --dry-run
```

All checks must be green before tagging. Fix any failures before proceeding.

### 6. Regenerate README.md

`cargo xtask publish-crate --dry-run` (step 5) regenerates `README.md` automatically via
[`cargo-rdme`](https://github.com/orium/cargo-rdme) if it is out of date.  Review the diff
before staging:

```bash
git diff README.md
```

If `cargo-rdme` is not installed: `cargo install cargo-rdme`.

### 7. Commit the release

Stage only the version-bump and generated files:

```bash
git add Cargo.toml CHANGELOG.md README.md
git commit -m "chore: release v0.0.X"
```

Do **not** amend earlier commits or squash history — keep the release commit separate and minimal.

### 8. Tag the release

```bash
git tag -a v0.0.X -m "Release v0.0.X"
git push origin main --tags
```

### 9. Publish to crates.io

```bash
cargo xtask publish-crate
```

This re-runs all checks (tests, clippy, doc, README) and then calls `cargo publish -p cmx`.
If anything fails, fix it before re-running — do not push the tag until this step succeeds.

### 10. Publish to npm (cmx-icc)

```bash
cargo xtask publish-npm
```

This does three things in order:

1. Regenerates `cmx-icc/README.md` from the doc comment in `cmx-icc/src/lib.rs`
   via `cargo rdme -w cmx-icc` (the README is the npm package page).
2. Runs `wasm-pack build cmx-icc --target bundler --release`, which compiles the
   `.wasm` binary and copies `cmx-icc/README.md` into `cmx-icc/pkg/`.
3. Runs `npm publish cmx-icc/pkg --access public`.

Requires `wasm-pack` and an active `npm login` session.  The published package
name is `cmx-icc` at the same version as the Rust crate.

To update the npm README, edit the module doc comment (`//!`) at the top of
`cmx-icc/src/lib.rs` — do not edit `cmx-icc/README.md` directly, as it is
generated and will be overwritten.
