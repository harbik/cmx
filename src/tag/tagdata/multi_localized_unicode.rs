// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use zerocopy::{BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, U32};

use crate::tag::tagdata::MultiLocalizedUnicodeData;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, HashSet};

/// Structured representation of MultiLocalizedUnicodeType for serialization/deserialization
/// Uses a map of locale codes to strings for easy access.
/// Example keys: "en", "de", "fr", "en-US", "de-DE"
/// Values are the corresponding localized strings.
#[derive(Serialize)]
pub struct MultiLocalizedUnicodeType {
    #[serde(flatten)]
    entries: BTreeMap<String, String>,
    shared_data: bool,
}

pub type MlucData = (
    Vec<([u8; 2], [u8; 2], U32<BigEndian>, U32<BigEndian>)>,
    Vec<u8>,
);

impl MultiLocalizedUnicodeType {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            shared_data: true,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Convert the entries map into a Vec of (language, country, utf16be_data)
    /// language and country are [u8;2] arrays
    /// utf16be_data is a Vec<u8> containing the UTF-16BE encoded string
    /// The order of entries in the Vec is the same as the order in the BTreeMap (i.e. sorted by key)
    /// This is an intermediate step before constructing the final binary data.
    pub fn to_icc_data(&self) -> Vec<([u8; 2], [u8; 2], Vec<u8>)> {
        let mut icc_vec = Vec::new();
        for (key, value) in &self.entries {
            // Split locale key "ll" or "ll-cc"
            let parts: Vec<&str> = key.split('-').collect();

            // Normalize to lower-case and convert to [u8;2]
            let language = parts[0]
                .to_ascii_lowercase()
                .as_bytes()
                .try_into()
                .unwrap_or([0; 2]);

            // Convert country code if present, else [0;2]
            let country = if parts.len() <= 1 {
                [0; 2]
            } else {
                parts[1]
                    .to_ascii_lowercase()
                    .as_bytes()
                    .try_into()
                    .unwrap_or([0; 2])
            };

            // Encode value as UTF-16BE
            let utf16_be: Vec<u8> = value.encode_utf16().flat_map(|u| u.to_be_bytes()).collect();

            icc_vec.push((language, country, utf16_be))
        }
        icc_vec
    }

    /// Build the MLUC records table and the contiguous UTF‑16BE data block.
    ///
    /// # Returns
    /// A tuple `(records_table, string_data)` where:
    /// - `records_table`: `Vec<([u8; 2], [u8; 2], U32<BigEndian>, U32<BigEndian>)>` — for each entry,
    ///   `(language, country, offset, length)`;
    ///   - `language` / `country` are 2‑byte ASCII codes (lowercase) per ICC MLUC.
    ///   - `offset` is the byte offset **from the start of the MLUC tag** to the start of the string.
    ///   - `length` is the string length in **bytes** (UTF‑16BE), not code units or chars.
    /// - `string_data`: `Vec<u8>` — all strings concatenated in the exact order implied below.
    ///
    /// # Layout & invariants
    /// - MLUC header is 16 bytes; each record is 12 bytes. Offsets are computed as
    ///   `16 + records.len() * 12 + intra_data_offset`.
    /// - When `shared_data == true`, identical strings are **deduplicated** and point to the
    ///   same `offset`. When `false`, each record owns a private copy.
    /// - Iteration order is that of the internal `BTreeMap`, so output is deterministic.
    /// - Strings are encoded UTF‑16BE and thus have even byte lengths.
    ///
    /// # Notes
    /// - Offsets and lengths are serialized as big‑endian `U32` per ICC.
    /// - The returned buffers are ready to be appended after writing the MLUC header.
    ///
    /// # Example
    /// ```ignore
    /// let (records, data) = mluc.records_table_and_data();
    /// // write header(16), then `records` table (12 bytes each), then `data`.
    /// ```
    pub fn records_table_and_data(&self) -> MlucData {
        let n = self.entries.len();
        let start = 16 + n * 12;
        let icc_data = self.to_icc_data();
        let mut strings = Vec::new();
        if self.shared_data {
            // Deduplicate by string contents and assign the first-seen offset.
            let mut offset_map: HashMap<&[u8], usize> = HashMap::with_capacity(icc_data.len());
            let mut next = start;
            for (_, _, data) in &icc_data {
                let key = data.as_slice();
                if !offset_map.contains_key(key) {
                    offset_map.insert(key, next);
                    strings.extend_from_slice(data);
                    next += data.len();
                }
            }
            (
                icc_data
                    .iter()
                    .map(|(lang, ctry, data)| {
                        let off = *offset_map.get(data.as_slice()).unwrap();
                        (
                            *lang,
                            *ctry,
                            U32::<BigEndian>::new(off as u32),
                            U32::<BigEndian>::new(data.len() as u32),
                        )
                    })
                    .collect(),
                strings,
            )
        } else {
            // No sharing: each record gets a unique, sequential offset.
            let mut out = Vec::with_capacity(icc_data.len());
            let mut next = start;
            for (lang, ctry, data) in &icc_data {
                out.push((
                    *lang,
                    *ctry,
                    U32::new(next as u32),
                    U32::<BigEndian>::new(data.len() as u32),
                ));
                strings.extend_from_slice(data);
                next += data.len();
            }
            (out, strings)
        }
    }
}

