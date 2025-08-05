//! # Chromaticity Type
//! - The chromaticity tag type provides basic chromaticity data and type of phosphors or colorants of a monitor to
//!   applications and utilities. The byte assignment shall be as given in `ChromaticityMap`.
//! - Chromaticity data are the CIE xy values of the phosphors or colorants of a monitor, using the CIE 1931
//!   color space.
//! - The `Primaries` enum defines the standard sets of primaries used for display devices, but can be 
//!   set to `Unknown` if the primaries are not known or do not match any standard.
//! - Even though the chromaticity tag is not required, it is recommended to include it in profiles for
//!   display devices to ensure accurate color representation.
//! - Per the ICC stanard, the chromaticity tag shall contain at least the CIE xy values of the first channel,
//!   even when one of the standard `Primaries` is used.
//! - The CIE xy values shall be as measured, and shall not be chromatically adapted to the PCS
//!   adopted white.
//! - The standard requires that: "Each colour component shall be assigned to a device channel. Table 38
//!   lut16Type channel encodings” shows these assignments." The tag has no field for this.
//!   Defacto, `RGB` channels are used, as also implied by the standard primaries, but this is not enforced by the ICC standard.
//! - This type and the Chromaticity tag is not a required for any device profile, and also not mentioned in the ICC specification as optional tag for any of them.
use num::{FromPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};
use serde::Serialize;
use zerocopy::{BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, U32, U16};

use crate::tags::ChromaticityType;
use colorimetry::xyz as cmt;

const ITU: [cmt::Chromaticity;3] = [
    cmt::Chromaticity::new(0.640, 0.330),
    cmt::Chromaticity::new(0.300, 0.600),
    cmt::Chromaticity::new(0.150, 0.060),
];

const SMPTE: [cmt::Chromaticity;3] = [
    cmt::Chromaticity::new(0.630, 0.340),
    cmt::Chromaticity::new(0.310, 0.595),
    cmt::Chromaticity::new(0.155, 0.070),
];

const EBU: [cmt::Chromaticity;3] = [
    cmt::Chromaticity::new(0.640, 0.330),
    cmt::Chromaticity::new(0.290, 0.600),
    cmt::Chromaticity::new(0.150, 0.060),
];

const P22: [cmt::Chromaticity;3] = [
    cmt::Chromaticity::new(0.625, 0.340),
    cmt::Chromaticity::new(0.280, 0.605),
    cmt::Chromaticity::new(0.155, 0.070),
];

const P3: [cmt::Chromaticity;3] = [
    cmt::Chromaticity::new(0.680, 0.320),
    cmt::Chromaticity::new(0.265, 0.690),
    cmt::Chromaticity::new(0.150, 0.060),
];

const ITU2020: [cmt::Chromaticity;3] = [
    cmt::Chromaticity::new(0.708, 0.292),
    cmt::Chromaticity::new(0.170, 0.797),
    cmt::Chromaticity::new(0.131, 0.046),
];

#[derive(
    Debug,
    Serialize,
    FromPrimitive,
    ToPrimitive,
    Clone,
    Copy,
    PartialEq,
    TryFromBytes,
    IntoBytes,
    KnownLayout,
    Immutable,
)]


#[repr(C)]
pub enum Primaries {
    Unknown = 0x0000,
    ITU = 0x0001,   // ITU-R BT.709-2
    SMPTE = 0x0002, // SMPTE RP145
    EBU = 0x0003,   // EBU Tech. 3213-E
    P22 = 0x0004,
    P3 = 0x0005,      // SMPTE ST 428-1
    ITU2020 = 0x0006, // ITU-R BT.2020, Rec 2020, BT2020
}

impl Primaries {
    pub fn rgb(&self) -> Option<[cmt::Chromaticity; 3]> {
        match self {
            Primaries::ITU => Some(ITU),
            Primaries::SMPTE => Some(SMPTE),
            Primaries::EBU => Some(EBU),
            Primaries::P22 => Some(P22),
            Primaries::P3 => Some(P3),
            Primaries::ITU2020 => Some(ITU2020),
            Primaries::Unknown => None,
        }
    }
}

impl Default for Primaries {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
struct ChromaticityMap {
    type_signature: [u8; 4],
    reserved: [u8; 4],
    channels: U16<BigEndian>,
    primaries: U16<BigEndian>,
    data: [[U32<BigEndian>; 2]],
}

impl ChromaticityType {
    /// Returns the primaries of the chromaticity map.
    fn chromaticity_map(&self) -> &ChromaticityMap {
        ChromaticityMap::try_ref_from_bytes(self.0.as_slice())
            .expect("Failed to convert ChromaticityMap from bytes")
    }

    fn chromaticity_map_mut(&mut self) -> &mut ChromaticityMap {
        ChromaticityMap::try_mut_from_bytes(&mut self.0)
            .expect("Failed to convert ChromaticityMap from bytes")
    }


    pub fn chromaticities(&self) -> Option<[cmt::Chromaticity; 3]> {
        let t = ChromaticityMap::try_ref_from_bytes(self.0.as_slice())
            .expect("Failed to convert ChromaticityMap from bytes");

        let primaries = FromPrimitive::from_u16(t.primaries.get()).unwrap_or_default();
        match primaries {
            Primaries::Unknown => {
                // Get the measured chromaticities
                self.get_custom_chromaticities()
            }
            _ => primaries.rgb()
        }
    }

    pub fn get_custom_chromaticities(&self) -> Option<[cmt::Chromaticity; 3]> {
        let t = self.chromaticity_map();
        if t.channels.get() != 3 {
            // The chromaticity map should have exactly 3 channels
            return None;
        }

        if t.primaries.get() != 0 {
            // An unknown primaries value was specified
            None
        } else {
            let values: Vec<[f64; 2]> = t.data.iter()
                .map(|&x| [x[0].get() as f64/u16::MAX as f64, x[1].get() as f64 /u16::MAX as f64])
                .collect();
            if values.len() < 3 {
                // Not enough chromaticity values provided
                None
            }
            else {
                Some([
                    cmt::Chromaticity::new(values[0][0], values[0][1]),
                    cmt::Chromaticity::new(values[1][0], values[1][1]),
                    cmt::Chromaticity::new(values[2][0], values[2][1]),
                ])
            }
        }
    }

    pub fn set_custom_chromaticities(&mut self, chromaticities: [cmt::Chromaticity; 3])  {
        let mut data = [[U32::new(0); 2]; 16];
        for (i, &chromaticity) in chromaticities.iter().enumerate() {
            if i < data.len() {
                data[i][0].set((chromaticity.x() * u16::MAX as f64) as u32);
                data[i][0].set((chromaticity.y() * u16::MAX as f64) as u32);
            }
        }

        self.0.resize(4 + 4 + 2 + 2 + 3 * 4, 0);
      //  let mut_ref = ChromaticityMap::try_mut_from_bytes(&mut self.0)
      //      .expect("Failed to convert ChromaticityMap from bytes");
        let mut_ref = self.chromaticity_map_mut();
        mut_ref.primaries.set(0);
        mut_ref.data.copy_from_slice(&data[..mut_ref.data.len()]);
    }
}
