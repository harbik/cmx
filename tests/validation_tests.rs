// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! Tests that verify the input-validation fixes applied in the fix/input-validation branch.
//!
//! Each test targets a specific panic or silent-error that was present before the fixes and
//! confirms the new, safe behaviour (return an Err, skip a bad record, etc.).

use cmx::{
    error::Error,
    profile::RawProfile,
    tag::tagdata::{
        curve::CurveType, lut8::Lut8Type, CurveData, Lut8Data, MultiLocalizedUnicodeData,
        ParametricCurveData,
    },
};

// ---------------------------------------------------------------------------
// Helper: build a minimal but structurally valid 128-byte ICC header.
//
// The fields required by `from_bytes` to not error before reaching the code
// under test are: profile_size (bytes 0-3) and a plausible date (bytes 24-35).
// Everything else is zero.
// ---------------------------------------------------------------------------
fn minimal_header(profile_size: u32, month: u16, day: u16, hour: u16) -> [u8; 128] {
    let mut h = [0u8; 128];
    // profile size
    h[0..4].copy_from_slice(&profile_size.to_be_bytes());
    // version 4.3 = 0x04300000
    h[8] = 0x04;
    h[9] = 0x30;
    // device class 'mntr' = 0x6D6E7472
    h[12..16].copy_from_slice(b"mntr");
    // color space 'RGB ' = 0x52474220
    h[16..20].copy_from_slice(b"RGB ");
    // PCS 'XYZ ' = 0x58595A20
    h[20..24].copy_from_slice(b"XYZ ");
    // creation date: year 2024, and whatever the caller passes
    h[24..26].copy_from_slice(&2024u16.to_be_bytes()); // year
    h[26..28].copy_from_slice(&month.to_be_bytes());
    h[28..30].copy_from_slice(&day.to_be_bytes());
    h[30..32].copy_from_slice(&hour.to_be_bytes());
    // minutes and seconds = 0
    // file signature 'acsp' = 0x61637370 — needed for check_file_signature(),
    // not for from_bytes(), but set it for completeness.
    h[36..40].copy_from_slice(b"acsp");
    h
}

// ---------------------------------------------------------------------------
// 1. ParametricCurveData::set_parameters_slice — invalid count
// ---------------------------------------------------------------------------

#[test]
fn parametric_curve_invalid_count_returns_error() {
    let mut para = ParametricCurveData::default();

    // 2 and 6 are not ICC-defined parameter counts
    assert!(matches!(
        para.set_parameters_slice(&[1.0, 2.0]),
        Err(Error::UnsupportedParameterCount(2))
    ));
    assert!(matches!(
        para.set_parameters_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
        Err(Error::UnsupportedParameterCount(6))
    ));
}

#[test]
fn parametric_curve_valid_counts_succeed() {
    let mut para = ParametricCurveData::default();
    // All five ICC-defined counts must succeed
    assert!(para.set_parameters_slice(&[2.2]).is_ok()); // 1 — simple gamma
    assert!(para.set_parameters_slice(&[2.2, 0.9, 0.1]).is_ok()); // 3
    assert!(para.set_parameters_slice(&[2.2, 0.9, 0.1, 0.03]).is_ok()); // 4
    assert!(para.set_parameters_slice(&[2.4, 0.948, 0.052, 0.077, 0.040]).is_ok()); // 5 (sRGB)
    assert!(
        para.set_parameters_slice(&[2.4, 0.948, 0.052, 0.077, 0.040, 0.0, 0.0]).is_ok()
    ); // 7
}

// ---------------------------------------------------------------------------
// 2. RawProfile::creation_date() — invalid date fields in the header
// ---------------------------------------------------------------------------

#[test]
fn creation_date_invalid_month_returns_error() {
    // Build the smallest parseable profile (0 tags) with month = 13.
    let total = 128 + 4; // header + tag count
    let mut bytes = vec![0u8; total];
    let header = minimal_header(total as u32, 13, 1, 0); // month = 13
    bytes[..128].copy_from_slice(&header);
    // tag count = 0
    bytes[128..132].copy_from_slice(&0u32.to_be_bytes());

    let profile = RawProfile::from_bytes(&bytes).expect("profile should parse");
    assert!(
        matches!(profile.creation_date(), Err(Error::InvalidDate(_))),
        "month=13 should yield InvalidDate"
    );
}

#[test]
fn creation_date_invalid_day_returns_error() {
    let total = 128 + 4;
    let mut bytes = vec![0u8; total];
    let header = minimal_header(total as u32, 1, 32, 0); // day = 32
    bytes[..128].copy_from_slice(&header);
    bytes[128..132].copy_from_slice(&0u32.to_be_bytes());

    let profile = RawProfile::from_bytes(&bytes).expect("profile should parse");
    assert!(
        matches!(profile.creation_date(), Err(Error::InvalidDate(_))),
        "day=32 should yield InvalidDate"
    );
}

