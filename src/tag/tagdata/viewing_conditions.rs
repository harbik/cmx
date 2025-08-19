// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

#![allow(unused)]
use crate::tag::tagdata::{Illuminant, ViewingConditionsData};
use num::FromPrimitive;
use serde::Serialize;
use zerocopy::{BigEndian, FromBytes, Immutable, KnownLayout, Unaligned, I32, U32};

#[derive(FromBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct Layout {
    signature: [u8; 4], // "view"
    _reserved: [u8; 4], // reserved, must be 0
    xyz_illuminant: [I32<BigEndian>; 3],
    xyz_surround: [I32<BigEndian>; 3],
    illuminant: U32<BigEndian>, // StandardIlluminant
}

#[derive(Default, Serialize, Debug, Clone, PartialEq)]
pub struct ViewingConditionsType {
    pub xyz_illuminant: [f64; 3],
    pub xyz_surround: [f64; 3],
    pub illuminant: Illuminant, // StandardIlluminant
}

impl From<&ViewingConditionsData> for ViewingConditionsType {
    fn from(data: &ViewingConditionsData) -> Self {
        if data.0.is_empty() {
            return ViewingConditionsType::default();
        }
        let (layout, _) = Layout::ref_from_prefix(&data.0).unwrap();

        // Header validation (debug-only): signature must be "view", reserved must be zero.
        debug_assert_eq!(
            &layout.signature, b"view",
            "ViewingConditionsType: invalid signature"
        );
        debug_assert_eq!(
            layout._reserved, [0; 4],
            "ViewingConditionsType: reserved must be zero"
        );

        ViewingConditionsType {
            xyz_illuminant: layout.xyz_illuminant.map(|v| v.get() as f64 / 65536.0),
            xyz_surround: layout.xyz_surround.map(|v| v.get() as f64 / 65536.0),
            illuminant: Illuminant::from_u32(layout.illuminant.get()).unwrap(),
        }
    }
}
