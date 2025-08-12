use cmx::profile::RawProfile;
use std::fs;
use std::path::Path; // Adjust the path to your crate/module

#[test]
fn test_profile_toml_display() {
    let input_path = Path::new("tests/profiles/sRGB.icc");
    let profile_bytes = fs::read(input_path).expect("Failed to read test profile");

    // Read profile
    let profile = RawProfile::from_bytes(&profile_bytes).expect("Failed to parse profile");

    println!("{}", cmx::profile::Profile::Raw(profile));
}

#[test]
fn toml_all_profiles() {
    let profiles_dir = Path::new("tests/profiles");
    for entry in fs::read_dir(profiles_dir).expect("Failed to read profiles directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        // bold ANSI escape code bold
        println!("\u{001b}[1;4mProfile: {:?} ... \u{001b}[0m", path);
        if path.extension().and_then(|s| s.to_str()) == Some("icc") {
            let profile_bytes = fs::read(path).expect("Failed to read test profile");
            let profile = RawProfile::from_bytes(&profile_bytes).expect("Failed to parse profile");
            println!("{}", cmx::profile::Profile::Raw(profile));
        }
    }
}
