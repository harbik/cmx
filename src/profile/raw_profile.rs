// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use indexmap::IndexMap;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::str::FromStr;

use crate::profile::{set_profile_id, Profile};
use crate::signatures::{DeviceClass, Pcs};
use crate::tag::tagdata::TagData;
use crate::tag::TagDataTraits;
use crate::tag::{Tag, TagSignature};

/// # RawProfile
///
/// Represents a deconstructed ICC color profile with direct access to its binary structure.
///
/// ## Structure
/// An ICC profile consists of:
/// - A 128-byte header containing metadata about the profile
/// - A tag table listing all tags in the profile
/// - Tag data blocks containing the actual color data
///
/// `RawProfile` stores the header as a raw byte array and maintains tags in an ordered map
/// that preserves the original tag order from the source profile.
///
/// ## Tag Management
/// Rather than maintaining a separate tag table, `RawProfile` embeds offset and size information
/// directly in the `ProfileTagRecord` structure for each tag. This approach allows for:
/// - Direct access to tag data without parsing the entire profile
/// - Preservation of the original tag order from the source profile
/// - Automatic recalculation of offsets when tag data changes
///
/// ## Tag Sharing
/// ICC profiles can optimize storage by having multiple tag references point to the same data block.
/// `RawProfile` detects and preserves this optimization:
/// - When reading profiles, it detects duplicate tag offsets
/// - When writing profiles, it can share identical tag data by default
/// - Tag sharing can be disabled for round-trip testing purposes
///
/// ## Usage
/// `RawProfile` provides methods to:
/// - Read profiles from files or byte arrays
/// - Write profiles to files or byte arrays
/// - Access and modify profile header fields
/// - Access and modify tag data
/// - Convert between raw profiles and type-specific profile interfaces
///
/// ## Conversion
/// A `RawProfile` can be converted to a type-specific profile interface (like `InputProfile`
/// or `DisplayProfile`) based on its device class using the `into_class_profile()` method.
///
/// ## Binary Representation
/// When serializing to bytes, `RawProfile` ensures:
/// - Tag data is properly aligned on 4-byte boundaries
/// - Tag offsets and sizes are updated to reflect any changes
/// - Special tags like NamedColor2 are handled correctly with PCS information
/// - The profile ID is calculated if requested
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RawProfile {
    #[serde(with = "serde_arrays")]
    pub header: [u8; 128], // 128 bytes
    pub tags: IndexMap<TagSignature, ProfileTagRecord>, // preserves insertion order
    #[serde(skip)]
    shared_tags: bool,
    // whether to share tag data for identical tags
    // this is normally true, but can be set to false for round trip testing
    // as not all profiles use tag data sharing.
    // NOTE: This field is automatically detected from the profile structure during from_bytes(),
    // and may change during roundtrip (e.g., minimal profiles start with true but become false
    // after serialization, which is correct behavior as they don't actually have shared tags).
}

/// A record representing a tag in an ICC profile.
///
/// In ICC profiles, tag records contain metadata about tag elements including:
/// - The offset where the tag data begins in the profile
/// - The size of the tag data
/// - The actual tag data itself
///
/// This struct provides methods for creating new tag records and accessing
/// their raw byte representation.
///
/// # ICC Profile Tag Table
/// Tag records are part of the ICC profile's tag table, which acts as a directory
/// of all the tags within the profile. Each record points to the actual tag data
/// elsewhere in the profile file.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ProfileTagRecord {
    pub offset: u32,
    pub size: i32,
    pub tag: Tag,
}

impl ProfileTagRecord {
    /// Creates a new `ProfileTagRecord` with the given offset, size, and tag.
    /// It is used to represent a tag as present in a ICC profile, with its offset and size.
    /// size can be negative, which indicates that the tag data is shared with another tag,
    /// and the offset is not updated for the next tag.
    pub fn new(offset: u32, size: i32, tag: Tag) -> Self {
        Self { offset, size, tag }
    }

    /// Returns the raw bytes of the tag.
    pub fn as_slice(&self) -> &[u8] {
        self.tag.as_slice()
    }
}

