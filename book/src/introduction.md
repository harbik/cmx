# Spectral Color Management

*The project is in very early stage, and actively developed. 
See [Updates](#updates) for current status.*

CMX is a set of command line tools and Rust library for spectral color management:
instead of using three values to represent colors, it uses spectral distribution –whenever possible– 
in digital color management workflows.
It can create, read, and process color profiles, as defined by the [International Color Consortium](color.org):
in particular, it supports most of the iccMAX profile tags, relevant for spectral color management.

Spectral color management has very limited support in existing color management modules.
It is backward compatible with older ICC profile version, and there are tools to upgrade, or downgrade, 
your spectral color profiles from older version, whenever possible.
Upgrading a profile requires supplementary information; downgrading allows to create a digital resource which can be processed by color management modules which only support V4 profiles, at the end of a color production workflow.





## Updates

The CMX library follows a quarterly release schedule.

### V0.0.1beta (2022Q1)

- Current CMX pre-release version is V0.0.1 (still in development):
it is expected to be released by the end of this quarter: 2022 Q1.
- Main features in this release
    - Read and write (binary) ICC files, with extension ".icc" into generic Rust `Profile` types:
    this type collects the header and tag information for V2, V4 and V5 (iccMAX) profiles.
    - Read and ICC-profiles in JSON format, using [Serde](https://serde.rs).
    - Command line `cmx` tool functions:
        - `get` extracts color profile from a digital image source, and produces a profile file
        - `set` embeds a color profile in a digital image
        - `check` validates a binary or JSON icc profile, and reports any issues.





