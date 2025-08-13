// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

use crate::tag::tagdata::RawData;

#[derive(Serialize)]
pub struct RawType {
    type_signature: String,
    #[serde(skip)]
    #[allow(unused)]
    data: Vec<u8>,
    hex: String,
}

#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
pub struct RawTagDataLayout {
    /// TagData signature, must be `b"raw "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    /// Raw data bytes.
    data: [u8],
}

impl From<&RawData> for RawType {
    fn from(raw: &RawData) -> Self {
        let layout = RawTagDataLayout::ref_from_bytes(&raw.0).unwrap();

        Self {
            type_signature: String::from_utf8_lossy(&layout.signature).to_string(),
            data: layout.data.to_vec(),
            hex: hex::encode(&layout.data),
        }
    }
}
