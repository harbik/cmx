// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! This module provides a fully parsed representation of an ICC `textDescriptionData` tag.
//! It includes methods to parse the tag from bytes and retrieve the ASCII and Unicode descriptions.
//!
//! # Notes
//! - The use of this type is deprecated in favor of the `multiLocalizedUnicodeData`.
//! - Only

use serde::Serialize;

use std::io::{Cursor, Read, Seek, SeekFrom};
use std::str;

/// A fully parsed representation of an ICC `textDescriptionData` tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TextDescriptionType {
    /// The 7-bit ASCII description.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ascii: String,
    /// The Unicode description.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub unicode: String,
    /// The Unicode language code for the Unicode description.
    #[serde(skip_serializing_if = "crate::is_zero")]
    pub unicode_language_code: u32,
    /// The ScriptCode code for the Macintosh script name.
    #[serde(skip_serializing_if = "crate::is_zero")]
    pub scriptcode_code: u16,
    /// The Macintosh script name.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub mac_script_name: String,
}

impl From<&super::TextDescriptionData> for TextDescriptionType {
    fn from(text_desc: &super::TextDescriptionData) -> Self {
        // Convert the raw bytes into a TextDescriptionToml
        TextDescriptionType::from_bytes(text_desc.0.as_slice())
            .expect("Failed to parse textDescriptionData")
    }
}

impl TextDescriptionType {
    /// Creates a new `TextDescription` with default values for non-provided fields.
    #[allow(unused)]
    fn new(description: &str) -> Self {
        Self {
            ascii: description.to_string(),
            unicode: description.to_string(),
            unicode_language_code: 0, // Undefined
            scriptcode_code: 0,       // Roman
            mac_script_name: String::new(),
        }
    }

    /// Attempts to parse a `textDescriptionData` tag from a byte buffer.
    fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(buf);

        // 1. Signature (4 bytes) & Reserved (4 bytes)
        let mut signature = [0u8; 4];
        cursor
            .read_exact(&mut signature)
            .map_err(|e| e.to_string())?;
        if &signature != b"desc" {
            return Err("Invalid TextDescription signature".to_string());
        }
        cursor
            .seek(SeekFrom::Current(4))
            .map_err(|e| e.to_string())?; // Skip reserved

        // 2. ASCII description
        let mut len_buf = [0u8; 4];
        cursor.read_exact(&mut len_buf).map_err(|e| e.to_string())?;
        let ascii_len = u32::from_be_bytes(len_buf) as usize;
        let mut ascii_bytes = vec![0u8; ascii_len];
        cursor
            .read_exact(&mut ascii_bytes)
            .map_err(|e| e.to_string())?;
        // The length includes the null terminator, so we trim it before converting.
        let ascii = str::from_utf8(&ascii_bytes[..ascii_len.saturating_sub(1)])
            .map_err(|e| e.to_string())?
            .to_string();

        // 3. Unicode language code
        cursor.read_exact(&mut len_buf).map_err(|e| e.to_string())?;
        let unicode_language_code = u32::from_be_bytes(len_buf);

        // 4. Unicode description
        cursor.read_exact(&mut len_buf).map_err(|e| e.to_string())?;
        let unicode_len = u32::from_be_bytes(len_buf) as usize; // Length in characters
        let mut unicode_bytes = vec![0u8; unicode_len * 2];
        cursor
            .read_exact(&mut unicode_bytes)
            .map_err(|e| e.to_string())?;
        let unicode_u16s: Vec<u16> = unicode_bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        let unicode = String::from_utf16(&unicode_u16s).map_err(|e| e.to_string())?;

        // 5. ScriptCode code
        let mut script_buf = [0u8; 2];
        cursor
            .read_exact(&mut script_buf)
            .map_err(|e| e.to_string())?;
        let scriptcode_code = u16::from_be_bytes(script_buf);

        // 6. Macintosh script name (padded to 67 bytes total)
        let mut mac_len_buf = [0u8; 1];
        cursor
            .read_exact(&mut mac_len_buf)
            .map_err(|e| e.to_string())?;
        let mac_len = mac_len_buf[0] as usize;
        if mac_len > 66 {
            return Err("Invalid Macintosh script name length".to_string());
        }
        let mut mac_bytes = vec![0u8; mac_len];
        cursor
            .read_exact(&mut mac_bytes)
            .map_err(|e| e.to_string())?;
        let mut mac_script_name = str::from_utf8(&mac_bytes)
            .map_err(|e| e.to_string())?
            .trim_end_matches('\0')
            .to_string();

        if mac_script_name == ascii {
            mac_script_name.clear(); // If the mac script name is the same as ASCII, clear it
        }
        Ok(TextDescriptionType {
            ascii,
            unicode,
            unicode_language_code,
            scriptcode_code,
            mac_script_name,
        })
    }

    /*
    fn get_ascii(&self) -> String {
        self.ascii.clone()
    }

    fn get_unicode(&self) -> (String, Language) {
        (
        self.unicode.clone(),
        self.unicode_language_code
            .try_into()
            .unwrap_or_else(|_| Language::default())

        )
    }
     */
}
