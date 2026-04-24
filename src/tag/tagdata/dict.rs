// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! ICC `dictType` tag data (ICC.1:2022 §10.12).
//!
//! `dictType` stores an ordered sequence of Unicode key/value string pairs.  It is used by the
//! `meta` tag to embed arbitrary metadata in a profile (e.g. display-calibration provenance,
//! manufacturer identifiers, or EDID fields).
//!
//! ## Binary layout
//!
//! ```text
//! Bytes  0– 3  type signature  'dict' (0x64696374)
//! Bytes  4– 7  reserved        0
//! Bytes  8–11  N               number of records
//! Bytes 12–15  record size     16 (key + value only) or 24 (+ display name offset/length)
//!
//! Per record (16 bytes):
//!   Bytes 0–3   key offset    (from start of tag data)
//!   Bytes 4–7   key length    (in bytes of UTF-16BE)
//!   Bytes 8–11  value offset
//!   Bytes 12–15 value length
//! ```
//!
//! Keys and values are encoded as **UTF-16BE** with no BOM.  This implementation reads both
//! 16-byte (key+value) and 24-byte (key+value+display-name) record sizes; the display-name
//! field is silently skipped on read (it references an embedded `mluc` sub-structure that
//! is rarely populated in practice).  The builder always writes 16-byte records.

use indexmap::IndexMap;
use serde::Serialize;
use zerocopy::{BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, U32};

use crate::tag::tagdata::DictData;

// ── Binary layout structs ────────────────────────────────────────────────────

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct Header {
    type_signature: U32<BigEndian>,
    reserved: [u8; 4],
    num_records: U32<BigEndian>,
    record_size: U32<BigEndian>,
}

/// A single 16-byte dict record (key + value offsets/lengths).
#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy)]
#[repr(C, packed)]
struct Record {
    key_offset: U32<BigEndian>,
    key_length: U32<BigEndian>,
    value_offset: U32<BigEndian>,
    value_length: U32<BigEndian>,
}

// ── Parsed / serialisable representation ────────────────────────────────────

/// A parsed ICC `dictType` tag: an ordered map of Unicode key/value string pairs.
///
/// Serialises to TOML as a flat map, e.g.:
///
/// ```toml
/// [meta]
/// "CMF_version" = "3.8.8.0"
/// "CMF_product" = "DisplayCAL"
/// "ACCURACY_dE76_avg" = "0.5972"
/// ```
#[derive(Serialize)]
pub struct DictType {
    #[serde(flatten)]
    entries: IndexMap<String, String>,
}

// ── Parser: DictData → DictType ─────────────────────────────────────────────

/// Decode a UTF-16BE byte slice into a `String`.
///
/// Returns `None` if `bytes` has an odd length, which would indicate corrupt tag
/// data (all UTF-16BE strings must have an even byte count).
fn decode_utf16be(bytes: &[u8]) -> Option<String> {
    if bytes.len() % 2 != 0 {
        return None;
    }
    let words: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .collect();
    Some(String::from_utf16_lossy(&words))
}

