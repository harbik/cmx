
mod raw_profile;
pub use raw_profile::RawProfile;

mod input_profile;
pub use input_profile::InputProfile;

mod display_profile;
pub use display_profile::DisplayProfile;

mod output_profile;
pub use output_profile::OutputProfile;

mod device_link_profile;
pub use device_link_profile::DeviceLinkProfile;

mod abstract_profile;
pub use abstract_profile::AbstractProfile;

mod color_space_profile;
pub use color_space_profile::ColorSpaceProfile;

mod named_color_profile;
pub use named_color_profile::NamedColorProfile;

mod spectral_profile;
pub use spectral_profile::SpectralProfile;

/// delegates methods from the RawProfile to all Profiles.
mod delegate;

mod with_tag;

mod tag_setter;
pub use tag_setter::TagSetter;

mod checksum;
pub use {checksum::set_profile_id,  checksum::md5checksum};

#[derive(Debug)]
pub enum Profile {
    Input(InputProfile),
    Display(DisplayProfile),
    Output(OutputProfile),
    DeviceLink(DeviceLinkProfile),
    Abstract(AbstractProfile),
    ColorSpace(ColorSpaceProfile),
    NamedColor(NamedColorProfile),
    Spectral(SpectralProfile),
    Raw(RawProfile),
}


