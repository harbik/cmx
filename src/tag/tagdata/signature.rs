// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::{
    signatures::Signature,
    tag::{
        tagdata::{DataSignature, SignatureData},
        TagDataTraits,
    },
};

#[derive(Serialize)]
pub struct SignatureType {
    signature: String,
}

#[repr(C)]
#[derive(FromBytes, IntoBytes, KnownLayout, Unaligned, Immutable)]
pub struct Layout {
    /// TagData signature, must be `b"raw "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    /// Raw data bytes.
    data: [u8; 4],
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            signature: DataSignature::SignatureData.into(),
            _reserved: [0; 4],
            data: [0; 4],
        }
    }
}

impl Layout {
    pub fn new(signature: &str) -> Self {
        let s: Signature = signature
            .parse()
            .expect("Signature must be a valid 4-character string");
        let data: [u8; 4] = s.into();
        Self {
            signature: DataSignature::SignatureData.into(),
            _reserved: [0; 4],
            data,
        }
    }
}

impl From<&SignatureData> for SignatureType {
    fn from(signature_data: &SignatureData) -> Self {
        let layout = Layout::ref_from_bytes(signature_data.as_slice()).unwrap();

        Self {
            signature: format!("{}", String::from_utf8_lossy(&layout.data)),
        }
    }
}

impl SignatureData {
    pub fn set_signature(&mut self, signature: &str) {
        // overwrite the existing data
        // if this is a new tag, there will be no existing data
        let data = Layout::new(signature);
        self.0 = data.as_bytes().to_vec();
    }
}
