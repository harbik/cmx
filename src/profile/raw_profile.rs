// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use indexmap::IndexMap;
use serde::Serialize;
use zerocopy::{U32, BigEndian, FromBytes, IntoBytes, KnownLayout};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::str::FromStr;

use crate::profile::Profile;
use crate::signatures::{DeviceClass, Pcs};
use crate::tag::tagdata::TagData;
use crate::tag::TagDataTraits;
use crate::tag::{Tag, TagSignature};

/// An ICC profile, deconstructed in:
///
/// - a raw header array, with a length of 128 bytes,
/// - a indexmap of datablocks, with a `TagSignature`` as key and `TagBytes`` as value.
///
/// An indexmap is used to preserve the insertion order of tags, which is technically not required
/// by the ICC specification, but is used to maintain the order of tags as they appear in profiles
/// read from a a file file, and to maximize compatibility with existing ICC profiles.
///
/// It does not include a separate tag table; the profile tags are the used as the indexmap's key,
/// while offsets and sizes are included in the `DataBlock` struct. Those offsets and sizes
/// are used to recreate the tag table on writing.
/// Whenever the size of a tag's data changes, the offsets and sizes of all tags are updated.
///
///
///


/// Represents a single tag entry in an ICC profile,
/// containing an offset, size, and it's raw tag bytes,
/// and is the main interface for accessing tag data through
/// the IndexMap in the `RawProfile`.
/// These are the offset and size used to import the tag data from
/// the raw bytes of the ICC profile, and are also used to write the
/// tag data back to the raw bytes when exporting the profile.
/// When writing the data the size of all the tags is checked to see
/// if any tag data has changed in size, and if so all the tags are
/// re-arranged to fit the new size.
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

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RawProfile {
    #[serde(with = "serde_arrays")]
    pub header: [u8; 128], // 128 bytes
    pub tags: IndexMap<TagSignature, ProfileTagRecord>, // preserves insertion order
}



#[derive(Debug, FromBytes, IntoBytes, KnownLayout)]
#[repr(C)]
struct ICCTagtableEntryLayout {
    tag_signature: U32<BigEndian>,
    offset: U32<BigEndian>,
    size: U32<BigEndian>,
}

impl Default for RawProfile {
    fn default() -> Self {
        Self {
            header: [0; 128],
            tags: IndexMap::new(),
        }
        .with_valid_file_signature()
        .with_version(4, 3)
        .unwrap()
        .with_pcs(Pcs::XYZ)
        .with_pcs_illuminant([0.9642, 1.0, 0.8249]) // Default to D50
        .with_creation_date(None) // Current date and time
    }
}

impl RawProfile {
    /// Reads an ICC profile from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
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
            let mut data = vec![0u8; *size as usize];
            cursor.read_exact(&mut data)?;

