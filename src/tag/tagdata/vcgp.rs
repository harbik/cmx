// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2026, Harbers Bik LLC

//! Apple `vcgp` (Video Card Gamma and Primary) tag data decoder.
//!
//! `vcgp` is an Apple-private tag that combines two aspects of calibrated display
//! state into one structure.  It is not part of the ICC specification.
//!
//! ## Relationship to `vcgt`
//!
//! | Tag    | Stands for                        | Contents                          |
//! |--------|-----------------------------------|-----------------------------------|
//! | `vcgt` | Video Card Gamma Table            | Calibration LUT only              |
//! | `vcgp` | Video Card Gamma and Primary      | Calibration curves + chromaticity |
//!
//! ## Binary layout
//!
//! ```text
//! Bytes  0– 3  type signature   'vcgp' (0x76636770)
//! Bytes  4– 7  reserved         0
//!
//! ── Calibration section (per-channel gamma, 24 bytes) ──────────────────────
//! Bytes  8–11  red   gamma_in   S15.16 — reference / source gamma
//! Bytes 12–15  red   gamma_out  S15.16 — target display gamma
//! Bytes 16–19  green gamma_in   S15.16
//! Bytes 20–23  green gamma_out  S15.16
//! Bytes 24–27  blue  gamma_in   S15.16
//! Bytes 28–31  blue  gamma_out  S15.16
//!
//! ── Chromaticity section (per-channel primaries, 24 bytes) ─────────────────
//! Bytes 32–35  red   primary_x  U32 normalized (value / 0xFFFF_FFFF → [0, 1])
//! Bytes 36–39  red   primary_y  U32 normalized
//! Bytes 40–43  green primary_x  U32 normalized
//! Bytes 44–47  green primary_y  U32 normalized
//! Bytes 48–51  blue  primary_x  U32 normalized
//! Bytes 52–55  blue  primary_y  U32 normalized
//! ```
//!
//! ## Field semantics (reverse-engineered)
//!
//! **Calibration section** — same role as `vcgt`.  The `gamma_in` field (observed: 3.0)
//! encodes the reference or native panel gamma; `gamma_out` (observed: 2.4) is the
//! target electro-optical transfer function gamma (matching the sRGB TRC).  Both are
//! stored as S15.16 big-endian fixed-point.
//!
//! **Chromaticity section** — distinguishes `vcgp` from `vcgt`.  Encodes the effective
//! RGB primary chromaticities (x, y) of the display after calibration, using a
//! normalized unsigned 32-bit encoding where `0x00000000` = 0.0 and `0xFFFFFFFF` = 1.0.
//!
//! In the test profiles examined (Color LCD, SyncMaster, LG HDR WQHD) all three
//! channels show the same placeholder values (`x ≈ 0.0`, `y ≈ 0.2`), indicating the
//! chromaticity section is only populated after a full ColorSync calibration.

use serde::Serialize;
use zerocopy::{BigEndian, Immutable, KnownLayout, TryFromBytes, Unaligned, I32, U32};

use crate::{s15fixed16, tag::tagdata::VcgpData};

// ── Binary layout ─────────────────────────────────────────────────────────────

/// Fixed-size layout of the `vcgp` payload (56 bytes total including the 8-byte header).
#[derive(TryFromBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct Layout {
    _type_signature: [u8; 4],
    _reserved: [u8; 4],
    // Calibration section: per-channel gamma (S15.16)
    red_gamma_in: I32<BigEndian>,
    red_gamma_out: I32<BigEndian>,
    green_gamma_in: I32<BigEndian>,
    green_gamma_out: I32<BigEndian>,
    blue_gamma_in: I32<BigEndian>,
    blue_gamma_out: I32<BigEndian>,
    // Chromaticity section: per-channel primaries (U32 normalized)
    red_primary_x: U32<BigEndian>,
    red_primary_y: U32<BigEndian>,
    green_primary_x: U32<BigEndian>,
    green_primary_y: U32<BigEndian>,
    blue_primary_x: U32<BigEndian>,
    blue_primary_y: U32<BigEndian>,
}

// ── Normalization helper ──────────────────────────────────────────────────────

/// Convert a U32 chromaticity value to `f64` in [0, 1], rounded to 6 decimal places.
///
/// The encoding uses the full 32-bit range: `0x00000000` → 0.0, `0xFFFFFFFF` → 1.0.
/// Rounding to 6 dp removes floating-point noise from the `/ 0xFFFF_FFFF` division while
/// preserving more precision than ICC profiles ever report (4 significant figures).
#[inline]
fn norm_u32(v: u32) -> f64 {
    let raw = v as f64 / u32::MAX as f64;
    (raw * 1_000_000.0).round() / 1_000_000.0
}

// ── Parsed / serialisable representation ──────────────────────────────────────

/// Per-channel data decoded from an Apple `vcgp` tag.
#[derive(Serialize)]
pub struct VcgpChannel {
    /// Reference or source gamma (observed: 3.0), stored as S15.16.
    pub gamma_in: f64,
    /// Target display gamma (observed: 2.4, matching the sRGB electro-optical TRC),
    /// stored as S15.16.
    pub gamma_out: f64,
    /// Primary chromaticity x coordinate, normalized U32 encoding.
    /// Populated after ColorSync calibration; placeholder value ≈ 0.0 otherwise.
    pub primary_x: f64,
    /// Primary chromaticity y coordinate, normalized U32 encoding.
    /// Placeholder value ≈ 0.2 in uncalibrated profiles.
    pub primary_y: f64,
}

