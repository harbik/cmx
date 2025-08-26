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
    let profile2 = RawProfile::from_bytes(&roundtrip).expect("Failed to parse roundtrip profile");
    if original != roundtrip {
        //  print_first_diff(&original, &roundtrip);
        // let roundtrip_profile = RawProfile::from_bytes(&roundtrip)
        //     .expect("Failed to parse roundtrip profile");
        for (l, (i,j)) in original.iter().zip(roundtrip.iter()).enumerate() {
            if i != j {
                println!("byte {l}: left = {i}, right = {j}");
            }
        }
    }

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
            let roundtrip = profile.clone().into_bytes().expect("Failed to serialize profile");
            if original != roundtrip {
              //  print_first_diff(&original, &roundtrip);
               // let roundtrip_profile = RawProfile::from_bytes(&roundtrip)
               //     .expect("Failed to parse roundtrip profile");
                println!("Error in profile: {path:?} ... ");
                for (l, (i,j)) in original.iter().zip(roundtrip.iter()).enumerate() {
                    if i != j {
                        println!("byte {l}: left = {i}, right = {j}");
                    }
                }
            }
        }
    }
}
