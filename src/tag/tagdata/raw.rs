// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

use crate::{
    format_hex_with_spaces,
    tag::{
        tagdata::{RawData, TagData},
        TagDataTraits,
    },
};

#[derive(Serialize)]
pub struct RawType {
    #[serde(rename = "type")]
    type_signature: String,
    #[serde(skip)]
    #[allow(unused)]
    data: Vec<u8>,
    hex: String,
}

#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
pub struct Layout {
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    /// Raw data bytes.
    data: [u8],
}

impl From<&TagData> for RawType {
    fn from(tagdata: &TagData) -> Self {
        let layout = Layout::ref_from_bytes(tagdata.as_slice()).unwrap();

        Self {
            type_signature: format!("{}", String::from_utf8_lossy(&layout.signature)),
            data: layout.data.to_vec(),
            hex: format_hex_with_spaces(&layout.data),
        }
    }
}

impl RawData {
    pub fn set_bytes(&mut self, data: &[u8]) {
        let mut new_data = Vec::with_capacity(8 + data.len());
        new_data.extend_from_slice(&self.0[..8]);
        new_data.extend_from_slice(data);
        self.0 = new_data;
    }

    pub fn set_hex(&mut self, hex: &str) {
        if hex.is_empty()
            || hex
                .chars()
                .any(|c| !(c.is_whitespace() || c.is_ascii_hexdigit()))
        {
            panic!("Invalid hex string");
        }
        let bytes = crate::parse_hex_string(hex).unwrap();
        self.set_bytes(&bytes);
    }
}

#[cfg(test)]
mod raw_test {
    use crate::tag::{TagDataTraits, TagSignature};


    #[test]
    fn test_raw_tag() {
        let profile = crate::profile::DisplayProfile::default()
            .with_tag("cmx0")
            .as_raw(|raw| {
                raw.set_hex("12345678 9abc");
            });
        let cmx0: TagSignature = "cmx0".into();
        let data = profile.0.tags.get(&cmx0).unwrap().tag.data();
        let data_hex = crate::format_hex_with_spaces(data.as_slice());

        // "cmxx0" tag signature, 4 reserved bytes (0), and the data we set.
        assert_eq!(data_hex, "636d7830 00000000 12345678 9abc");
        //println!("{profile}");
    }
}
