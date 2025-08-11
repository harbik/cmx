use serde::Serialize;
use zerocopy::{BigEndian, FromBytes, Immutable, KnownLayout, Unaligned, I32};

use crate::tag::tag_value::XYZArrayType;


/// The fixed-point denominator for an s15Fixed16Number (2^16).
const S15_FIXED_16_DIVISOR: f64 = 65536.0;

/// Represents the raw memory layout of an ICC `XYZType` tag.
///
/// It is marked with `#[repr(C)]` to ensure a predictable field order
/// and memory layout, which is required for safe, zero-cost casting.
#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
struct XYZTagLayout {
    /// TagValue signature, must be `b"XYZ "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    /// Array of three CIEXYZ values, stored as s15Fixed16Numbers.
    xyz: [[I32<BigEndian>; 3]],
}

// Serializable structs for each tag type
#[derive(Serialize)]
pub struct XYZArrayTypeToml{
    xyz: Vec<f64>
}


/// Parses the raw data wrapped in XYZType into a XYZTypeToml instance,
/// as used 
impl From<&XYZArrayType> for XYZArrayTypeToml {
    fn from(xyz: &XYZArrayType) -> Self {
        let layout = XYZTagLayout::ref_from_bytes(&xyz.0).unwrap();
        
        // Flatten directly during the conversion
        let xyz_vec: Vec<f64> = layout.xyz.iter()
            .flat_map(|xyz| {
                [
                    xyz[0].get() as f64 / S15_FIXED_16_DIVISOR,
                    xyz[1].get() as f64 / S15_FIXED_16_DIVISOR,
                    xyz[2].get() as f64 / S15_FIXED_16_DIVISOR,
                ]
            })
            .collect();

        Self{
           xyz: xyz_vec
        }
    }
}



/*

impl XYZ {
    /// Creates a new `XYZ` value.
    #[allow(unused)]
    pub fn new(x:f64, y: f64, z:f64) -> Self {
        let xyz = colorimetry::xyz::XYZ::new([x, y, z], colorimetry::observer::Observer::Cie1931);
        Self(vec![xyz])
    }

    #[allow(unused)]
    pub fn from_vec(values: Vec<[f64; 3]>) -> Self {
        Self(values.into_iter().map(
            |xyz| colorimetry::xyz::XYZ::new(xyz,  colorimetry::observer::Observer::Cie1931))
            .collect())
    }

    /// Parses an `XYZType` tag from a byte buffer using a zero-copy read.
    ///
    /// This function safely casts the beginning of the buffer to an `XYZTagLayout`,
    /// validates the signature, and then converts the s15Fixed16Number fixed-point
    /// values into `f64` floating-point numbers.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is too short, if the signature is not `b"XYZ "`,
    /// or if the buffer contains more than one XYZ value (as this parser expects
    /// tags like 'wtpt' or 'bkpt' which contain a single XYZ value).
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Result<Self, crate::Error> {

        // Use `ref_from_prefix` for a safe, zero-cost cast from the byte slice.
        // This checks alignment and size, returning an Option.
        let layout = 
            XYZTagLayout::ref_from_bytes(buf)
            .map_err(|e| crate::error::ParseError::new(format!("Failed to parse XYZType: {}", e)))?;

        // Validate the signature.
        if &layout.signature != b"XYZ " {
            return Err(crate::error::ParseError::new("Invalid XYZType signature").into());
        }

        // Convert the s15Fixed16Number values to f64.
        let x = layout.xyz[0].get() as f64 / S15_FIXED_16_DIVISOR;
        let y = layout.xyz[1].get() as f64 / S15_FIXED_16_DIVISOR;
        let z = layout.xyz[2].get() as f64 / S15_FIXED_16_DIVISOR;

        Ok(XYZ::new(x, y, z))
    }
}

impl fmt::Display for XYZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = Vec::<String>::from_iter(self.0.iter().map(|xyz| {
            let [x, y, z] = xyz.values();
            format!("X={:.4}, Y={:.4}, Z={:.4}", x, y, z)
        }));
        Ok(write!(f, "{}", output.join("\n"))?)
    }
}
 */