            // Special handling for specific tag signatures
            // NamedColor2 needs to know the PCS (Profile Connection Space) type, which is either XYZ or Lab.
            // If it is Lab, we set the PCS flag in the private flag field (bit 17) of this tag.
            if signature == &TagSignature::NamedColor2 {
                let pcs = u32::from_be_bytes(header[20..24].try_into().unwrap()); // slice has 4 bytes
                if pcs == 0x4C616220 {
                    // "Lab "
                    let mut flag = u32::from_be_bytes(data[8..12].try_into().unwrap()); // slice has 4 bytes
                    flag |= 0x1_0000; // Set the PCS flag
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
        })
    }

    /*
    /// Reads an ICC profile from a string (as bytes).
    pub fn from_str(s: &str) -> Result<Self> {
        Self::from_bytes(s.as_bytes())
    }
     */

    /// Writes the ICC profile to a file.
    pub fn to_file<P: AsRef<Path>>(self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.into_bytes()?;
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    pub fn into_bytes(mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Update offsets and sizes of tags based on their data
        // Tag offsets and sizes are updated, with offsets increasing with position of the tag
        // in the indexmap, and sizes being the length of the tag data.
        // The tag table entries have the same order as the tag table, with all tages
        // lined up in the order they were added to the profile.

        self = self.with_updated_tagrecord_offsets_and_sizes();
        let mut buf = Vec::new();
        
        // Copy header
        buf.extend_from_slice(&self.header);
        debug_assert!(buf.len() == 128, "Header should be exactly 128 bytes long");
         
        // Write tag count
        let tag_count = self.tags.len() as u32;
        buf.extend_from_slice(&tag_count.to_be_bytes());
        debug_assert!(buf.len() == 128 + 4, "Header + tag count should be 132 bytes long");
        
        // Write tag table (each entry is 12 bytes)
        for (sig, tag) in &self.tags {
            buf.extend_from_slice(&sig.to_u32().to_be_bytes());
            buf.extend_from_slice(&tag.offset.to_be_bytes());

            // shared tags have negative size, but 
            // we store the absolute value in the tag table.
            buf.extend_from_slice(&tag.size.abs().to_be_bytes()); 
        }
        debug_assert!(buf.len() == 128 + 4 + self.tags.len() * 12,
                      "Header + tag count + tag table should be {} bytes long",
                      128 + 4 + self.tags.len() * 12);


        // Write tag data directly to the buffer
        for tag in self.tags.values() {
            if tag.size > 0 {
                    // If the size is negative, it means the tag data is shared and we do not write it again.
                // If the current buffer length is less than the tag's offset, we need to pad it
                // to ensure we write at the correct position.
                if buf.len() < tag.offset as usize {
                    buf.resize(tag.offset as usize, 0);
                }
                
                // Append the tag data
                buf.extend_from_slice(tag.tag.as_slice());
            } // else do nothing, tag data is shared and already written.``
        }
        
        // All Tags written to buf, add padding if needed if the last tag does not end on a 4-byte boundary.
        buf.extend(vec![0u8; crate::pad_size(buf.len())]);
        
        // Update profile size
        let length = buf.len() as u32;
        buf[0..4].copy_from_slice(&length.to_be_bytes());
        
        Ok(buf)
    }

    /// Serializes the ICC profile to a String (best-effort for debugging; lossy for non-UTF-8).
    pub fn into_string(self) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = self.into_bytes()?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    
    // In a RawProfile, as defined in this library, the tag table information is embeded
    // in the `tags` field, which contains the offsets and sizes of the tags, as well as the
    // tag data itself.
    // The offsets and sizes are copied from the tag table when reading a binary profile from file
    // but have to be changed if the tag data changes, for example when adding or removing tags,
    // or when changing the tag data.
    // This method updates the offsets and sizes of the tags based on their data, ensuring that
    // the profile can be written back to a file with correct offsets and sizes.
    fn with_updated_tagrecord_offsets_and_sizes(mut self) -> Self {
        
        // TODO: TagType content sharing.
        // Go through the tags and see which ones have the same content, and share them.
        // This is a performance optimization to reduce the size of the profile.
        // This is not required by the ICC specification, but it can reduce the size of the profile
        // and improve performance when reading and writing profiles.
        // Indicate this by making the offset a negative value, which means that the tag data is shared
        // and the offset for next tag is not updated.
        // - create a hashmap of tag data to offset, and use that offset if a duplication is found
        
        let mut shared_location: HashMap<&[u8], (u32, i32)> = HashMap::new();

        
        // Calculate start of tag data area
        let tag_count = self.tags.len();
        let data_start = 128 + 4 + (tag_count * 12);

        // Position for next tag data (aligned to 4 bytes)
        let mut offset_for_next_tag = crate::padded_size(data_start);
        
        // Process each tag
        for (_signature, tag_record) in self.tags.iter_mut(){
            (tag_record.offset, tag_record.size) = if let Some(&(offset, size)) = shared_location.get(&tag_record.tag.data().as_slice()) {
                // If the tag data is shared, use the existing offset and a negative size
                // to indicate that the tag data is shared, and doesn't need to be written again.
                // This is a performance optimization to reduce the size of the profile.
                // We need the size information, as shared tags are still needed to write the tag table.
                (offset, -size)
            } else { // not shared
                let offset = offset_for_next_tag as u32;
                let size = tag_record.tag.len() as i32;
                shared_location.insert(tag_record.tag.data().as_slice(), (offset, size));    
                // Otherwise, use the current offset and size
                offset_for_next_tag += crate::padded_size(tag_record.tag.len());
                (offset, size)
            };
            
         //   tag_record.offset = offset_for_next_tag as u32;
        //    tag_record.size = tag_record.tag.len() as u32;
        } 
        dbg!(&self);
        
        self
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