impl Default for MultiLocalizedUnicodeType {
    fn default() -> Self {
        Self::new()
    }
}

// Layouts for parsing and constructing MultiLocalizedUnicodeData
/// Zerocopy layout for the MultiLocalizedUnicodeData header
#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct HeaderLayout {
    type_signature: U32<BigEndian>,
    reserved: [u8; 4],
    number_of_records: U32<BigEndian>,
    record_size: U32<BigEndian>, // always 12
}

impl HeaderLayout {
    #[allow(dead_code)]
    /// Create a new header with the given number of records
    pub fn new(n: u32) -> Self {
        let sig: u32 = super::DataSignature::MultiLocalizedUnicodeData.to_u32();
        Self {
            type_signature: U32::new(sig),
            reserved: [0; 4],
            number_of_records: U32::new(n),
            record_size: U32::new(12), // size of MultiLocalizedUnicodeRecord
        }
    }
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct RecordsLayout {
    records: [MultiLocalizedUnicodeRecord],
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct MultiLocalizedUnicodeRecord {
    language: [u8; 2],
    country: [u8; 2],
    length: U32<BigEndian>, //  length in bytes of the string
    offset: U32<BigEndian>, // offset in bytes from the start of the MultiLocalizedUnicode tag to the start of the string
}

/// Parse binary `MultiLocalizedUnicodeData` into structured `MultiLocalizedUnicodeType`.
/// - Reads the 16‑byte MLUC header, then the records table (12 bytes per record),
///   then the contiguous UTF‑16BE string data.
/// - Constructs the `BTreeMap<String, String>` of locale codes to strings.
/// - Normalizes locale keys to lower-case (e.g. "en-us").
impl From<&super::MultiLocalizedUnicodeData> for MultiLocalizedUnicodeType {
    fn from(mluc_data: &super::MultiLocalizedUnicodeData) -> Self {
        {
            let data = &mluc_data.0;
            let mut total_string_length = 0;
            let mut unique_strings = HashSet::new();

            if data.len() < 16 {
                // create an empty MultiLocalizedUnicodeType, as there is no valid data
                return MultiLocalizedUnicodeType::new();
            }

            // Parse the header data
            let header = HeaderLayout::try_ref_from_bytes(&data[..16]).unwrap();
            let n = header.number_of_records.get() as usize;

            // Parse the records table
            let record_size = header.record_size.get() as usize;
            let table_end = 16 + n * record_size;
            let table = RecordsLayout::try_ref_from_bytes(&data[16..table_end]).unwrap();

            let mut mluc = MultiLocalizedUnicodeType::new();

            // Iterate over records and extract strings
            for r in &table.records {
                // Parse language and country codes
                let language = r.language;
                let country = r.country;

                // Normalize locale key to lower-case (e.g. "en-us")
                let key = if country != [0; 2] {
                    let lang_str = std::str::from_utf8(&language)
                        .unwrap_or_default()
                        .to_ascii_lowercase();
                    let country_str = std::str::from_utf8(&country)
                        .unwrap_or_default()
                        .to_ascii_uppercase();
                    format!("{lang_str}-{country_str}")
                } else {
                    std::str::from_utf8(&language)
                        .unwrap_or_default()
                        .to_ascii_lowercase()
                };

                // Extract the UTF-16BE string
                let offset = r.offset.get() as usize;
                let length = r.length.get() as usize;
                let value_bytes = &data[offset..offset + length];
                total_string_length += length;

                // Convert UTF-16BE bytes to Rust String
                let value = String::from_utf16(
                    &value_bytes
                        .chunks(2)
                        .map(|x| u16::from_be_bytes([x[0], x[1]]))
                        .collect::<Vec<u16>>(),
                )
                .unwrap();

                // Add the entry to the map
                unique_strings.insert(value.clone());
                mluc.entries.insert(key, value);
            }

            let could_share = unique_strings.len() < mluc.entries.len();
            let is_shared = data.len() < total_string_length + 16 + n * 12;
            if could_share && !is_shared {
                // Data is not shared, but could be
                // Maintain the status of the input data
                mluc.shared_data = false;
            } else {
                mluc.shared_data = true;
            }
            mluc
        }
    }
}

impl From<&MultiLocalizedUnicodeType> for MultiLocalizedUnicodeData {
    /// Serialize a structured `MultiLocalizedUnicodeType` into binary `MultiLocalizedUnicodeData`.
    ///
    /// Writes the 16‑byte MLUC header, then the records table (12 bytes per record),
    /// then the contiguous UTF‑16BE string data produced by [`records_table_and_data`].
    ///
    /// Invariants:
    /// - Offsets/lengths are `U32<BigEndian>` per ICC.
    /// - Offsets are **from the start of the MLUC tag** (header included).
    /// - Record order follows the internal `BTreeMap`, giving deterministic output.
    /// - `string_data` is concatenated verbatim; when `shared_data == true`, identical strings share offsets.
    fn from(mluc_type: &MultiLocalizedUnicodeType) -> Self {
        let mut mluc_data: Vec<u8> = Vec::new();
        let n = mluc_type.entries.len() as u32;
        // Fast path: If no records, just write the header and return.
        if n == 0 {
            mluc_data.reserve(16);
            mluc_data.extend(HeaderLayout::new(0).as_bytes());
            return MultiLocalizedUnicodeData(mluc_data);
        }

        // get data for records and strings
        let (records_table, string_data) = mluc_type.records_table_and_data();
        let string_data_len = string_data.len();
        mluc_data.reserve(16 + (n as usize) * 12 + string_data_len);

        // Header
        mluc_data.extend(HeaderLayout::new(n).as_bytes());

        // Each record: language[2], country[2], offset[4], length[4] (all big-endian, per ICC spec)
        for (lang, ctry, offset, len) in records_table {
            mluc_data.extend(lang);
            mluc_data.extend(ctry);
            mluc_data.extend(len.as_bytes());
            mluc_data.extend(offset.as_bytes());
        }
        mluc_data.extend(string_data);

        // Validate that the final length matches the expected MLUC size:
        // header (16) + 12*n records + string_data.len()
        debug_assert!(
            mluc_data.len() == 16 + (n as usize) * 12 + string_data_len,
            "MLUC serialization: buffer size mismatch (expected {}, got {})",
            16 + (n as usize) * 12 + string_data_len,
            mluc_data.len()
        );

        MultiLocalizedUnicodeData(mluc_data)
    }
}

impl MultiLocalizedUnicodeData {
    pub fn clear(&mut self) {
        let header = HeaderLayout::new(0);
        self.0 = header.as_bytes().to_vec();
    }

    pub fn insert(&mut self, language: &str, country: Option<&str>, value: &str) {
        let mut mluc_type: MultiLocalizedUnicodeType = (&*self).into();
        // Normalize key to lower-case for language and upper-case for country
        let key = if let Some(c) = country {
            let lang = language
                .chars()
                .take(2)
                .collect::<String>()
                .to_ascii_lowercase();
            let ctry = c.chars().take(2).collect::<String>().to_ascii_uppercase();
            format!("{lang}-{ctry}")
        } else {
            language
                .chars()
                .take(2)
                .collect::<String>()
                .to_ascii_lowercase()
        };
        mluc_type.entries.insert(key, value.to_string());
        *self = MultiLocalizedUnicodeData::from(&mluc_type);
    }
}

#[cfg(test)]
mod mluc_test {

    use crate::{profile::DisplayProfile, tag::tags};

    use super::*;

    #[test]
    fn test_mluc_entries() {
        let mut mluc_data = MultiLocalizedUnicodeData(Vec::new());

        // Append some entries
        mluc_data.insert("en", Some("US"), "Hello");
        mluc_data.insert("de", Some("de"), "Hallo");
        mluc_data.insert("fr", None, "Bonjour");

        // Convert to structured type
        let mluc_type: MultiLocalizedUnicodeType = (&mluc_data).into();

        // Check entries
        assert_eq!(mluc_type.entries.get("en-US").unwrap(), "Hello");
        assert_eq!(mluc_type.entries.get("de-DE").unwrap(), "Hallo");
        assert_eq!(mluc_type.entries.get("fr").unwrap(), "Bonjour");

        // Clear entries
        mluc_data.clear();
        let mluc_type: MultiLocalizedUnicodeType = (&mluc_data).into();
        assert!(mluc_type.is_empty());
    }

    #[test]
    fn test_mluc_add_profile() {
        let profile = DisplayProfile::cmx_srgb(crate::tag::RenderingIntent::RelativeColorimetric)
            .with_tag(tags::CopyrightTag)
            .as_multi_localized_unicode(|mluc| {
                mluc.insert("du", Some("BE"), "© 2024 Voorbeeld BV");
                mluc.insert("du", Some("NL"), "© 2024 Voorbeeld BV");
                mluc.insert("en", Some("US"), "© 2024 Example Corp.");
                mluc.insert("de", Some("DE"), "© 2024 Beispiel GmbH");
            });
        print!("{profile}");
        let mluc_data = profile.0.tag_data(tags::CopyrightTag).unwrap();
        if let crate::tag::tagdata::TagData::MultiLocalizedUnicode(mluc) = mluc_data {
            let mluc_type: MultiLocalizedUnicodeType = mluc.into();
            assert!(mluc_type.shared_data);
            assert_eq!(
                mluc_type.entries.get("en-US").unwrap(),
                "© 2024 Example Corp."
            );
            assert_eq!(
                mluc_type.entries.get("de-DE").unwrap(),
                "© 2024 Beispiel GmbH"
            );
            assert_eq!(
                mluc_type.entries.get("du-BE").unwrap(),
                "© 2024 Voorbeeld BV"
            );
            assert_eq!(
                mluc_type.entries.get("du-NL").unwrap(),
                "© 2024 Voorbeeld BV"
            );
        } else {
            panic!("Expected MultiLocalizedUnicode tag data");
        }
    }
}
