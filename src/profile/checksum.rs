// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! This module provides functionality to calculate and set the MD5 checksum for ICC profiles, and to set
//! the profile ID based on the checksum.

/// The checksum is calculated based on the profile's data, excluding the flags, rendering intent, and profile id fields
/// which are set to zero during the calculation. The checksum is a 16-byte MD5 hash of the profile data.
/// It is used to verify the integrity of the profile and is stored in the profile's header.
///
/// # Notes:
/// - We need a mut reference to the byte slice because we modify it by zeroing out certain fields.
pub fn md5checksum(bytes: &mut [u8]) -> [u8; 16] {
    if bytes.len() < 128 {
        panic!("ICC profile data must be at least 128 bytes long");
    }

    let flags: [u8; 4] = bytes[40..=43].try_into().unwrap();
    bytes[44..=47].fill(0);

    let rendering_intent: [u8; 4] = bytes[64..=67].try_into().unwrap();
    bytes[64..=67].fill(0);

    // clear the profile ID
    bytes[84..=99].fill(0);

    // calculate the checksum
    let digest = md5::compute(&bytes);
    let checksum: [u8; 16] = digest.into();

    bytes[44..=47].copy_from_slice(&flags);
    bytes[64..=67].copy_from_slice(&rendering_intent);
    checksum
}

/// Sets the profile ID in the ICC profile data.
/// This is used just before writing the profile to a file.
///
/// # Notes:
/// - The profile ID is a 16-byte MD5 checksum of the profile data, excluding the flags, rendering intent, and profile ID fields.
/// - This is not a method of the `RawProfile` struct, as that is a parsed ICC profile, and the
///   checksum has to be calculated from the binary profile representations, as stored in a file, or
///   embedded in an image.
pub fn set_profile_id(bytes: &mut [u8]) -> [u8; 16] {
    let checksum = md5checksum(bytes);
    bytes[84..=99].copy_from_slice(&checksum);
    checksum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_with_profile_id() {
        let icc_data = include_bytes!("../../tests/profiles/Display P3.icc");
        let mut icc_data_mut = icc_data.to_vec();

        // &mut Vec<u8> is a mutable byte slice because it implements `DerefMut` for `&mut [u8]`
        let result = set_profile_id(&mut icc_data_mut);

        // checksum as reported by ColorSync
        let expected_checksum: [u8; 16] = [
            0xca, 0x1a, 0x95, 0x82, 0x25, 0x7f, 0x10, 0x4d, 0x38, 0x99, 0x13, 0xd5, 0xd1, 0xea,
            0x15, 0x82,
        ];

        assert_eq!(&result, &expected_checksum);
    }
}
