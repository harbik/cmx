#[cfg(test)]
mod profile_id_logic_test {
    use crate::profile::RawProfile;

    #[test]
    fn test_profile_id_flag_logic() {
        println!("Testing profile ID flag logic...");
        
        // Test 1: Profile without profile ID request
        let profile1 = RawProfile::default().without_profile_id();
        let bytes1 = profile1.into_bytes().unwrap();
        let profile_id_bytes1 = &bytes1[84..=99];
        println!("Profile without ID - bytes 84-99: {:?}", profile_id_bytes1);
        println!("Byte 99: {}", bytes1[99]);
        
        // All bytes should be zero for without_profile_id
        assert_eq!(bytes1[99], 0);
        assert!(profile_id_bytes1.iter().all(|&x| x == 0));
        
        // Test 2: Profile with profile ID request  
        let profile2 = RawProfile::default().with_profile_id();
        let profile_id_before = profile2.profile_id();
        println!("Profile ID before serialization: {:?}", profile_id_before);
        println!("Last byte before serialization: {}", profile_id_before[15]);
        
        // The flag should be set (byte 99 should be 1)
        assert_eq!(profile_id_before[15], 1);
        
        let bytes2 = profile2.into_bytes().unwrap();
        let profile_id_bytes2 = &bytes2[84..=99];
        println!("Profile with ID - bytes 84-99: {:?}", profile_id_bytes2);
        println!("Byte 99 after serialization: {}", bytes2[99]);
        
        // After serialization, should contain a valid MD5 hash (not all zeros, not the flag)
        assert_ne!(bytes2[99], 0);
        assert_ne!(bytes2[99], 1); // Should not be the flag anymore
        assert!(!profile_id_bytes2.iter().all(|&x| x == 0)); // Should not be all zeros
        
        // Test 3: Verify roundtrip
        let profile3 = RawProfile::from_bytes(&bytes2).unwrap();
        let profile_id_after = profile3.profile_id();
        println!("Profile ID after roundtrip: {:?}", profile_id_after);
        
        // The profile ID should be preserved in roundtrip
        assert_eq!(profile_id_after, profile_id_bytes2);
        
        println!("Test completed successfully.");
    }
}