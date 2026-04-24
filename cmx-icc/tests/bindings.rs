// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use wasm_bindgen_test::*;

// Run tests in Node.js (no browser driver needed).
wasm_bindgen_test_configure!(run_in_node_experimental);

use cmx_icc::{WasmDisplayProfile, WasmProfile, WasmRenderingIntent};

// ---------------------------------------------------------------------------
// WasmDisplayProfile — preset constructors
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn preset_srgb_serializes() {
    let p = WasmDisplayProfile::srgb(WasmRenderingIntent::RelativeColorimetric);
    let bytes = p.to_bytes().expect("sRGB preset should serialize");
    // ICC profiles always start with the 4-byte profile size followed by 'acsp'
    assert!(
        bytes.len() > 128,
        "serialized profile must be larger than the 128-byte header"
    );
    assert_eq!(
        &bytes[36..40],
        b"acsp",
        "ICC file signature must be 'acsp' at offset 36"
    );
}

#[wasm_bindgen_test]
fn preset_display_p3_serializes() {
    let p = WasmDisplayProfile::display_p3(WasmRenderingIntent::Perceptual);
    let bytes = p.to_bytes().expect("Display P3 preset should serialize");
    assert!(bytes.len() > 128);
    assert_eq!(
        &bytes[36..40],
        b"acsp",
        "ICC file signature must be 'acsp' at offset 36"
    );
}

#[wasm_bindgen_test]
fn preset_adobe_rgb_serializes() {
    let p = WasmDisplayProfile::adobe_rgb(WasmRenderingIntent::Saturation);
    let bytes = p.to_bytes().expect("Adobe RGB preset should serialize");
    assert!(bytes.len() > 128);
    assert_eq!(
        &bytes[36..40],
        b"acsp",
        "ICC file signature must be 'acsp' at offset 36"
    );
}

// ---------------------------------------------------------------------------
// WasmProfile — round-trip parsing
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn round_trip_srgb() {
    // Build → serialize
    let original_bytes = WasmDisplayProfile::srgb(WasmRenderingIntent::RelativeColorimetric)
        .to_bytes()
        .expect("serialize");

    // Parse → re-serialize
    let parsed = WasmProfile::from_bytes(&original_bytes).expect("parse");
    let round_tripped = parsed.to_bytes().expect("re-serialize");

    assert_eq!(
        original_bytes, round_tripped,
        "round-trip must be byte-identical"
    );
}

#[wasm_bindgen_test]
fn round_trip_display_p3() {
    let original_bytes = WasmDisplayProfile::display_p3(WasmRenderingIntent::RelativeColorimetric)
        .to_bytes()
        .expect("serialize");
    let parsed = WasmProfile::from_bytes(&original_bytes).expect("parse");
    let round_tripped = parsed.to_bytes().expect("re-serialize");
    assert_eq!(original_bytes, round_tripped);
}

#[wasm_bindgen_test]
fn from_bytes_invalid_data_errors() {
    let result = WasmProfile::from_bytes(&[0u8; 32]);
    assert!(result.is_err(), "32 garbage bytes should fail to parse");
}

// ---------------------------------------------------------------------------
// WasmProfile — rendering intent accessor
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn rendering_intent_relative_colorimetric() {
    let bytes = WasmDisplayProfile::srgb(WasmRenderingIntent::RelativeColorimetric)
        .to_bytes()
        .expect("serialize");
    let p = WasmProfile::from_bytes(&bytes).expect("parse");
    assert!(
        matches!(
            p.rendering_intent(),
            WasmRenderingIntent::RelativeColorimetric
        ),
        "rendering intent should round-trip through serialization"
    );
}

#[wasm_bindgen_test]
fn rendering_intent_perceptual() {
    let bytes = WasmDisplayProfile::display_p3(WasmRenderingIntent::Perceptual)
        .to_bytes()
        .expect("serialize");
    let p = WasmProfile::from_bytes(&bytes).expect("parse");
    assert!(matches!(
        p.rendering_intent(),
        WasmRenderingIntent::Perceptual
    ));
}

// ---------------------------------------------------------------------------
// WasmDisplayProfile — custom builder
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn custom_profile_builds_and_parses() {
    let mut p = WasmDisplayProfile::new();
    p.set_rendering_intent(WasmRenderingIntent::RelativeColorimetric);
    p.set_profile_description("Test Profile");
    p.set_copyright("CC0 1.0");
    p.set_white_point(0.950455, 1.0, 1.08905);
    p.set_red_matrix_column(0.436066, 0.222488, 0.013916);
    p.set_green_matrix_column(0.385147, 0.716873, 0.097076);
    p.set_blue_matrix_column(0.143066, 0.060608, 0.714096);
    p.set_red_trc_gamma(2.2);
    p.set_green_trc_gamma(2.2);
    p.set_blue_trc_gamma(2.2);
    p.finalize();

    let bytes = p.to_bytes().expect("custom profile should serialize");
    assert!(bytes.len() > 128);
    assert_eq!(
        &bytes[36..40],
        b"acsp",
        "ICC file signature must be 'acsp' at offset 36"
    );

    // Must also parse back cleanly
    WasmProfile::from_bytes(&bytes).expect("custom profile bytes must parse back");
}

#[wasm_bindgen_test]
fn parametric_trc_builds_and_parses() {
    let mut p = WasmDisplayProfile::new();
    p.set_rendering_intent(WasmRenderingIntent::RelativeColorimetric);
    p.set_profile_description("Parametric TRC Test");
    p.set_copyright("CC0 1.0");
    p.set_white_point(0.950455, 1.0, 1.08905);
    p.set_red_matrix_column(0.436066, 0.222488, 0.013916);
    p.set_green_matrix_column(0.385147, 0.716873, 0.097076);
    p.set_blue_matrix_column(0.143066, 0.060608, 0.714096);
    // sRGB parametric curve (type 4, 5 params)
    p.set_red_trc_parametric(&[2.39999, 0.94786, 0.05214, 0.07739, 0.04045])
        .expect("valid sRGB parametric params");
    p.set_green_trc_parametric(&[2.39999, 0.94786, 0.05214, 0.07739, 0.04045])
        .expect("valid sRGB parametric params");
    p.set_blue_trc_parametric(&[2.39999, 0.94786, 0.05214, 0.07739, 0.04045])
        .expect("valid sRGB parametric params");
    p.finalize();

    let bytes = p
        .to_bytes()
        .expect("parametric TRC profile should serialize");
    WasmProfile::from_bytes(&bytes).expect("parametric TRC profile bytes must parse back");
}

#[wasm_bindgen_test]
fn chromatic_adaptation_wrong_size_errors() {
    let mut p = WasmDisplayProfile::new();
    // 8 values instead of 9 — must return Err
    let result = p.set_chromatic_adaptation(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
    assert!(result.is_err(), "8-element matrix should be rejected");
}

#[wasm_bindgen_test]
fn chromatic_adaptation_correct_size_succeeds() {
    let mut p = WasmDisplayProfile::new();
    #[rustfmt::skip]
    let result = p.set_chromatic_adaptation(&[
        1.047882, 0.022919, -0.050201,
        0.029587, 0.990479, -0.017059,
       -0.009232, 0.015076,  0.751678,
    ]);
    assert!(
        result.is_ok(),
        "9-element Bradford matrix should be accepted"
    );
}
