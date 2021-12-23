use crate::profile::{read_signature, read_be_u32, read_be_u16, read_date_time, read_xyz};



#[derive(Debug)]
pub struct ColorantOrder(Vec<u8>);

#[derive(Debug)]
pub struct Curve(Vec<u16>);

#[derive(Debug)]
pub struct Data(Vec<u8>);

#[derive(Debug)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

#[derive(Debug)]
pub struct XYZ(Vec<[f64;3]>);

#[derive(Debug)]
pub enum Tag {
    ColorantOrder(ColorantOrder), // 'clro'
    Curve(Curve), // 'data' with flag 1
    Data(Data), // 'data' with flag 1
    DateTime(DateTime), // 'dtim'
    Dict(Vec<u8>), // 'dict' 
    EmbeddedHeigthImage(Vec<u8>), // 'ehim'
    EmbeddedNormalImage(Vec<u8>), // 'enim'
    Float16Array(Vec<half::f16>), // 'fl16'
    Float32Array(Vec<f32>), // 'fl32'
    Float64Array(Vec<f64>), // 'fl64'
    GamutBoundaryDescription(Vec<u8>), // 'gbd'
    LutAToB(Vec<u8>), // 'mAB'
    LutBToA(Vec<u8>), // 'mBA'
    Measurement(Vec<u8>), // 'meas'
    MultiLocalizedUnicode(Vec<u8>), // 'mluc'
    MultiProcessElements(Vec<u8>), // 'mpet'
    S15Fixed16Array(Vec<f64>), // 'sf32'
    Signature([u8;4]), // 'sig'
    SparseMatrixArray(Vec<u8>), // 'smat'
    SpectralViewingConditions(Vec<u8>), // 'svcn'
    TagStruct(Vec<u8>), // 'tstr'
    U16Fixed16Array(Vec<f32>), // 'uf32'
    UInt8Array(Vec<u8>), // 'ui16'
    UInt16Array(Vec<u16>), // 'ui16'
    UInt32Array(Vec<u32>), // 'ui16'
    UInt64Array(Vec<u64>), // 'ui64'
    Utf8(Vec<String>), // 'utf8'
    Utf16(Vec<String>), // 'ut16'
    Utf8Zip(Vec<String>), // 'zut8'
    XYZ(XYZ), // 'XYZ'
    Custom(String, Vec<u8>), // unknown data type
}

impl Tag {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self, Box< dyn std::error::Error + 'static>> {
        let sig = read_signature(buf)?.ok_or("illegal data type")?;
        let _reserved = read_be_u32(buf)?;
        match sig.as_str() {
            "clro" => Ok(Self::ColorantOrder(ColorantOrder(buf.to_owned()))),
            "curv" => {
                let n = read_be_u32(buf)? as usize;
                let mut v: Vec<u16> = Vec::with_capacity(n);
                for _ in 0..n {
                    v.push(read_be_u16(buf)?);
                }
                Ok(Self::Curve(Curve(v)))
            }
            "data" => {
                let _n = read_be_u32(buf)? as usize;
                Ok(Self::Data(Data(buf.to_owned())))
            },
            "dtim" => {
                Ok(Self::DateTime(DateTime(read_date_time(buf)?.unwrap())))
            },
            "XYZ " => {
                let n = buf.len()/12;
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    if let Some(xyz) = read_xyz(buf)? {
                        v.push(xyz);
                    } else {
                        return Err("Wrong number of XYZ values".into());
                    }
                }
                Ok(Self::XYZ(XYZ(v)))

            }
            _ => Ok(Self::Custom(sig.to_owned(), buf.to_owned())),
        } 
    }
}