impl From<&DictData> for DictType {
    fn from(data: &DictData) -> Self {
        let bytes = &data.0;
        let mut entries = IndexMap::new();

        // Need at least the 16-byte header.
        let Ok(header) = Header::try_ref_from_bytes(bytes.get(..16).unwrap_or(&[])) else {
            return DictType { entries };
        };

        let n = header.num_records.get() as usize;
        let rec_size = header.record_size.get() as usize;

        // Record size must be 16 or 24; anything else is malformed.
        if rec_size != 16 && rec_size != 24 {
            return DictType { entries };
        }

        let table_start = 16usize;
        let table_end = table_start.saturating_add(n.saturating_mul(rec_size));
        if table_end > bytes.len() {
            return DictType { entries };
        }

        for i in 0..n {
            let rec_start = table_start + i * rec_size;
            let rec_end = rec_start + 16; // we only need the first 16 bytes (key+value)
            let Ok(rec) = Record::try_ref_from_bytes(&bytes[rec_start..rec_end]) else {
                continue;
            };

            let key_off = rec.key_offset.get() as usize;
            let key_len = rec.key_length.get() as usize;
            let val_off = rec.value_offset.get() as usize;
            let val_len = rec.value_length.get() as usize;

            // Bounds-check before slicing.
            let Some(key_end) = key_off.checked_add(key_len) else {
                continue;
            };
            let Some(val_end) = val_off.checked_add(val_len) else {
                continue;
            };
            if key_end > bytes.len() || val_end > bytes.len() {
                continue;
            }

            // Skip this record if either string has an odd byte count (corrupt data).
            let (Some(key), Some(value)) = (
                decode_utf16be(&bytes[key_off..key_end]),
                decode_utf16be(&bytes[val_off..val_end]),
            ) else {
                continue;
            };
            entries.insert(key, value);
        }

        DictType { entries }
    }
}

// ── Serialiser: DictType → DictData ─────────────────────────────────────────

impl From<&DictType> for DictData {
    fn from(dict: &DictType) -> Self {
        let n = dict.entries.len();

        // Encode all keys and values as UTF-16BE first so we know the offsets.
        let mut encoded: Vec<(Vec<u8>, Vec<u8>)> = Vec::with_capacity(n);
        for (k, v) in &dict.entries {
            let key_bytes: Vec<u8> = k.encode_utf16().flat_map(|c| c.to_be_bytes()).collect();
            let val_bytes: Vec<u8> = v.encode_utf16().flat_map(|c| c.to_be_bytes()).collect();
            encoded.push((key_bytes, val_bytes));
        }

        // Layout:  16-byte header + n×16-byte records + string data
        const HEADER_SIZE: usize = 16;
        const RECORD_SIZE: usize = 16;
        let string_data_start = HEADER_SIZE + n * RECORD_SIZE;

        // Compute record entries (offsets into the final buffer).
        let mut records: Vec<Record> = Vec::with_capacity(n);
        let mut cursor = string_data_start;
        for (key_bytes, val_bytes) in &encoded {
            let val_offset = cursor + key_bytes.len();
            debug_assert!(cursor <= u32::MAX as usize, "dictType key offset overflows u32");
            debug_assert!(val_offset <= u32::MAX as usize, "dictType value offset overflows u32");
            debug_assert!(key_bytes.len() <= u32::MAX as usize, "dictType key length overflows u32");
            debug_assert!(val_bytes.len() <= u32::MAX as usize, "dictType value length overflows u32");
            records.push(Record {
                key_offset: U32::new(cursor as u32),
                key_length: U32::new(key_bytes.len() as u32),
                value_offset: U32::new(val_offset as u32),
                value_length: U32::new(val_bytes.len() as u32),
            });
            cursor += key_bytes.len() + val_bytes.len();
        }

        // Build the buffer.
        let total = cursor;
        let mut buf = Vec::with_capacity(total);

        let header = Header {
            type_signature: U32::new(super::DataSignature::DictData.to_u32()),
            reserved: [0; 4],
            num_records: U32::new(n as u32),
            record_size: U32::new(RECORD_SIZE as u32),
        };
        buf.extend_from_slice(header.as_bytes());

        for rec in &records {
            buf.extend_from_slice(rec.as_bytes());
        }
        for (key_bytes, val_bytes) in &encoded {
            buf.extend_from_slice(key_bytes);
            buf.extend_from_slice(val_bytes);
        }

        debug_assert_eq!(buf.len(), total, "dictType serialisation: buffer size mismatch");
        DictData(buf)
    }
}

// ── Builder methods on DictData ──────────────────────────────────────────────

impl DictData {
    /// Remove all entries, leaving an empty `dictType` payload.
    pub fn clear(&mut self) {
        *self = DictData::from(&DictType {
            entries: IndexMap::new(),
        });
    }