impl Default for RawProfile {
    fn default() -> Self {
        Self {
            header: [0; 128],
            tags: IndexMap::new(),
            shared_tags: true,
        }
        .with_valid_file_signature()
        .with_version(4, 3)
        .unwrap()
        .with_pcs(Pcs::XYZ)
        .with_pcs_illuminant([0.9642, 1.0, 0.8249]) // Default to D50
        .with_now_as_creation_date() // Current date and time
    }
}

// Accept a slice to avoid needless Vec typing.
fn share_tags(tag_entries: &[(TagSignature, u32, u32)]) -> bool {
    // Duplicate offsets in the tag table imply shared data blocks.
    let mut seen_offsets = HashSet::new();
    for &(_sig, offset, _size) in tag_entries {
        if !seen_offsets.insert(offset) {
            return true;
        }
    }
    false
}

/// Implementation of the `RawProfile` struct for handling ICC color profiles.
///
/// A `RawProfile` represents an ICC color profile in a raw form with direct access to the
/// profile header, tag table, and tag data. This implementation provides methods to:
///
/// - Read profiles from files or byte arrays
/// - Write profiles to files
/// - Convert profiles to binary data
/// - Update tag record offsets and sizes
/// - Convert raw profiles to specific device class profiles
///
/// ICC profiles consist of a 128-byte header, a tag count, a tag table, and tag data.
/// The tag table contains records of tag signatures, offsets, and sizes, which point to
/// the actual tag data in the file. This implementation handles the complexities of
/// reading and writing this structure, including special handling for certain tags
/// (like NamedColor2) and optional tag data sharing.
///
/// Tag sharing is an optimization where identical tag data blocks are stored only once
/// in the profile and referenced by multiple tag table entries.
impl RawProfile {
    /// Reads an ICC profile from a file.
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Self::from_bytes(&buf)
    }

    /// Reads an ICC profile from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(bytes);

        // Read the header (first 128 bytes)
        let mut header = [0u8; 128];
        cursor.read_exact(&mut header)?;

        // Read the tag count (next 4 bytes)
        let mut count_buf = [0u8; 4];
        cursor.read_exact(&mut count_buf)?;
        let tag_count = u32::from_be_bytes(count_buf);

        // Read the tag table (next tag_count * 12 bytes)
        // Each tag entry consists of 12 bytes:
        // - 4 bytes for signature
        // - 4 bytes for offset in the file
        // - 4 bytes for size
        let mut tag_entries = Vec::with_capacity(tag_count as usize);

        let mut max_end = 0usize;
        for _ in 0..tag_count {
            let mut entry = [0u8; 12];
            cursor.read_exact(&mut entry)?;

            // Parse the tag entry
            let signature_value = u32::from_be_bytes([entry[0], entry[1], entry[2], entry[3]]);
            let signature = TagSignature::new(signature_value);

            // Convert the offset and size from big-endian to u32
            let offset = u32::from_be_bytes([entry[4], entry[5], entry[6], entry[7]]);
            let size = u32::from_be_bytes([entry[8], entry[9], entry[10], entry[11]]);

            // Track the farthest end position across all tags
            let end = offset as usize + size as usize;
            if end > max_end {
                max_end = end;
            }

            // Store the tag entry in the tag table
            tag_entries.push((signature, offset, size));
        }

        // Detect if the source profile used shared tag data (duplicate offsets)
        let shared_tags = share_tags(&tag_entries);

        // Create a map to hold the tags
        let mut tags = IndexMap::with_capacity(tag_count as usize);

        // Read each tag's data based on the offsets and sizes from the tag table
        for (signature, offset, size) in &tag_entries {
            // Bounds check to avoid out-of-bounds reads
            let end = *offset as usize + *size as usize;
            if end > bytes.len() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "tag data offset/size exceeds input length",
                )));
            }

            // Move the cursor to the tag's offset and read its data
            cursor.set_position(*offset as u64);

            // Read the tag data
            let mut data = vec![0u8; *size as usize];
            cursor.read_exact(&mut data)?;

            // Special handling for specific tag signatures
            // NamedColor2 needs to know the PCS (Profile Connection Space) type, which is either XYZ or Lab.
            // If it is Lab, we set the PCS flag in the private flag field (bit 17) of this tag.
            // This will be reset in the tobytes() method when generating the profile.
            if signature == &TagSignature::NamedColor2 {
                // Check the PCS in the header (bytes 20-23)
                let pcs = u32::from_be_bytes(header[20..24].try_into().unwrap()); // slice has 4 bytes
                if pcs == 0x4C616220 {
                    // Uses "Lab " connection space
                    // Get the flag field of this tag (bytes 8-11 of the tag data)
                    let mut flag = u32::from_be_bytes(data[8..12].try_into().unwrap()); // slice has 4 bytes
                    flag |= 0x1_0000; // Set the PCS flag
                                      // set the updated flag back in the tag data
                    data[8..12].copy_from_slice(&flag.to_be_bytes()); // Update the flag in the data
                }
            }

            tags.insert(
                *signature,
                ProfileTagRecord {
                    offset: *offset,
                    size: *size as i32, // size is stored as i32 in the tag record
                    tag: Tag::new(signature.to_u32(), TagData::new(data)),
                },
            );
        }

        Ok(RawProfile {
            header,
            tags,
            shared_tags,
        })
    }

    /*
    /// Reads an ICC profile from a string (as bytes).
    pub fn from_str(s: &str) -> Result<Self> {
        Self::from_bytes(s.as_bytes())
    }
     */

    /// Writes the ICC profile to a file.
    pub fn write<P: AsRef<Path>>(self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.into_bytes()?;
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    pub fn into_bytes(mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Update offsets and sizes of tags based on their data
        // Tag offsets and sizes are updated, with offsets increasing with position of the tag
        // in the indexmap, and sizes being the length of the tag data.
        // The tag table entries have the same order as the tag table, with all tags
        // lined up in the order they were added to the profile.

        self = self.with_updated_tagrecord_offsets_and_sizes();
        let mut buf = Vec::new();

        // Copy header
        buf.extend_from_slice(&self.header);
        debug_assert!(buf.len() == 128, "Header should be exactly 128 bytes long");

        // Write tag count
        let tag_count = self.tags.len() as u32;
        buf.extend_from_slice(&tag_count.to_be_bytes());
        debug_assert!(
            buf.len() == 128 + 4,
            "Header + tag count should be 132 bytes long"
        );

        // Write tag table (each entry is 12 bytes)
        for (sig, tag) in &self.tags {
            buf.extend_from_slice(&sig.to_u32().to_be_bytes());
            buf.extend_from_slice(&tag.offset.to_be_bytes());

            // shared tags have negative size, but
            // we store the absolute value in the tag table.
            buf.extend_from_slice(&tag.size.abs().to_be_bytes());
        }
        debug_assert!(
            buf.len() == 128 + 4 + self.tags.len() * 12,
            "Header + tag count + tag table should be {} bytes long",
            128 + 4 + self.tags.len() * 12
        );

        // Write tag data directly to the buffer
        for (tag_signature, mut tag) in self.tags {
            // Write all tags when sharing disabled; otherwise write only primaries (size > 0)
            let should_write = if self.shared_tags { tag.size > 0 } else { true };
            if should_write {
                // If the current buffer length is less than the tag's offset, we need to pad it
                // to ensure we write at the correct position.
                if buf.len() < tag.offset as usize {
                    buf.resize(tag.offset as usize, 0);
                }

                // reset the flags field for NamedColor2 tags if needed
                if tag_signature == TagSignature::NamedColor2 {
                    // Check the PCS in the header (bytes 20-23)
                    let pcs = u32::from_be_bytes(self.header[20..24].try_into().unwrap()); // slice has 4 bytes
                    if pcs == 0x4C616220 {
                        // Uses "Lab " connection space
                        // Get the flag field of this tag (bytes 8-11 of the tag data)
                        let mut flag =
                            u32::from_be_bytes(tag.tag.as_slice()[8..12].try_into().unwrap()); // slice has 4 bytes
                                                                                               // Clear the flag
                        flag &= !0x1_0000; // Clear the PCS flag
                                           // set the updated flag back in the tag data
                        tag.tag.data_mut().as_mut_slice()[8..12]
                            .copy_from_slice(&flag.to_be_bytes()); // Update the flag in the data
                    };
                }

                // Append the tag data
                buf.extend_from_slice(tag.tag.as_slice());
            } // else do nothing, tag data is shared and already written.
        }

        // All Tags written to buf, add padding if needed if the last tag does not end on a 4-byte boundary.
        buf.extend(vec![0u8; crate::pad_size(buf.len())]);

        // Update profile size
        let length = buf.len() as u32;
        buf[0..4].copy_from_slice(&length.to_be_bytes());

        // calculate the profile ID if requested
        if buf[99] > 0 {
            // calculate it
            set_profile_id(&mut buf);
        } else {
            buf[84..=99].fill(0); // clear the profile ID
        }

        Ok(buf)
    }

    /// Serializes the ICC profile to a String (best-effort for debugging; lossy for non-UTF-8).
    pub fn into_string(self) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = self.into_bytes()?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    // In a RawProfile, as defined in this library, the tag table information is embedded
    // in the `tags` field, which contains the offsets and sizes of the tags, as well as the
    // tag data itself.
    // The offsets and sizes are copied from the tag table when reading a binary profile from file
    // but have to be changed if the tag data changes, for example when adding or removing tags,
    // or when changing the tag data.
    // This method updates the offsets and sizes of the tags based on their data, ensuring that
    // the profile can be written back to a file with correct offsets and sizes.
    fn with_updated_tagrecord_offsets_and_sizes(mut self) -> Self {
        // Calculate start of tag data area
        let tag_count = self.tags.len();
        let data_start = 128 + 4 + (tag_count * 12);
        let mut offset_for_next_tag = crate::padded_size(data_start);

        // If the source profile did not use tag sharing, assign unique offsets/sizes and return.
        if !self.shared_tags {
            for (_signature, tag_record) in self.tags.iter_mut() {
                tag_record.offset = offset_for_next_tag as u32;
                tag_record.size = tag_record.tag.len() as i32;
                offset_for_next_tag += crate::padded_size(tag_record.tag.len());
            }
            // Ensure we did not accidentally mark any tag as shared in this path.
            debug_assert!(self.tags.values().all(|t| t.size > 0));
            return self;
        }

        // Otherwise, share identical payloads: subsequent identical tags get same offset and negative size.
        let mut shared_location: HashMap<&[u8], (u32, i32)> = HashMap::new();

        for (_signature, tag_record) in self.tags.iter_mut() {
            (tag_record.offset, tag_record.size) = if let Some(&(offset, size)) =
                shared_location.get(tag_record.tag.data().as_slice())
            {
                (offset, -size)
            } else {
                let offset = offset_for_next_tag as u32;
                let size = tag_record.tag.len() as i32;
                shared_location.insert(tag_record.tag.data().as_slice(), (offset, size));
                offset_for_next_tag += crate::padded_size(tag_record.tag.len());
                (offset, size)
            };
        }

        self
    }

    /// Returns whether this profile will use shared tag data when (re)serialized.
    pub fn uses_shared_tags(&self) -> bool {
        self.shared_tags
    }

    pub fn into_class_profile(self) -> Profile {
        match self.device_class() {
            DeviceClass::Input => Profile::Input(super::InputProfile(self)),
            DeviceClass::Display => Profile::Display(super::DisplayProfile(self)),
            DeviceClass::Output => Profile::Output(super::OutputProfile(self)),
            DeviceClass::DeviceLink => Profile::DeviceLink(super::DeviceLinkProfile(self)),
            DeviceClass::Abstract => Profile::Abstract(super::AbstractProfile(self)),
            DeviceClass::ColorSpace => Profile::ColorSpace(super::ColorSpaceProfile(self)),
            DeviceClass::NamedColor => Profile::NamedColor(super::NamedColorProfile(self)),
            DeviceClass::Spectral => Profile::Spectral(super::SpectralProfile(self)),
            DeviceClass::None => Profile::Raw(self),
        }
    }
}

impl FromStr for RawProfile {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}