/// Decoded Apple `vcgp` (Video Card Gamma and Primary) tag.
///
/// Serialises to TOML as three per-channel sections:
///
/// ```toml
/// [vcgp.red]
/// gamma_in  = 3.0
/// gamma_out = 2.4
/// primary_x = 0.0
/// primary_y = 0.2
///
/// [vcgp.green]
/// # … identical in uncalibrated profiles
///
/// [vcgp.blue]
/// # … identical in uncalibrated profiles
/// ```
#[derive(Serialize)]
pub struct VcgpType {
    pub red: VcgpChannel,
    pub green: VcgpChannel,
    pub blue: VcgpChannel,
}

// ── Parser ────────────────────────────────────────────────────────────────────

impl From<&VcgpData> for VcgpType {
    fn from(data: &VcgpData) -> Self {
        let fallback = || VcgpType {
            red:   VcgpChannel { gamma_in: 0.0, gamma_out: 0.0, primary_x: 0.0, primary_y: 0.0 },
            green: VcgpChannel { gamma_in: 0.0, gamma_out: 0.0, primary_x: 0.0, primary_y: 0.0 },
            blue:  VcgpChannel { gamma_in: 0.0, gamma_out: 0.0, primary_x: 0.0, primary_y: 0.0 },
        };

        let Some(layout) = Layout::try_ref_from_bytes(data.0.as_slice()).ok() else {
            return fallback();
        };

        VcgpType {
            red: VcgpChannel {
                gamma_in:  s15fixed16(layout.red_gamma_in.get()),
                gamma_out: s15fixed16(layout.red_gamma_out.get()),
                primary_x: norm_u32(layout.red_primary_x.get()),
                primary_y: norm_u32(layout.red_primary_y.get()),
            },
            green: VcgpChannel {
                gamma_in:  s15fixed16(layout.green_gamma_in.get()),
                gamma_out: s15fixed16(layout.green_gamma_out.get()),
                primary_x: norm_u32(layout.green_primary_x.get()),
                primary_y: norm_u32(layout.green_primary_y.get()),
            },
            blue: VcgpChannel {
                gamma_in:  s15fixed16(layout.blue_gamma_in.get()),
                gamma_out: s15fixed16(layout.blue_gamma_out.get()),
                primary_x: norm_u32(layout.blue_primary_x.get()),
                primary_y: norm_u32(layout.blue_primary_y.get()),
            },
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a vcgp payload with explicit values for all fields.
    fn make_vcgp(
        gamma_in: i32,
        gamma_out: i32,
        primary_x: u32,
        primary_y: u32,
    ) -> VcgpData {
        let mut buf = Vec::with_capacity(56);
        buf.extend_from_slice(b"vcgp");       // type signature
        buf.extend_from_slice(&[0u8; 4]);     // reserved
        // Calibration: gamma_in, gamma_out interleaved per channel
        for _ in 0..3 {
            buf.extend_from_slice(&gamma_in.to_be_bytes());
            buf.extend_from_slice(&gamma_out.to_be_bytes());
        }
        // Chromaticity: x, y × 3 channels
        for _ in 0..3 {
            buf.extend_from_slice(&primary_x.to_be_bytes());
            buf.extend_from_slice(&primary_y.to_be_bytes());
        }
        VcgpData(buf)
    }

    #[test]
    fn parses_real_profile_values() {
        // Exact bytes observed in Color LCD, LG HDR WQHD, SyncMaster profiles.
        let data = make_vcgp(0x00030000, 0x00026666, 0x00000002, 0x33333400);
        let parsed = VcgpType::from(&data);

        assert!((parsed.red.gamma_in  - 3.0).abs() < 1e-4);
        assert!((parsed.red.gamma_out - 2.4).abs() < 1e-3);
        // Chromaticity placeholder values: x ≈ 0, y ≈ 0.2
        assert!(parsed.red.primary_x < 1e-6);
        assert!((parsed.red.primary_y - 0.2).abs() < 1e-4);

        // All channels are identical in uncalibrated profiles.
        assert_eq!(parsed.red.gamma_in,  parsed.green.gamma_in);
        assert_eq!(parsed.red.gamma_out, parsed.green.gamma_out);
        assert_eq!(parsed.red.gamma_in,  parsed.blue.gamma_in);
    }

    #[test]
    fn calibrated_chromaticity_roundtrip() {
        // Simulate a calibrated profile with real sRGB primary chromaticities.
        // sRGB red: x=0.64, y=0.33 — encoded as U32 normalized.
        let x_enc = (0.64_f64 * u32::MAX as f64) as u32;
        let y_enc = (0.33_f64 * u32::MAX as f64) as u32;
        let data = make_vcgp(0x00030000, 0x00026666, x_enc, y_enc);
        let parsed = VcgpType::from(&data);

        assert!((parsed.red.primary_x - 0.64).abs() < 1e-6);
        assert!((parsed.red.primary_y - 0.33).abs() < 1e-6);
    }

    #[test]
    fn zero_payload_returns_zeros() {
        let data = make_vcgp(0, 0, 0, 0);
        let parsed = VcgpType::from(&data);
        assert_eq!(parsed.red.gamma_in,  0.0);
        assert_eq!(parsed.red.gamma_out, 0.0);
        assert_eq!(parsed.red.primary_x, 0.0);
        assert_eq!(parsed.red.primary_y, 0.0);
    }

    #[test]
    fn malformed_too_short_returns_zeros() {
        let data = VcgpData(b"vcgp\x00\x00\x00\x00".to_vec()); // only 8 bytes
        let parsed = VcgpType::from(&data);
        assert_eq!(parsed.red.gamma_in, 0.0);
    }
}