/*
/// Represents a single ICC tag (raw).
#[derive(Debug, Clone, Serialize)]
pub struct DataBlock {
    pub offset: u32,
    pub size: u32,
    pub data: Vec<u8>,
}

pub struct Data {
    pub type_signature: TypeSignature,
    pub data: Vec<u8>,
}

/// An ICC profile, deconstructed in:
/// 
/// - a raw header array, with a length of 128 bytes,
/// - a indexmap of datablocks, with a ProfileTag as key and DataBlock as value.
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

#[derive(Debug, Clone, Serialize)]
pub struct RawProfile {
    #[serde(with = "serde_arrays")]
    pub header: [ u8; 128 ], // 128 bytes
    pub tags: IndexMap<ProfileTag, DataBlock>, // preserves insertion order
    pub padding: usize, // number of padding bytes found in a profile read
}


impl RawProfile {
    /// Reads an ICC profile from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Self::from_bytes(&buf)
    }

    /// Reads an ICC profile from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the header (first 128 bytes)
      //  let mut header = vec![0u8; 128];
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

        let mut total_data_size = 0;
        for _ in 0..tag_count {
            let mut entry = [0u8; 12];
            cursor.read_exact(&mut entry)?;

            // Parse the tag entry
            let signature_value = u32::from_be_bytes([entry[0], entry[1], entry[2], entry[3]]);
            let signature = ProfileTag::new(signature_value);

            // Convert the offset and size from big-endian to u32
            let offset = u32::from_be_bytes([entry[4], entry[5], entry[6], entry[7]]);
            let size = u32::from_be_bytes([entry[8], entry[9], entry[10], entry[11]]);
            // Store the tag entry in the tag table
            tag_entries.push((signature, offset, size));
            total_data_size += size as usize;
        }

        // Create a map to hold the tags
        let mut tags = IndexMap::with_capacity(tag_count as usize);

        // Read each tag's data based on the offsets and sizes from the tag table
        // Note: The tag data is read from the original byte slice, not from the cursor.
        // This is because the tag data blocks are not contiguous in the file,
        // and we need to read them at their specified offsets.
        for (signature, offset, size) in &tag_entries {
            // Move the cursor to the tag's offset and read its data
            cursor.set_position(*offset as u64);
            let mut data = vec![0u8; *size as usize];
            cursor.read_exact(&mut data)?;

            tags.insert(*signature, DataBlock {
                offset: *offset,
                size: *size,
                data,
            });
        }

        // In the ICC profile format, each tag's data block starts at the offset specified in the tag table,
        // and the size is given in the tag table as well. The tag data block itself starts with a 4-byte type signature
        // and a 4-byte reserved field, followed by the actual tag data.

        let size =
            128 + // header size
            4 + // tag count byte size
            tag_count as usize * 20 +
            total_data_size;

        let padding = if bytes.len() > size {
            bytes.len() - size
        } else {
            0
        };
        Ok(RawProfile { header, tags, padding })
    }

    /// Reads an ICC profile from a string (as bytes).
    pub fn from_str(s: &str) -> Result<Self> {
        Self::from_bytes(s.as_bytes())
    }

    /// Writes the ICC profile to a file.
    pub fn to_file<P: AsRef<Path>>(self, path: P) -> Result<()> {
        let bytes = self.into_bytes()?;
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    /// Serializes the ICC profile to a Vec<u8>.
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        // Update offsets and sizes of tags based on their data
        let updated_self = self.with_updated_offsets_and_sizes();
        //let updated_self = self;
        let mut buf = Vec::new();
        buf.extend_from_slice(&updated_self.header);

        let tag_count = updated_self.tags.len() as u32;
        buf.extend_from_slice(&tag_count.to_be_bytes());

        // Write tag table using updated offsets and sizes
        for (sig, tag) in &updated_self.tags {
            buf.extend_from_slice(sig.to_u32().to_be_bytes().as_ref());
            buf.extend_from_slice(&tag.offset.to_be_bytes());
            buf.extend_from_slice(&tag.size.to_be_bytes());
        }

        // Find the end of the tag table
        let data_start = 128 + 4 + updated_self.tags.len() * 12;
        // Prepare a buffer large enough for all tag data
        let mut data_buf = vec![0u8; updated_self.tags.values().map(|t| (t.offset + t.size) as usize).max().unwrap_or(data_start)];

        // Copy tag data into the correct offsets
        for tag in updated_self.tags.values() {
            let start = tag.offset as usize;
            let end = start + tag.size as usize;
            data_buf[start..end].copy_from_slice(&tag.data);
        }
        buf.extend_from_slice(&data_buf[data_start..]);

        // copy any padding in the orginal profile, if present
        if updated_self.padding > 0 {
            buf.extend_from_slice(&vec![0u8; updated_self.padding as usize]);
        }
        Ok(buf)
    }

    /// Serializes the ICC profile to a String (lossless for binary data).
    pub fn into_string(self) -> Result<String> {
        let bytes = self.into_bytes()?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    /// Builds a tags table as a vector of an array of three u32 values:
    ///
    /// - The first u32 is the tag signature (as u32),
    /// - The second u32 is the offset of the tag data in the profile,
    /// - The third u32 is the size of the tag data.
    ///
    /// It only uses the data field
    pub fn with_updated_offsets_and_sizes(mut self) -> Self {
        // Build a BTreeMap to sort tags by their original offset
        let mut btree_map = BTreeMap::new();
        for (tag_signature, tag) in &self.tags {
            btree_map.insert(tag.offset, *tag_signature);
        }
        // Collect tag signatures in order of original offset
        let tag_signatures_by_data_order: Vec<ProfileTag> = btree_map.values().cloned().collect();

        let mut changed = false;
        for tag_signature in tag_signatures_by_data_order.clone() {
            if let Some(tag) = self.tags.get_mut(&tag_signature) {
                if tag.size != tag.data.len() as u32 {
                    changed = true;
                    break; 
                }
            }
        }

        // If no changes are needed, return the profile as is
        if !changed {
            return self;
        }

        // Calculate the starting offset for tag data (after header and tag table)
        let mut offset = 128 + 4 + self.tags.len() * 12;

        // For each tag (in data order), update its offset and size, and pad data to 4 bytes
        for tag_signature in tag_signatures_by_data_order {
            if let Some(tag) = self.tags.get_mut(&tag_signature) {
                let pad = (4 - (tag.data.len() % 4)) % 4;
                if tag.size != tag.data.len() as u32 {
                    if pad > 0 {
                        tag.data.extend(std::iter::repeat(0u8).take(pad));
                    }
                    tag.size = tag.data.len() as u32;
                }
                tag.offset = offset as u32;
                offset += tag.size as usize;
            }
        }
        self
    }


}


 */