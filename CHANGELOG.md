# Changelog

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).

## Categories each change fall into

* **Added**: for new features.
* **Changed**: for changes in existing functionality.
* **Deprecated**: for soon-to-be removed features.
* **Removed**: for now removed features.
* **Fixed**: for any bug fixes.
* **Security**: in case of vulnerabilities.

## [Unreleased]

### Added

* **`dictType` parser and builder** — ICC `meta` tag (`dictType`, §10.12) now
  has a full parser (`DictType` struct, `IndexMap`-backed key/value pairs encoded
  as UTF-16BE), a serializer (`DictType → DictData`), and builder methods
  `DictData::insert`, `DictData::remove`, and `DictData::clear`.  Profiles
  containing a `meta` tag now serialize to readable TOML (flat key/value map)
  instead of raw hex.  The builder API is available via
  `TagSetter::as_dict(|d| { … })` on `MetadataTag`.
* **`mmod` decoder** — Apple `mmod` (Make and Model) tag now decodes to structured
  TOML fields (`manufacturer`, `model`, `serial_number`, `manufacture_date`) rendered
  as `"0xNNNNNNNN"` hex strings, instead of a raw hex dump.

## [0.1.0] - 2026-04-24

### Fixed

* **LUT8 parser** — replaced silent integer overflow in CLUT size calculation
  (`g.pow(n) * m`) with `checked_pow` / `checked_mul`; added an `assert!` bounds
  check before all slice indexing to prevent out-of-bounds panics on malformed
  tag data.
* **LUT8 / LUT16 parsers** — replaced `.unwrap()` on the `zerocopy` header
  overlay with `.expect(…)` carrying a descriptive message.
* **`RawProfile::creation_date()`** — changed return type from
  `DateTime<Utc>` to `Result<DateTime<Utc>, Error>`; invalid header date fields
  (e.g. month 13, hour 25) now return `Error::InvalidDate` instead of panicking.
  All callers updated; `parsed_header` renders `<invalid: …>` for display.
* **`RawProfile::with_now_as_creation_date()`** — replaced `.unwrap()` on
  `with_nanosecond(0)` with `.expect(…)` and a comment explaining the invariant.
* **Curve tag parser** — added a `debug_assert` to detect odd-length payloads
  (which `chunks_exact(2)` would silently drop); clarified the safety invariant
  for the `try_into().unwrap()` that follows.
* **`ParametricCurveData::set_parameters_slice()`** — changed return type to
  `Result<(), Error>`; invalid ICC parameter counts (anything other than 1, 3, 4,
  5, or 7) now return `Error::UnsupportedParameterCount` instead of panicking.
  `WriteLayoutHeader::new` updated accordingly; three internal call sites in
  `DisplayProfile` updated to use `.expect(…)`.
* **`RawProfile::into_bytes()`** — replaced silent `buf.len() as u32` cast with
  `u32::try_from(buf.len())`, returning `Error::ProfileTooLarge` if the profile
  exceeds the ICC-specified 4 GiB limit.
* **MultiLocalizedUnicode parser** — added bounds checks on the records-table
  slice and on each record's offset + length before indexing; replaced `.unwrap()`
  on `String::from_utf16` with a lossy fallback; added a `debug_assert` to flag
  non-even UTF-16 byte counts.
* **`RawProfile::from_bytes()` tag-sharing detection** — `share_tags` now also
  validates that duplicate tag-table offsets carry identical sizes; two tags at the
  same offset but with different sizes are treated as a corrupt profile and an
  `InvalidData` error is returned.

### Added

* `Error::InvalidDate(String)` — new error variant for out-of-range date/time
  fields in ICC profile headers.
* `Error::UnsupportedParameterCount(usize)` — new error variant for parametric
  curve parameter counts that are not defined by the ICC specification.
* `Error::ProfileTooLarge(usize)` — new error variant when a serialised profile
  exceeds the ICC 4 GiB size limit.

## [0.0.6] - 2026-04-20

### Changed

* Bumped `colorimetry` dependency from `0.0.8` to `0.0.9`; adapted call sites to
  the renamed `XYZ::values()` → `XYZ::to_array()` method.

## [0.0.5] - 2025-09-17

### Added

* `mluc` MultiLocalizedUnicode type builder pattern, allowing to set localized
  strings for a large collection of tags.
* `as_raw` tag setter in builder pattern, allowing setting low level tag data
  as byte array or hex-string.
* `DisplayProfile` constructors `cmx_srgb`, `cmx_adobe_rgb`, and `cmx_display_p3`;
  lightweight bare bone profiles with CC0 1.0 Public Domain License, for use for
  embedding into images.
* GitHub Build and Test Actions, to qualify contributed code.
* `examples/primaries.rs`, creates RGB test patches with different embedded profiles
* `DisplayProfile::from_rgb_space`, to create a display profile associated with a `Colorimetry` color space

### Removed

* `InputProfile::from_rgb_space`; Input profiles are used for scanners and cameras, and are typically not embedded
  in images to be processed by Web Browsers and Color Managed Applications.

## [0.0.4] - 2025-09-06

### Fixed

* Colorimetry dependency set to v0.0.8, needed to expose RgbSpace gamma curve values

## [0.0.3] - 2025-09-03

### Added

* InputProfile::from_rgb_space, using Colorimetry RgbSpace
* Example setting intent to relative intent for a given ICC profile

## [0.0.2] - 2025-08-28

### Added

* Profile creation via Builder API
* Roundtrip testing for reading, parsing (low-level), and writing profiles
* Support for common tag types
* Automatic update of Profile ID on write
* Optional tag data sharing to reduce profile size when tags share the same data

### Fixed

* Removed tracking of unnecessary local folders from the repository

## [0.0.1] - 2025-08-12

A first release, implementing the basic framework for reading and writing ICC profiles, and changing
ICC profile information for the ICC header, and a selection of tag types.

### Added

* `cmx` command line tool, which reads a binary ICC profile, and prints it to
  `stdout` or to a file, in a `TOML` format
* `IccHeader`, with methods to read and change all the header information using a `zerocopy` layout
  struct
* `IndexMap` as container of all the `Tag` elementa, using a `TagSignature` as key
* for most frequently used Tag-types, functions to read and write high level Tag information
* `xtask` sub-package, for library maintenance, and other future utility functions
