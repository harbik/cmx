// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//use isocountry::CountryCode;
//use isolang::Language;
use zerocopy::{BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, U32};

use crate::tag::tagdata::MultiLocalizedUnicodeData;
use serde::Serialize;
use std::collections::BTreeMap;

/// Structured representation of MultiLocalizedUnicodeType for serialization/deserialization
/// Uses a map of locale codes to strings for easy access.
/// Example keys: "en", "de", "fr", "en-US", "de-DE"
/// Values are the corresponding localized strings.
#[derive(Serialize)]
pub struct MultiLocalizedUnicodeType {
    #[serde(flatten)]
    entries: BTreeMap<String, String>,
}

impl MultiLocalizedUnicodeType {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
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

impl From<&super::MultiLocalizedUnicodeData> for MultiLocalizedUnicodeType {
    fn from(mluc_data: &super::MultiLocalizedUnicodeData) -> Self {
        {
            let data = &mluc_data.0;

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
                        .to_ascii_lowercase();
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

                // Convert UTF-16BE bytes to Rust String
                let value = String::from_utf16(
                    &value_bytes
                        .chunks(2)
                        .map(|x| u16::from_be_bytes([x[0], x[1]]))
                        .collect::<Vec<u16>>(),
                )
                .unwrap();

                // Add the entry to the map
                mluc.entries.insert(key, value);
            }
            mluc
        }
    }
}

impl From<&MultiLocalizedUnicodeType> for MultiLocalizedUnicodeData {
    /// Convert the structured MultiLocalizedUnicodeType back into the binary MultiLocalizedUnicodeData format
    fn from(mluc_type: &MultiLocalizedUnicodeType) -> Self {
        let mut mluc_data: Vec<u8> = Vec::new();
        let n = mluc_type.entries.len() as u32;
        if n == 0 {
            mluc_data.extend(HeaderLayout::new(0).as_bytes());
            return MultiLocalizedUnicodeData(mluc_data);
        }

        // Header
        mluc_data.extend(HeaderLayout::new(n).as_bytes());

        // Prepare record table and data block
        let mut offset = 16 + n * 12; // header + records
        let mut data_block: Vec<u8> = Vec::new();

        for (key, value) in &mluc_type.entries {
            // Split locale key "ll" or "ll-cc"
            let parts: Vec<&str> = key.split('-').collect();
            let language = parts[0].to_ascii_lowercase();
            let country = if parts.len() > 1 {
                parts[1].to_ascii_lowercase()
            } else {
                String::new()
            };

            // Encode value as UTF-16BE
            let utf16_be: Vec<u8> = value.encode_utf16().flat_map(|u| u.to_be_bytes()).collect();
            let len_bytes = utf16_be.len() as u32;

            // Build record
            let entry = MultiLocalizedUnicodeRecord {
                language: language.as_bytes().try_into().unwrap_or([0; 2]),
                country: if country.is_empty() {
                    [0; 2]
                } else {
                    country.as_bytes().try_into().unwrap_or([0; 2])
                },
                length: U32::new(len_bytes),
                offset: U32::new(offset),
            };

            // Update offset and accumulate data
            offset += len_bytes;
            data_block.extend_from_slice(&utf16_be);

            // Append record to table
            mluc_data.extend(entry.as_bytes());
        }

        // Append all strings
        mluc_data.extend_from_slice(&data_block);
        MultiLocalizedUnicodeData(mluc_data)
    }
}

impl MultiLocalizedUnicodeData {
    pub fn clear(&mut self) {
        let header = HeaderLayout::new(0);
        self.0 = header.as_bytes().to_vec();
    }

    pub fn append(&mut self, language: &str, country: Option<&str>, value: &str) {
        let mut mluc_type: MultiLocalizedUnicodeType = (&*self).into();
        // Normalize key to lower-case for both parts
        let key = if let Some(c) = country {
            let lang = language
                .chars()
                .take(2)
                .collect::<String>()
                .to_ascii_lowercase();
            let ctry = c.chars().take(2).collect::<String>().to_ascii_lowercase();
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
        mluc_data.append("en", Some("US"), "Hello");
        mluc_data.append("de", Some("de"), "Hallo");
        mluc_data.append("fr", None, "Bonjour");

        // Convert to structured type
        let mluc_type: MultiLocalizedUnicodeType = (&mluc_data).into();

        // Check entries
        assert_eq!(mluc_type.entries.get("en-us").unwrap(), "Hello");
        assert_eq!(mluc_type.entries.get("de-de").unwrap(), "Hallo");
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
                mluc.append("en", Some("US"), "© 2024 Example Corp.");
                mluc.append("de", Some("DE"), "© 2024 Beispiel GmbH");
                mluc.append("du", None, "© 2024 Voorbeeld BV");
            });
        print!("{profile}");
        let mluc_data = profile.0.tag_data(tags::CopyrightTag).unwrap();
        if let crate::tag::tagdata::TagData::MultiLocalizedUnicode(mluc) = mluc_data {
            let mluc_type: MultiLocalizedUnicodeType = mluc.into();
            assert_eq!(
                mluc_type.entries.get("en-us").unwrap(),
                "© 2024 Example Corp."
            );
            assert_eq!(
                mluc_type.entries.get("de-de").unwrap(),
                "© 2024 Beispiel GmbH"
            );
            assert_eq!(mluc_type.entries.get("du").unwrap(), "© 2024 Voorbeeld BV");
        } else {
            panic!("Expected MultiLocalizedUnicode tag data");
        }
    }
}