#[test]
fn creation_date_invalid_hour_returns_error() {
    let total = 128 + 4;
    let mut bytes = vec![0u8; total];
    let header = minimal_header(total as u32, 1, 1, 25); // hour = 25
    bytes[..128].copy_from_slice(&header);
    bytes[128..132].copy_from_slice(&0u32.to_be_bytes());

    let profile = RawProfile::from_bytes(&bytes).expect("profile should parse");
    assert!(
        matches!(profile.creation_date(), Err(Error::InvalidDate(_))),
        "hour=25 should yield InvalidDate"
    );
}

#[test]
fn creation_date_valid_date_succeeds() {
    let total = 128 + 4;
    let mut bytes = vec![0u8; total];
    let header = minimal_header(total as u32, 6, 15, 12);
    bytes[..128].copy_from_slice(&header);
    bytes[128..132].copy_from_slice(&0u32.to_be_bytes());

    let profile = RawProfile::from_bytes(&bytes).expect("profile should parse");
    let date = profile.creation_date().expect("valid date should not error");
    use chrono::Datelike;
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), 6);
    assert_eq!(date.day(), 15);
}

// ---------------------------------------------------------------------------
// 3. share_tags — two tag table entries at the same offset but different sizes
// ---------------------------------------------------------------------------

/// Build raw ICC bytes with the given tag table entries.
/// `tags`: list of (sig_bytes, offset, size).  The data region is filled with zeros.
fn build_profile_with_tags(tags: &[([u8; 4], u32, u32)]) -> Vec<u8> {
    // Tag table starts at 132; first tag data can start right after.
    let table_end = 132 + tags.len() * 12;
    // Data region: enough to cover the largest claimed offset+size.
    let data_end = tags
        .iter()
        .map(|(_, off, sz)| *off as usize + *sz as usize)
        .max()
        .unwrap_or(table_end)
        .max(table_end);

    let total = data_end;
    let mut buf = vec![0u8; total];

    // Header
    let header = minimal_header(total as u32, 1, 1, 0);
    buf[..128].copy_from_slice(&header);

    // Tag count
    buf[128..132].copy_from_slice(&(tags.len() as u32).to_be_bytes());

    // Tag table entries
    for (i, (sig, offset, size)) in tags.iter().enumerate() {
        let base = 132 + i * 12;
        buf[base..base + 4].copy_from_slice(sig);
        buf[base + 4..base + 8].copy_from_slice(&offset.to_be_bytes());
        buf[base + 8..base + 12].copy_from_slice(&size.to_be_bytes());
    }

    buf
}

#[test]
fn share_tags_same_offset_different_size_is_rejected() {
    // Two tags at the same offset (156) but different sizes (8 and 16).
    let offset = 156u32;
    let buf = build_profile_with_tags(&[
        (*b"rXYZ", offset, 8),
        (*b"gXYZ", offset, 16), // ← mismatch: same offset, different size
    ]);
    let result = RawProfile::from_bytes(&buf);
    assert!(result.is_err(), "mismatched sizes at same offset should be rejected");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("corrupt") || msg.contains("offset") || msg.contains("size"),
        "error message should mention the corruption: {msg}"
    );
}

#[test]
fn share_tags_same_offset_same_size_is_accepted() {
    // Legitimate shared tag: two entries at identical offset AND size.
    let offset = 156u32;
    let buf = build_profile_with_tags(&[
        (*b"rXYZ", offset, 8),
        (*b"gXYZ", offset, 8), // ← same offset AND same size → valid sharing
    ]);
    assert!(
        RawProfile::from_bytes(&buf).is_ok(),
        "identical offset+size should be accepted as shared tag data"
    );
}

// ---------------------------------------------------------------------------
// 4. MultiLocalizedUnicodeData — out-of-bounds string offset in a record
// ---------------------------------------------------------------------------

/// Build raw MLUC bytes.
/// `records`: list of (language, country, length, offset) all in host order,
///            will be stored big-endian as per ICC spec.
/// `string_bytes`: appended verbatim after the record table.
fn build_mluc(records: &[([u8; 2], [u8; 2], u32, u32)], string_bytes: &[u8]) -> Vec<u8> {
    let n = records.len() as u32;
    let mut buf = Vec::new();
    // Header: 'mluc'(4) + reserved(4) + num_records(4) + record_size=12(4)
    buf.extend_from_slice(b"mluc");
    buf.extend_from_slice(&0u32.to_be_bytes());
    buf.extend_from_slice(&n.to_be_bytes());
    buf.extend_from_slice(&12u32.to_be_bytes()); // record size
    // Records: lang(2) + country(2) + length(4) + offset(4)
    for (lang, ctry, length, offset) in records {
        buf.extend_from_slice(lang);
        buf.extend_from_slice(ctry);
        buf.extend_from_slice(&length.to_be_bytes());
        buf.extend_from_slice(&offset.to_be_bytes());
    }
    buf.extend_from_slice(string_bytes);
    buf
}

