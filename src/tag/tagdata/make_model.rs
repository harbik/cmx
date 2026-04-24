// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! Apple `mmod` (Make and Model) tag data decoder.
//!
//! `mmod` is an Apple-private tag used in macOS-generated display profiles to record
//! hardware identifiers for the display device.  It is not part of the ICC standard.
//!
//! ## Binary layout
//!
//! ```text
//! Bytes  0– 3  type signature  'mmod' (0x6D6D6F64)
//! Bytes  4– 7  reserved        0
//! Bytes  8–11  manufacturer    u32 big-endian (Apple device code)
//! Bytes 12–15  model           u32 big-endian (device model code)
//! Bytes 16–19  serial_number   u32 big-endian (often ASCII-encoded, e.g. "MY24")
//! Bytes 20–23  manufacture_date u32 big-endian (opaque timestamp)
//! Bytes 24–…   reserved        zeros
//! ```
//!
//! All four identifier fields are currently rendered as `"0xNNNNNNNN"` hex strings.
//! Future work may decode `serial_number` as ASCII when all bytes are printable, and
//! `manufacture_date` as a human-readable date.

use serde::Serialize;
use zerocopy::{BigEndian, Immutable, KnownLayout, TryFromBytes, Unaligned, U32};

use crate::tag::tagdata::MakeAndModelData;

// ── Binary layout ────────────────────────────────────────────────────────────

/// The fixed-size prefix of an `mmod` tag (type signature through manufacture_date).
#[derive(TryFromBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct Layout {
    _type_signature: U32<BigEndian>,
    _reserved: [u8; 4],
    manufacturer: U32<BigEndian>,
    model: U32<BigEndian>,
    serial_number: U32<BigEndian>,
    manufacture_date: U32<BigEndian>,
}

// ── Parsed / serialisable representation ─────────────────────────────────────

/// Decoded Apple `mmod` tag: hardware manufacturer and model identifiers.
///
/// All fields are rendered as `"0xNNNNNNNN"` hex strings because the exact
/// semantics of each code are Apple-internal and not publicly documented.
///
/// Serialises to TOML as:
///
/// ```toml
/// [mmod]
/// manufacturer     = "0x00004c2d"
/// model            = "0x00000587"
/// serial_number    = "0x4d593234"
/// manufacture_date = "0xc76c2600"
/// ```
#[derive(Serialize)]
pub struct MakeAndModelType {
    manufacturer: String,
    model: String,
    serial_number: String,
    manufacture_date: String,
}

// ── Parser ───────────────────────────────────────────────────────────────────

impl From<&MakeAndModelData> for MakeAndModelType {
    fn from(data: &MakeAndModelData) -> Self {
        // We need at least the 24-byte prefix (header + 4 identifier fields).
        let Some(layout) = data.0.get(..24).and_then(|b| Layout::try_ref_from_bytes(b).ok())
        else {
            return MakeAndModelType {
                manufacturer: "0x00000000".to_string(),
                model: "0x00000000".to_string(),
                serial_number: "0x00000000".to_string(),
                manufacture_date: "0x00000000".to_string(),
            };
        };

        MakeAndModelType {
            manufacturer: format!("0x{:08x}", layout.manufacturer.get()),
            model: format!("0x{:08x}", layout.model.get()),
            serial_number: format!("0x{:08x}", layout.serial_number.get()),
            manufacture_date: format!("0x{:08x}", layout.manufacture_date.get()),
        }
    }
}

// ── Builder methods on MakeAndModelData ──────────────────────────────────────

impl MakeAndModelData {
    /// Initialize the binary payload with the `mmod` type signature and zeroed fields.
    fn init(&mut self) {
        if self.0.len() < 40 {
            self.0 = vec![0u8; 40];
            self.0[0..4].copy_from_slice(&0x6D6D6F64u32.to_be_bytes()); // 'mmod'
        }
    }

    /// Set the manufacturer identifier (bytes 8–11).
    pub fn set_manufacturer(&mut self, value: u32) {
        self.init();
        self.0[8..12].copy_from_slice(&value.to_be_bytes());
    }

    /// Set the model identifier (bytes 12–15).
    pub fn set_model(&mut self, value: u32) {
        self.init();
        self.0[12..16].copy_from_slice(&value.to_be_bytes());
    }

    /// Set the serial number field (bytes 16–19).
    ///
    /// On Apple hardware this is often four ASCII characters (e.g. `b"MY24"`),
    /// but the field is opaque and any `u32` value is accepted.
    pub fn set_serial_number(&mut self, value: u32) {
        self.init();
        self.0[16..20].copy_from_slice(&value.to_be_bytes());
    }

    /// Set the manufacture-date field (bytes 20–23).
    pub fn set_manufacture_date(&mut self, value: u32) {
        self.init();
        self.0[20..24].copy_from_slice(&value.to_be_bytes());
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal `mmod` byte slice and round-trip it through the parser.
    fn make_mmod(manufacturer: u32, model: u32, serial: u32, date: u32) -> MakeAndModelData {
        let mut buf = Vec::with_capacity(40);
        buf.extend_from_slice(&0x6D6D6F64u32.to_be_bytes()); // type sig 'mmod'
        buf.extend_from_slice(&0u32.to_be_bytes()); // reserved
        buf.extend_from_slice(&manufacturer.to_be_bytes());
        buf.extend_from_slice(&model.to_be_bytes());
        buf.extend_from_slice(&serial.to_be_bytes());
        buf.extend_from_slice(&date.to_be_bytes());
        buf.extend_from_slice(&[0u8; 16]); // reserved padding
        MakeAndModelData(buf)
    }

    #[test]
    fn parses_known_values() {
        let data = make_mmod(0x00004c2d, 0x00000587, 0x4d593234, 0xc76c2600);
        let parsed = MakeAndModelType::from(&data);
        assert_eq!(parsed.manufacturer, "0x00004c2d");
        assert_eq!(parsed.model, "0x00000587");
        assert_eq!(parsed.serial_number, "0x4d593234");
        assert_eq!(parsed.manufacture_date, "0xc76c2600");
    }

    #[test]
    fn zero_values() {
        let data = make_mmod(0, 0, 0, 0);
        let parsed = MakeAndModelType::from(&data);
        assert_eq!(parsed.manufacturer, "0x00000000");
        assert_eq!(parsed.model, "0x00000000");
    }

    #[test]
    fn malformed_too_short_returns_zeroes() {
        let data = MakeAndModelData(vec![0x6D, 0x6D, 0x6F, 0x64]); // only 4 bytes
        let parsed = MakeAndModelType::from(&data);
        assert_eq!(parsed.manufacturer, "0x00000000");
        assert_eq!(parsed.model, "0x00000000");
    }
}
