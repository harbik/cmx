// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use isocountry::CountryCode;
use isolang::Language;
use zerocopy::{BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, U16, U32};

use crate::tag::tagdata::MultiLocalizedUnicodeData;
use serde::Serialize;

#[derive(Serialize)]
pub struct MultiLocalizedUnicodeEntry {
    pub language: Language,
    pub country: Option<CountryCode>,
    pub value: String,
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct MultiLocalizedUnicodeRecord {
    language: U16<BigEndian>, // First record language code: in accordance with the language code specified in ISO 639-1
    country: U16<BigEndian>, // First record country code: in accordance with the country code specified in ISO 3166-1
    length: U32<BigEndian>,  //  length in bytes of the string
    offset: U32<BigEndian>, // offset in bytes from the start of the MultiLocalizedUnicode tag to the start of the string
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct MultiLocalizedUnicodeHeaderLayout {
    type_signature: U32<BigEndian>,
    reserved: [u8; 4],
    number_of_records: U32<BigEndian>,
    record_size: U32<BigEndian>, // always 12
}
#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct MultiLocalizedUnicodeRecordsTableLayout {
    records: [MultiLocalizedUnicodeRecord],
}

//pub struct MultiLocalizedUnicodeData(pub Vec<u8>);

impl MultiLocalizedUnicodeData {
    pub fn entries(&self) -> Vec<MultiLocalizedUnicodeEntry> {
        let header = MultiLocalizedUnicodeHeaderLayout::try_ref_from_bytes(&self.0[..16]).unwrap();
        let n = header.number_of_records.get() as usize;
        let record_size = header.record_size.get() as usize;
        let table_end = 16 + n * record_size;
        let table =
            MultiLocalizedUnicodeRecordsTableLayout::try_ref_from_bytes(&self.0[16..table_end])
                .unwrap();
        let mut entries = Vec::with_capacity(n);
        for r in &table.records {
            let lang_code_bytes = r.language.get().to_be_bytes();
            let language =
                Language::from_639_1(std::str::from_utf8(&lang_code_bytes).unwrap()).unwrap();
            let country_code_bytes = r.country.get().to_be_bytes();
            let country =
                CountryCode::for_alpha2_caseless(std::str::from_utf8(&country_code_bytes).unwrap())
                    .ok();
            let offset = r.offset.get() as usize;
            let length = r.length.get() as usize;
            let value_bytes = &self.0[offset..offset + length];
            let value = String::from_utf16(
                &value_bytes
                    .chunks(2)
                    .map(|x| u16::from_be_bytes([x[0], x[1]]))
                    .collect::<Vec<u16>>(),
            )
            .unwrap();
            entries.push(MultiLocalizedUnicodeEntry {
                language,
                country,
                value,
            });
        }
        entries
    }

    #[allow(dead_code)]
    fn try_mut_from_bytes(&mut self) -> &mut MultiLocalizedUnicodeHeaderLayout {
        MultiLocalizedUnicodeHeaderLayout::try_mut_from_bytes(&mut self.0).unwrap()
    }
}

use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct MultiLocalizedUnicodeType {
    #[serde(flatten)]
    entries: BTreeMap<String, String>,
}

impl From<&super::MultiLocalizedUnicodeData> for MultiLocalizedUnicodeType {
    fn from(mluc: &super::MultiLocalizedUnicodeData) -> Self {
        let entries = mluc
            .entries()
            .into_iter()
            .map(|entry| {
                // Create keys like "en-US" or just "de" if no country
                let key = if let Some(country) = &entry.country {
                    format!(
                        "{}-{}",
                        entry.language.to_string().to_lowercase(),
                        country.alpha2()
                    )
                } else {
                    entry.language.to_string().to_lowercase()
                };

                // The value is just the string
                (key, entry.value)
            })
            .collect();

        Self { entries }
    }
}