#[test]
fn mluc_oob_offset_skips_record_gracefully() {
    // Record claims offset=10000 in a ~28-byte payload — should be skipped, not panic.
    let data = build_mluc(
        &[(*b"en", [0, 0], 4, 10_000)], // offset way past end
        &[],
    );
    let mluc = MultiLocalizedUnicodeData(data);
    // Trigger the From conversion (used internally during TOML serialisation).
    // The bad record should be silently skipped; the result is an empty MLUC.
    let mluc_type = cmx::tag::tagdata::multi_localized_unicode::MultiLocalizedUnicodeType::from(&mluc);
    assert!(mluc_type.is_empty(), "bad record should be skipped, leaving an empty MLUC");
}

// ---------------------------------------------------------------------------
// 5. MultiLocalizedUnicodeData — invalid UTF-16 sequence (lone surrogate)
// ---------------------------------------------------------------------------

#[test]
fn mluc_invalid_utf16_uses_lossy_fallback() {
    // String data: 0xD800 (lone high surrogate) followed by 0x000A (newline) — invalid UTF-16.
    // Record claims offset = 28 (right after the 16-byte header + 12-byte record).
    let string_bytes: &[u8] = &[0xD8, 0x00, 0x00, 0x0A];
    let data = build_mluc(
        &[(*b"en", [0, 0], 4, 28)],
        string_bytes,
    );
    let mluc = MultiLocalizedUnicodeData(data);
    // Should not panic; the lossy fallback replaces the invalid code unit.
    let mluc_type = cmx::tag::tagdata::multi_localized_unicode::MultiLocalizedUnicodeType::from(&mluc);
    // One entry should be present (lossy decoding, not skipped).
    assert!(
        !mluc_type.is_empty(),
        "invalid UTF-16 should produce a lossy entry, not be dropped"
    );
}

// ---------------------------------------------------------------------------
// 6. Lut8Data — CLUT size overflow (n=9, g=255 overflows usize)
// ---------------------------------------------------------------------------

/// Build the 48-byte Lut8HeaderLayout with the given channel/grid settings.
/// The e_mat is all zeros (identity-ish: doesn't matter for size calculation tests).
fn lut8_header_bytes(n: u8, m: u8, g: u8) -> Vec<u8> {
    let mut h = vec![0u8; 48];
    h[0..4].copy_from_slice(b"mft1"); // signature
    // _reserved bytes 4-7 = 0
    h[8] = n;
    h[9] = m;
    h[10] = g;
    // _padding = 0, e_mat = 0
    h
}

#[test]
#[should_panic(expected = "CLUT size overflow")]
fn lut8_clut_overflow_panics_with_message() {
    // 255^9 overflows usize on any 64-bit platform (> u64::MAX).
    // checked_pow returns None → expect fires.
    let data = Lut8Data(lut8_header_bytes(9, 3, 255));
    let _ = Lut8Type::from(&data);
}

#[test]
#[should_panic(expected = "truncated data")]
fn lut8_truncated_data_panics_with_message() {
    // n=2, g=4, m=3 → total_size = 48 + 2*256 + 4^2*3 + 3*256 = 1376.
    // Providing only the 48-byte header triggers the assert.
    let data = Lut8Data(lut8_header_bytes(2, 3, 4));
    let _ = Lut8Type::from(&data);
}

// ---------------------------------------------------------------------------
// 7. CurveData — odd-length payload triggers debug_assert
// ---------------------------------------------------------------------------

/// Build a minimal CurveData with the given number of u16 data points
/// plus an optional trailing odd byte to break alignment.
fn curve_data_bytes(points: u32, extra_byte: bool) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"curv");    // signature
    buf.extend_from_slice(&0u32.to_be_bytes()); // reserved
    buf.extend_from_slice(&points.to_be_bytes()); // count
    for i in 0..points {
        buf.extend_from_slice(&(i as u16).to_be_bytes());
    }
    if extra_byte {
        buf.push(0xFF); // misaligned trailing byte
    }
    buf
}

#[test]
fn curve_even_length_payload_succeeds() {
    // Even payload: no debug_assert should fire, conversion should work.
    let data = CurveData(curve_data_bytes(4, false));
    let ct = CurveType::from(&data);
    // 4 points → produces a `points` variant, not a `gamma` variant.
    let serialised = toml::to_string(&ct).expect("serialisation should succeed");
    assert!(serialised.contains("points"));
}

// The debug_assert only fires in debug builds (i.e. during `cargo test`).
#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "not a multiple of 2")]
fn curve_odd_length_payload_fires_debug_assert() {
    let data = CurveData(curve_data_bytes(4, true)); // 12 + 8 + 1 = 21 bytes — odd payload
    let _ = CurveType::from(&data);
}
