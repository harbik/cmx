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
    let target_dir = Path::new("tmp/toml-test");

    // Create target directory if it doesn't exist
    fs::create_dir_all(target_dir).expect("Failed to create target directory");

    for entry in fs::read_dir(profiles_dir).expect("Failed to read profiles directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("icc") {
            // Get the filename without extension
            let file_stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("Failed to get filename");

            // Create output path with .toml extension
            let output_path = target_dir.join(format!("{file_stem}.toml"));

            println!("Processing: {path:?} -> {output_path:?}");

            // Read and parse the profile
            let profile_bytes = fs::read(&path).expect("Failed to read test profile");
            let profile = RawProfile::from_bytes(&profile_bytes).expect("Failed to parse profile");

            // Generate TOML content
            let toml_content = cmx::profile::Profile::Raw(profile).to_string();

            // Write to file
            fs::write(&output_path, toml_content)
                .unwrap_or_else(|_| panic!("Failed to write TOML file: {output_path:?}"));
        }
    }

    println!("All profiles converted and saved to {target_dir:?}");
}
