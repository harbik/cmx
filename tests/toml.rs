
use std::fs;
use std::path::Path;
use cmx::profile::RawProfile; // Adjust the path to your crate/module


#[test]
fn test_profile_toml_display() {
    let input_path = Path::new("tests/profiles/sRGB.icc");
    let profile_bytes = fs::read(input_path).expect("Failed to read test profile");

    // Read profile
    let profile = RawProfile::from_bytes(&profile_bytes).expect("Failed to parse profile");

    println!("{}", cmx::profile::Profile::Raw(profile));
}
/*

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
 */