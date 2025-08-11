#![allow(unused)]
use serde::Serialize;
/*
use crate::tags::common::*;
use serde::Serialize;

// DEPRECATED_IN_MAC_OS_X_VERSION_10_6_AND_LATER

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct MakeAndModel {
    manufacturer: u32,
    model: u32,
    serial: u32,
    date: u32,
}

impl MakeAndModel {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let manufacturer = read_be_u32(buf)?;
        let model = read_be_u32(buf)?;
        let serial = read_be_u32(buf)?;
        let date = read_be_u32(buf)?;
        let _reserved1 = read_be_u32(buf)?;
        let _reserved2 = read_be_u32(buf)?;
        let _reserved3 = read_be_u32(buf)?;
        let _reserved4 = read_be_u32(buf)?;
        Ok(Self {
            manufacturer,
            model,
            serial,
            date,
        })
    }
}

 */