    /// Insert or overwrite a key/value pair.
    ///
    /// Both `key` and `value` may contain any Unicode text; they are stored as
    /// UTF-16BE in the binary payload.  Insertion order is preserved.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cmx::profile::DisplayProfile;
    /// use cmx::tag::tags::MetadataTag;
    ///
    /// let profile = DisplayProfile::new()
    ///     .with_tag(MetadataTag)
    ///     .as_dict(|d| {
    ///         d.insert("CMF_product", "MyProfileBuilder");
    ///         d.insert("CMF_version", "1.0.0");
    ///     });
    ///
    /// let bytes = profile.to_bytes().unwrap();
    /// assert!(bytes.len() > 128);
    /// ```
    pub fn insert(&mut self, key: &str, value: &str) {
        let mut dict: DictType = (&*self).into();
        dict.entries.insert(key.to_string(), value.to_string());
        *self = DictData::from(&dict);
    }

    /// Remove the entry with the given key, if present.
    pub fn remove(&mut self, key: &str) {
        let mut dict: DictType = (&*self).into();
        dict.entries.shift_remove(key);
        *self = DictData::from(&dict);
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_empty() {
        let mut d = DictData(Vec::new());
        d.clear();
        let parsed = DictType::from(&d);
        assert!(parsed.entries.is_empty());
        let back = DictData::from(&parsed);
        assert_eq!(DictType::from(&back).entries, parsed.entries);
    }

    #[test]
    fn roundtrip_entries() {
        let mut d = DictData(Vec::new());
        d.insert("CMF_product", "DisplayCAL");
        d.insert("CMF_version", "3.8.8.0");
        d.insert("unicode_key_\u{00e9}", "caf\u{00e9}");

        let parsed = DictType::from(&d);
        assert_eq!(parsed.entries["CMF_product"], "DisplayCAL");
        assert_eq!(parsed.entries["CMF_version"], "3.8.8.0");
        assert_eq!(parsed.entries["unicode_key_\u{00e9}"], "caf\u{00e9}");

        // Re-serialise and parse again — must be identical.
        let back = DictData::from(&parsed);
        let reparsed = DictType::from(&back);
        assert_eq!(reparsed.entries, parsed.entries);
    }

    #[test]
    fn insert_preserves_order() {
        let mut d = DictData(Vec::new());
        for i in 0..5u8 {
            d.insert(&format!("key{i}"), &format!("val{i}"));
        }
        let parsed = DictType::from(&d);
        let keys: Vec<&str> = parsed.entries.keys().map(String::as_str).collect();
        assert_eq!(keys, ["key0", "key1", "key2", "key3", "key4"]);
    }

    #[test]
    fn remove_entry() {
        let mut d = DictData(Vec::new());
        d.insert("a", "1");
        d.insert("b", "2");
        d.insert("c", "3");
        d.remove("b");
        let parsed = DictType::from(&d);
        assert!(!parsed.entries.contains_key("b"));
        assert_eq!(parsed.entries.len(), 2);
    }

    #[test]
    fn malformed_data_returns_empty() {
        // Too short to contain a header.
        let d = DictData(vec![0x64, 0x69, 0x63, 0x74, 0x00]);
        let parsed = DictType::from(&d);
        assert!(parsed.entries.is_empty());
    }

    #[test]
    fn odd_length_string_record_is_skipped() {
        // Build a valid 1-record dict, then corrupt the key length to be odd (3 instead of 4).
        let mut d = DictData(Vec::new());
        d.insert("ab", "cd"); // key = 4 bytes UTF-16BE, value = 4 bytes UTF-16BE

        // key_length is at bytes 20–23 of the payload (header=16, rec[0].key_length offset=4).
        // Force it to 3 (odd) to simulate corrupt tag data.
        d.0[20] = 0x00;
        d.0[21] = 0x00;
        d.0[22] = 0x00;
        d.0[23] = 0x03;

        let parsed = DictType::from(&d);
        // The corrupted record must be silently skipped.
        assert!(parsed.entries.is_empty());
    }
}
