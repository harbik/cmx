use cmx::profile::RawProfile;
use std::fs;
use std::path::Path; // Adjust the path to your crate/module

fn print_first_diff(a: &[u8], b: &[u8]) {
    let min_len = a.len().min(b.len());
    for i in 0..min_len {
        if a[i] != b[i] {
            println!(
                "Difference at byte {}: left = {}, right = {}",
                i, a[i], b[i]
            );
            return;
        }
    }
    if a.len() != b.len() {
        println!("Length differs: left = {}, right = {}", a.len(), b.len());
    } else {
        println!("No difference found.");
    }
}

#[test]
fn test_icc_roundtrip() {
    let input_path = Path::new("tests/profiles/sRGB.icc");
    let original = fs::read(input_path).expect("Failed to read test profile");

    // Read profile
    let profile = RawProfile::from_bytes(&original).expect("Failed to parse profile");

    // Write back to bytes
    let roundtrip = profile.into_bytes().expect("Failed to serialize profile");
    if original.len() != roundtrip.len() {
        println!(
            "Original length: {}, Roundtrip length: {}",
            original.len(),
            roundtrip.len()
        );
        print_first_diff(&original, &roundtrip);
    }
    if original != roundtrip {
        print_first_diff(&original, &roundtrip);
        assert_eq!(
            original, roundtrip,
            "Round-trip ICC profile does not match original"
        );
        return;
    }

    // Compare
}

#[test]
fn test_icc_roundtrip_all_profiles() {
    let profiles_dir = Path::new("tests/profiles");
    for entry in fs::read_dir(profiles_dir).expect("Failed to read profiles directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        println!("Testing profile: {:?} ... ", path);
        if path.extension().and_then(|s| s.to_str()) == Some("icc") {
            let original = fs::read(&path).expect("Failed to read test profile");
            let profile = RawProfile::from_bytes(&original).expect("Failed to parse profile");
            let roundtrip = profile.into_bytes().expect("Failed to serialize profile");
            if original != roundtrip {
                print_first_diff(&original, &roundtrip);
            }
        }
    }
}
