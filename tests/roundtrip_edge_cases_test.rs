#[cfg(test)]
mod roundtrip_edge_cases_test {
    use cmx::profile::RawProfile;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_roundtrip_with_profile_id_flag() {
        // Test that profiles with profile ID requested roundtrip correctly
        let profile_with_id = RawProfile::default()
            .with_profile_id()
            .with_pcs(cmx::signatures::Pcs::XYZ)
            .with_version(4, 3)
            .unwrap();

        let profile_id_before = profile_with_id.profile_id();
        println!("Profile ID before serialization: {profile_id_before:?}");

        // Serialize to bytes
        let bytes = profile_with_id.into_bytes().unwrap();
        let profile_id_in_bytes = &bytes[84..=99];
        println!("Profile ID in serialized bytes: {profile_id_in_bytes:?}");

        // Read back from bytes
        let profile_roundtrip = RawProfile::from_bytes(&bytes).unwrap();
        let profile_id_after = profile_roundtrip.profile_id();
        println!("Profile ID after roundtrip: {profile_id_after:?}");

        // The profile ID should not be the flag anymore
        assert_ne!(
            profile_id_after[15], 1,
            "Profile ID should not contain the flag after serialization"
        );

        // The profile ID in bytes and after roundtrip should match
        assert_eq!(
            profile_id_in_bytes, profile_id_after,
            "Profile ID should be preserved in roundtrip"
        );

        // Roundtrip again to ensure stability
        let bytes2 = profile_roundtrip.into_bytes().unwrap();
        assert_eq!(bytes, bytes2, "Second roundtrip should be identical");

        println!("Roundtrip with profile ID test passed");
    }

    #[test]
    fn test_roundtrip_without_profile_id_flag() {
        // Test that profiles without profile ID requested remain without it
        let profile_without_id = RawProfile::default()
            .without_profile_id()
            .with_pcs(cmx::signatures::Pcs::XYZ)
            .with_version(4, 3)
            .unwrap();

        let profile_id_before = profile_without_id.profile_id();
        println!("Profile ID before serialization: {profile_id_before:?}");

        // Should be all zeros
        assert!(
            profile_id_before.iter().all(|&x| x == 0),
            "Profile ID should be all zeros"
        );

        // Serialize to bytes
        let bytes = profile_without_id.into_bytes().unwrap();
        let profile_id_in_bytes = &bytes[84..=99];
        println!("Profile ID in serialized bytes: {profile_id_in_bytes:?}");

        // Should still be all zeros
        assert!(
            profile_id_in_bytes.iter().all(|&x| x == 0),
            "Profile ID should remain all zeros in serialized bytes"
        );

        // Read back from bytes
        let profile_roundtrip = RawProfile::from_bytes(&bytes).unwrap();
        let profile_id_after = profile_roundtrip.profile_id();
        println!("Profile ID after roundtrip: {profile_id_after:?}");

        // Should still be all zeros
        assert!(
            profile_id_after.iter().all(|&x| x == 0),
            "Profile ID should remain all zeros after roundtrip"
        );

        // Roundtrip again to ensure stability
        let bytes2 = profile_roundtrip.into_bytes().unwrap();
        assert_eq!(bytes, bytes2, "Second roundtrip should be identical");

        println!("Roundtrip without profile ID test passed");
    }

    #[test]
    fn test_real_profile_checksum_verification() {
        // Test with a real profile to verify checksum calculation
        let test_profile_path = Path::new("tests/profiles/Display P3.icc");
        if test_profile_path.exists() {
            let original_bytes = fs::read(test_profile_path).unwrap();
            println!("Original profile size: {} bytes", original_bytes.len());

            // Read the profile
            let profile = RawProfile::from_bytes(&original_bytes).unwrap();
            let original_profile_id = profile.profile_id();
            println!("Original profile ID: {original_profile_id:?}");

            // Set the profile to calculate its ID
            let profile_with_id = profile.with_profile_id();
            let bytes_with_calculated_id = profile_with_id.into_bytes().unwrap();
            let calculated_profile_id = &bytes_with_calculated_id[84..=99];
            println!("Calculated profile ID: {calculated_profile_id:?}");

            // The calculated ID should match what's expected for this profile
            // (This is the checksum from the test in checksum.rs)
            let expected_checksum: [u8; 16] = [
                0xca, 0x1a, 0x95, 0x82, 0x25, 0x7f, 0x10, 0x4d, 0x38, 0x99, 0x13, 0xd5, 0xd1, 0xea,
                0x15, 0x82,
            ];

            assert_eq!(
                calculated_profile_id, expected_checksum,
                "Calculated profile ID should match expected checksum for Display P3 profile"
            );

            println!("Real profile checksum verification passed");
        } else {
            println!("Display P3.icc not found, skipping real profile test");
        }
    }
}
