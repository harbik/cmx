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
    #[serde(rename = "unparsed")]
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
    pub fn set_data(&mut self, data: &[u8]) {
        let mut new_data = Vec::with_capacity(8 + data.len());
        new_data.extend_from_slice(&self.0[..8]);
        new_data.extend_from_slice(data);
        self.0 = new_data;
    }

    pub fn set_hex(&mut self, hex: &str) {
        let bytes = hex
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .collect::<Vec<u8>>();
        self.set_data(&bytes);
    }
}

#[cfg(test)]
mod raw_test {
    // use std::str::FromStr;

    // use crate::tag::TagSignature;

    #[test]
    fn test_raw_tag() {
        /*
        let signature = crate::signatures::Signature::from_str("ndin").unwrap();
        let tag_signature = TagSignature::from(signature.0);
        let _profile = crate::profile::RawProfile::default()
            .with_tag(tag_signature)
            .as_raw(|raw| {
                raw.set_hex("01 02 03 04 05 06 07 08 09 0A 0B 0C");
            });
         */
    }
}
