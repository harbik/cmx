use serde::Serialize;
use zerocopy::{BigEndian, FromBytes, Immutable, KnownLayout, Unaligned, I32, U16};
use crate::{is_zero, tag::tag_value::ParametricCurveType};

/// Represents the raw memory layout of an ICC `ParametricCurveType` tag.
#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
struct ParametricCurveTagLayout {
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    encoded_value: U16<BigEndian>,
    _reserved2: [u8; 2],
    /// Array of three CIEXYZ values, stored as s15Fixed16Numbers.
    parameters: [I32<BigEndian>],
}

// Serializable structs for each tag type
#[derive(Serialize)]
pub struct ParametricCurveTypeToml{
    #[serde(skip_serializing_if="is_zero")]
    a: f64,
    #[serde(skip_serializing_if="is_zero")]
    b: f64,
    #[serde(skip_serializing_if="is_zero")]
    c: f64,
    #[serde(skip_serializing_if="is_zero")]
    d: f64,
    #[serde(skip_serializing_if="is_zero")]
    e: f64,
    #[serde(skip_serializing_if="is_zero")]
    f: f64,
    #[serde(skip_serializing_if="is_zero")]
    g: f64,
}

/// Parses the raw data wrapped in XYZType into a XYZTypeToml instance,
/// as used 
impl From<&ParametricCurveType> for ParametricCurveTypeToml {
    fn from(para: &ParametricCurveType) -> Self {
        const S15_FIXED_16_DIVISOR: f64 = 65536.0;
        let layout = ParametricCurveTagLayout::ref_from_bytes(&para.0).unwrap();
        
        // Flatten directly during the conversion
        let vec: Vec<f64> = layout.parameters.iter()
            .map(|v| {
                    crate::round_to_precision(v.get() as f64 / S15_FIXED_16_DIVISOR, 4)
            })
            .collect();

        // Copy up to 7 values, defaulting the rest to zero
        let mut params = [0.0_f64; 7];
        for (i, v) in vec.iter().take(7).enumerate() {
            params[i] = *v;
        }
        let [g, a, b, c, d, e, f] = params;

        Self { g, a, b, c, d, e, f }
    }
}

