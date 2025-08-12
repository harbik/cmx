// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use std::fmt;

/// Represents an ISO 639-1 language and ISO 3166-1 country code pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language {
    /// 2-character language code (e.g., "en").
    pub language: [u8; 2],
    /// 2-character country code (e.g., "US").
    pub country: [u8; 2],
}

impl Default for Language {
    /// The default language is considered "undefined" as per the spec.
    fn default() -> Self {
        Language {
            language: [0, 0],
            country: [0, 0],
        }
    }
}

impl TryFrom<u32> for Language {
    type Error = &'static str;

    /// Converts a 4-byte integer from an ICC profile into a Language struct.
    /// Example: 0x656E5553 -> 'enUS'
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            // A value of 0 is explicitly "undefined" in the spec.
            return Err("Language code is undefined (0)");
        }

        // to_be_bytes() unpacks the u32 into its 4 constituent bytes
        // in big-endian order, which matches the ICC specification.
        let bytes = value.to_be_bytes();

        Ok(Language {
            language: [bytes[0], bytes[1]],
            country: [bytes[2], bytes[3]],
        })
    }
}

impl fmt::Display for Language {
    /// Formats the language code as a "lang-COUNTRY" string, e.g., "en-US".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.language == [0, 0] {
            return write!(f, "undefined");
        }

        // Safely convert byte arrays to string slices for printing.
        let lang_str = std::str::from_utf8(&self.language).unwrap_or("??");
        let country_str = std::str::from_utf8(&self.country).unwrap_or("??");
        write!(f, "{}-{}", lang_str, country_str)
    }
}
