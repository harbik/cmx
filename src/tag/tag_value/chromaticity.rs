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
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::Serialize;
use zerocopy::{BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, U16, U32};

use crate::{tag::tag_value::ChromaticityType, tag::tag_value::TypeSignature};
use colorimetry::xyz as cmt;

const ITU: [cmt::Chromaticity; 3] = [
    cmt::Chromaticity::new(0.640, 0.330),
    cmt::Chromaticity::new(0.300, 0.600),
    cmt::Chromaticity::new(0.150, 0.060),
];

const SMPTE: [cmt::Chromaticity; 3] = [
    cmt::Chromaticity::new(0.630, 0.340),
    cmt::Chromaticity::new(0.310, 0.595),
    cmt::Chromaticity::new(0.155, 0.070),
];

const EBU: [cmt::Chromaticity; 3] = [
    cmt::Chromaticity::new(0.640, 0.330),
    cmt::Chromaticity::new(0.290, 0.600),
    cmt::Chromaticity::new(0.150, 0.060),
];

const P22: [cmt::Chromaticity; 3] = [
    cmt::Chromaticity::new(0.625, 0.340),
    cmt::Chromaticity::new(0.280, 0.605),
    cmt::Chromaticity::new(0.155, 0.070),
];

const P3: [cmt::Chromaticity; 3] = [
    cmt::Chromaticity::new(0.680, 0.320),
    cmt::Chromaticity::new(0.265, 0.690),
    cmt::Chromaticity::new(0.150, 0.060),
];

const ITU2020: [cmt::Chromaticity; 3] = [
    cmt::Chromaticity::new(0.708, 0.292),
    cmt::Chromaticity::new(0.170, 0.797),
    cmt::Chromaticity::new(0.131, 0.046),
];

#[derive(
    Debug,
    Default,
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
    strum::Display
)]
#[repr(C)]
/// Colorant and Phosphor Encoding, or Primaries, as defined in Table 31 of the
/// [ICC specification](https://www.color.org/specification/ICC.1-2022-05.pdf).
pub enum StandardPrimaries {
    #[default]
    ITU = 0x0001, // ITU-R BT.709-2
    SMPTE = 0x0002, // SMPTE RP145
    EBU = 0x0003,   // EBU Tech. 3213-E
    P22 = 0x0004,
    P3 = 0x0005,      // SMPTE ST 428-1
    ITU2020 = 0x0006, // ITU-R BT.2020, Rec 2020, BT2020
}

impl StandardPrimaries {
    pub fn rgb(&self) -> Option<[cmt::Chromaticity; 3]> {
        match self {
            StandardPrimaries::ITU => Some(ITU),
            StandardPrimaries::SMPTE => Some(SMPTE),
            StandardPrimaries::EBU => Some(EBU),
            StandardPrimaries::P22 => Some(P22),
            StandardPrimaries::P3 => Some(P3),
            StandardPrimaries::ITU2020 => Some(ITU2020),
        }
    }
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
struct Layout {
    type_signature: [u8; 4],
    reserved: [u8; 4],
    channels: U16<BigEndian>,
    primaries: U16<BigEndian>,
    data: [[U32<BigEndian>; 2]],
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
struct WriteLayout<const N: usize> {
    type_signature: [u8; 4],
    reserved: [u8; 4],
    channels: U16<BigEndian>,
    primaries: U16<BigEndian>,
    data: [[U32<BigEndian>; 2]; N],
}

impl WriteLayout<1> {
    /// Returns the size of the ChromaticityMap in bytes.
    pub fn new_primary(sp: StandardPrimaries) -> Self {
        let mut map = WriteLayout::<1>::default();
        map.primaries
            .set(num::ToPrimitive::to_u16(&sp).unwrap_or_default());
        map
    }

    pub fn new_custom(chromaticities: [cmt::Chromaticity; 3]) -> Self {
        let mut map = WriteLayout::<1>::default();
        map.channels.set(3); // Set to 3 channels for RGB
        map.primaries.set(0); // Set to 0 for custom primaries

        for (i, chromaticity) in chromaticities.iter().enumerate() {
            if i < map.data.len() {
                map.data[i][0].set((chromaticity.x() * u16::MAX as f64) as u32);
                map.data[i][1].set((chromaticity.y() * u16::MAX as f64) as u32);
            }
        }
        map
    }
}

impl Default for WriteLayout<1> {
    fn default() -> Self {
        WriteLayout {
            type_signature: TypeSignature::ChromaticityType.into(),
            reserved: [0; 4],
            channels: U16::new(1),
            primaries: U16::new(1), // e.g., ITU-R BT.709 colorant
            data: [[U32::new(0), U32::new(0)]],
        }
    }
}

#[derive(Serialize)]
pub struct ChromaticityTypeToml {
    #[serde(skip_serializing_if = "Option::is_none")]
    primaries: Option<StandardPrimaries>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chromaticities: Option<Vec<[f64; 2]>>,
}

impl From<&ChromaticityType> for ChromaticityTypeToml {
    fn from(chromaticity: &ChromaticityType) -> Self {
        let chromaticities_opt = chromaticity.get_custom_chromaticities();
        let primaries = FromPrimitive::from_u16(chromaticity.chromaticity_map().primaries.get());

        let chromaticities = if let Some(chromaticities) = chromaticities_opt {
            Some(chromaticities.iter().map(|c| [c.x(), c.y()]).collect())
        } else {
            None
        };

        ChromaticityTypeToml {
            primaries,
            chromaticities,
        }
    }
}

// The ChromaticityType is a thin wrapper around a Vec<u6>, containing the raw bytes of the RawProfile.
impl ChromaticityType {
    /// Returns the primaries of the chromaticity map.
    fn chromaticity_map(&self) -> &Layout {
        Layout::try_ref_from_bytes(self.0.as_slice())
            .expect("Failed to convert ChromaticityMap from bytes")
    }

    /*
    fn chromaticity_map_mut(&mut self) -> &mut ChromaticityLayout {
        ChromaticityLayout::try_mut_from_bytes(&mut self.0)
            .expect("Failed to convert ChromaticityMap from bytes")
    }
     */

    /*
    pub fn chromaticities(&self) -> Option<[cmt::Chromaticity; 3]> {
        let t = Layout::try_ref_from_bytes(self.0.as_slice())
            .expect("Failed to convert ChromaticityMap from bytes");

        let primaries = FromPrimitive::from_u16(t.primaries.get()).unwrap_or_default();
        match primaries {
            StandardPrimaries::Unknown => {
                // Get the measured chromaticities
                self.get_custom_chromaticities()
            }
            _ => primaries.rgb()
        }
    }
     */

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
            let values: Vec<[f64; 2]> = t
                .data
                .iter()
                .map(|&x| {
                    [
                        x[0].get() as f64 / u16::MAX as f64,
                        x[1].get() as f64 / u16::MAX as f64,
                    ]
                })
                .collect();
            if values.len() < 3 {
                // Not enough chromaticity values provided
                None
            } else {
                Some([
                    cmt::Chromaticity::new(values[0][0], values[0][1]),
                    cmt::Chromaticity::new(values[1][0], values[1][1]),
                    cmt::Chromaticity::new(values[2][0], values[2][1]),
                ])
            }
        }
    }

    pub fn set_standard(&mut self, primaries: StandardPrimaries) {
        let map = WriteLayout::new_primary(primaries);
        self.0 = map.as_bytes().to_vec();
    }

    pub fn set_custom(&mut self, chromaticities: [cmt::Chromaticity; 3]) {
        let map = WriteLayout::new_custom(chromaticities);
        self.0 = map.as_bytes().to_vec();
    }
}
