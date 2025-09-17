# Changelog

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).

## Categories each change fall into

* **Added**: for new features.
* **Changed**: for changes in existing functionality.
* **Deprecated**: for soon-to-be removed features.
* **Removed**: for now removed features.
* **Fixed**: for any bug fixes.
* **Security**: in case of vulnerabilities.

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
