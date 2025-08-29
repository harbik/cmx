// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

use crate::{
    format_hex_with_spaces,
    tag::{tagdata::TagData, TagDataTraits},
};

#[derive(Serialize)]
pub struct UnparsedType {
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
    /// TagData signature, must be `b"raw "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    /// Raw data bytes.
    data: [u8],
}

impl From<&TagData> for UnparsedType {
    fn from(tagdata: &TagData) -> Self {
        let layout = Layout::ref_from_bytes(tagdata.as_slice()).unwrap();

        Self {
            type_signature: format!("{}", String::from_utf8_lossy(&layout.signature)),
            data: layout.data.to_vec(),
            hex: format_hex_with_spaces(&layout.data),
        }
    }
}
