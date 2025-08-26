//! ICC Profile Roundtrip Tests
//!
//! This module contains tests that verify the roundtrip serialization and deserialization
//! of ICC color profiles using the `cmx` crate's `RawProfile` functionality.
//!
//! Two main tests are included:
//! - A specific test for the Adobe RGB 1998 profile
//! - A comprehensive test that checks all ICC profiles in the test directory
//!
//! The tests ensure that profiles can be read from bytes, converted back to bytes,
//! and result in identical binary data, confirming that no information is lost or
//! corrupted during the process.
//!
//! If differences are found, the tests will print the byte position and values
//! where the original and roundtrip data differ, helping with debugging.
use cmx::profile::RawProfile;
use std::fs;
use std::path::Path; // Adjust the path to your crate/module

#[test]
fn test_icc_adobe_rgb_1998() {
    let input_path = Path::new("tests/profiles/AdobeRGB1998.icc");
    let original = fs::read(input_path).expect("Failed to read test profile");

    // Read profile
    let profile = RawProfile::from_bytes(&original).expect("Failed to parse profile");

    // Write back to bytes
    let roundtrip = profile.into_bytes().expect("Failed to serialize profile");
    let _profile2 = RawProfile::from_bytes(&roundtrip).expect("Failed to parse roundtrip profile");
    if original != roundtrip {
        for (l, (i, j)) in original.iter().zip(roundtrip.iter()).enumerate() {
            if i != j {
                println!("byte {l}: left = {i}, right = {j}");
            }
        }
    }
    assert!(original == roundtrip);
}

#[test]
fn test_icc_roundtrip_all_profiles() {
    let profiles_dir = Path::new("tests/profiles");
    for entry in fs::read_dir(profiles_dir).expect("Failed to read profiles directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("icc") {
            let original = fs::read(&path).expect("Failed to read test profile");
            let profile = RawProfile::from_bytes(&original).expect("Failed to parse profile");
            let roundtrip = profile
                .clone()
                .into_bytes()
                .expect("Failed to serialize profile");
            if original != roundtrip {
                println!("Error in profile: {path:?} ... ");
                for (l, (i, j)) in original.iter().zip(roundtrip.iter()).enumerate() {
                    if i != j {
                        println!("byte {l}: left = {i}, right = {j}");
                    }
                }
            }
            assert!(original == roundtrip);
        }
    }
}
