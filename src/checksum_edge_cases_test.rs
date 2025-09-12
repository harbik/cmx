#[cfg(test)]
mod checksum_edge_cases_test {
    use crate::profile::{md5checksum, set_profile_id};

    #[test]
    fn test_md5checksum_preserves_original_data() {
        // Test that md5checksum properly restores the original data
        let mut test_data = vec![0u8; 128];
        
        // Set some test values in the fields that should be preserved
        test_data[44] = 0x12; // flags byte 1
        test_data[45] = 0x34; // flags byte 2
        test_data[46] = 0x56; // flags byte 3
        test_data[47] = 0x78; // flags byte 4
        
        test_data[64] = 0xAB; // rendering intent byte 1
        test_data[65] = 0xCD; // rendering intent byte 2
        test_data[66] = 0xEF; // rendering intent byte 3
        test_data[67] = 0x01; // rendering intent byte 4
        
        // Set some test values in profile ID (should be cleared)
        for i in 84..=99 {
            test_data[i] = 0xFF;
        }
        
        // Store original data for comparison (unused but kept for potential debugging)
        let _original_data = test_data.clone();
        
        // Calculate checksum
        let checksum = md5checksum(&mut test_data);
        
        // Verify flags are restored
        assert_eq!(test_data[44], 0x12, "Flags byte 1 not restored");
        assert_eq!(test_data[45], 0x34, "Flags byte 2 not restored");
        assert_eq!(test_data[46], 0x56, "Flags byte 3 not restored");
        assert_eq!(test_data[47], 0x78, "Flags byte 4 not restored");
        
        // Verify rendering intent is restored
        assert_eq!(test_data[64], 0xAB, "Rendering intent byte 1 not restored");
        assert_eq!(test_data[65], 0xCD, "Rendering intent byte 2 not restored");
        assert_eq!(test_data[66], 0xEF, "Rendering intent byte 3 not restored");
        assert_eq!(test_data[67], 0x01, "Rendering intent byte 4 not restored");
        
        // Verify profile ID is cleared (this is expected behavior)
        for i in 84..=99 {
            assert_eq!(test_data[i], 0, "Profile ID byte {} not cleared", i);
        }
        
        // Verify checksum is not all zeros
        assert!(!checksum.iter().all(|&x| x == 0), "Checksum should not be all zeros");
        
        println!("Checksum: {:?}", checksum);
        println!("Test passed: md5checksum preserves original data correctly");
    }
    
    #[test]
    fn test_md5checksum_minimum_size() {
        // Test with exactly 128 bytes (minimum size)
        let mut test_data = vec![0u8; 128];
        test_data[0] = 0x01; // Add some non-zero data
        
        let checksum = md5checksum(&mut test_data);
        assert!(!checksum.iter().all(|&x| x == 0), "Checksum should not be all zeros");
        
        println!("Minimum size test passed");
    }
    
    #[test]
    #[should_panic(expected = "ICC profile data must be at least 128 bytes long")]
    fn test_md5checksum_too_small() {
        // Test with less than 128 bytes (should panic)
        let mut test_data = vec![0u8; 127];
        md5checksum(&mut test_data);
    }
    
    #[test]
    fn test_set_profile_id_consistency() {
        // Test that set_profile_id is consistent
        let mut test_data1 = vec![0u8; 128];
        let mut test_data2 = vec![0u8; 128];
        
        // Add same data to both
        test_data1[10] = 0x42;
        test_data2[10] = 0x42;
        
        let checksum1 = set_profile_id(&mut test_data1);
        let checksum2 = set_profile_id(&mut test_data2);
        
        // Checksums should be identical for identical data
        assert_eq!(checksum1, checksum2, "Checksums should be identical for identical data");
        
        // Profile ID fields should be set to the checksum
        assert_eq!(&test_data1[84..=99], &checksum1[..], "Profile ID should match checksum");
        assert_eq!(&test_data2[84..=99], &checksum2[..], "Profile ID should match checksum");
        
        println!("Consistency test passed");
    }
}