
use crate::{profile::*, tag_signatures::TagSignature};
use num::{FromPrimitive, Zero};
use num_derive::FromPrimitive;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Tag {
    tag_signature: TagSignature,
    type_signature: TagTypeSignature,
    data: TagData,
}


#[derive(Debug, Serialize)]
pub enum TagData {
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
    Measurement(Measurement), // 'meas'
    MultiLocalizedUnicode(Vec<u8>), // 'mluc'
    MultiProcessElements(Vec<u8>), // 'mpet'
    ParametricCurve(ParametricCurve), // 'para'
    S15Fixed16Array(Vec<f32>), // 'sf32'
    Signature([u8;4]), // 'sig'
    SparseMatrixArray(Vec<u8>), // 'smat'
    SpectralViewingConditions(Vec<u8>), // 'svcn'
    TagStruct(Vec<u8>), // 'tstr'
    Technology(TechnologySignature), // tag derived type
    Text(String),
    TextDescription(TextDescription),
    U16Fixed16Array(Vec<f32>), // 'uf32'
    UInt8Array(Vec<u8>), // 'ui16'
    UInt16Array(Vec<u16>), // 'ui16'
    UInt32Array(Vec<u32>), // 'ui16'
    UInt64Array(Vec<u64>), // 'ui64'
    Utf8(Vec<String>), // 'utf8'
    Utf16(Vec<String>), // 'ut16'
    Utf8Zip(Vec<String>), // 'zut8'
    ViewingConditions(ViewingConditions),
    XYZ(XYZ), // 'XYZ'
    Custom(TagTypeSignature, Vec<u8>), // unknown data type
}

impl Tag {
    pub fn try_new(tag_signature: TagSignature, buf: &mut &[u8]) -> Result<Self, Box< dyn std::error::Error + 'static>> {
        let type_signature = match FromPrimitive::from_u32(read_be_u32(buf)?) {
            Some(c) => c,
            None => TagTypeSignature::UndefinedType,
        };
        let _reserved = read_be_u32(buf)?;
        Ok(Self {
            tag_signature: tag_signature.clone(),
            type_signature,
            data: TagData::try_new(tag_signature, type_signature, buf)?,
        })
    }
}


#[derive(Debug, Serialize)]
pub struct ColorantOrder(Vec<u8>);

#[derive(Debug, Serialize)]
pub struct Curve(Vec<u16>);

#[derive(Debug, Serialize)]
pub struct Data(Vec<u8>);

#[derive(Debug, Serialize)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

#[derive(Debug, Serialize)]
pub struct Float16Array(Vec<half::f16>);

#[derive(Debug, Serialize)]
pub struct Float32Array(Vec<f32>);

#[derive(Debug, Serialize)]
pub struct Float64Array(Vec<f64>);

#[derive(Debug, Serialize)]
pub struct Measurement {
    pub standard_observer: StandardObserver,
    pub xyz: [f64;3],
    pub geometry: Geometry,
    pub flare: Flare,
    pub illuminant: StandardIlluminant,
}


impl Measurement {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        Ok(Measurement{
            standard_observer: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
            xyz: read_xyz(buf)?.unwrap_or_default(),
            geometry: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
            flare: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
            illuminant: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Serialize)]
pub enum ParametricCurve {
    ExponentGamma{g: f32},
    CIE122{g: f32, a: f32, b:f32},
    IEC61966_3{g: f32, a: f32, b:f32, c: f32},
    IEC61966_2_1{g: f32, a: f32, b:f32, c: f32, d: f32},
    SevenParameter{g: f32, a: f32, b:f32, c: f32, d: f32, e: f32, f: f32},
}

impl ParametricCurve{
    pub fn try_new(buf: &mut &[u8]) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let function_type = read_be_u16(buf)?;
        let _not_used = read_be_u16(buf)?;
        match function_type {
            0 => Ok(Self::ExponentGamma{g:read_s15fixed16(buf)?}),
            1 => Ok(Self::CIE122{
                g:read_s15fixed16(buf)?,
                a:read_s15fixed16(buf)?,
                b:read_s15fixed16(buf)?,
            }),
            2 => Ok(Self::IEC61966_3{
                g:read_s15fixed16(buf)?,
                a:read_s15fixed16(buf)?,
                b:read_s15fixed16(buf)?,
                c:read_s15fixed16(buf)?,
            }),
            3 => Ok(Self::IEC61966_2_1{
                g:read_s15fixed16(buf)?,
                a:read_s15fixed16(buf)?,
                b:read_s15fixed16(buf)?,
                c:read_s15fixed16(buf)?,
                d:read_s15fixed16(buf)?,
            }),
            4 => Ok(Self::SevenParameter{
                g:read_s15fixed16(buf)?,
                a:read_s15fixed16(buf)?,
                b:read_s15fixed16(buf)?,
                c:read_s15fixed16(buf)?,
                d:read_s15fixed16(buf)?,
                e:read_s15fixed16(buf)?,
                f:read_s15fixed16(buf)?,
            }),
            _ => Err("Illegal function type".into())

        }
    }

    pub fn value(&self, x: f32) -> f32 {
        if x<0.0 || x>1.0 { 
            f32::NAN
        } else {
            match *self {
                Self::ExponentGamma{g} => x.powf(g),
                Self::CIE122{g,a,b} => {
                    if x>= -b/a {
                        (a*x + b).powf(g)
                    } else {
                        0.0
                    }
                }
                Self::IEC61966_3{g,a,b, c} => {
                    if x>= -b/a {
                        (a*x + b).powf(g) + c
                    } else {
                       c 
                    }
                }
                Self::IEC61966_2_1{g,a,b, c, d} => {
                    if x>= d {
                        (a*x + b).powf(g)
                    } else {
                        c*x
                    }
                }
                Self::SevenParameter{g,a,b, c, d, e, f} => {
                    if x>= d {
                        (a*x + b).powf(g) + e
                    } else {
                        c*x + f
                    }
                }
            }

        }
    }
}

pub struct Text(String);

#[derive(Debug, Serialize)]
#[serde(default)]
pub struct TextDescription{
    pub ascii: String,
    #[serde(skip_serializing_if = "u32::is_zero")]
    pub unicode_language_code: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub unicode: String,
    #[serde(skip_serializing_if = "u16::is_zero")]
    pub scriptcode_code: u16,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub scriptcode: String,
}

impl TextDescription {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let n = read_be_u32(buf)? as usize;
        let ascii = read_ascii_string(buf, n)?;
        let unicode_language_code = read_be_u32(buf)?;
        let m = read_be_u32(buf)? as usize;
        let unicode = read_unicode_string(buf, m)?;
        let scriptcode_code = read_be_u16(buf)?;
        let l = read_u8(buf)? as usize;
        let scriptcode= read_ascii_string(buf, l)?;
        Ok(TextDescription{
            ascii,
            unicode_language_code,
            unicode,
            scriptcode_code,
            scriptcode
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ViewingConditions {
    pub xyz_illuminant: [f64;3],
    pub xyz_surround: [f64;3],
    pub illuminant: StandardIlluminant,
}


impl ViewingConditions {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        Ok(ViewingConditions{
            xyz_illuminant: read_xyz(buf)?.unwrap_or([0.0, 0.0, 0.0]),
            xyz_surround: read_xyz(buf)?.unwrap_or([0.0, 0.0, 0.0]),
            illuminant: FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct XYZ(Vec<[f64;3]>);


impl TagData {
    pub fn try_new(tag_signature: TagSignature, type_signature: TagTypeSignature, buf: &mut &[u8]) -> Result<Self, Box< dyn std::error::Error + 'static>> {
        match (tag_signature, type_signature) {
            (_, TagTypeSignature::ColorantOrderType) => Ok(Self::ColorantOrder(ColorantOrder(buf.to_owned()))),
            (_, TagTypeSignature::CurveType) => {
                let n = read_be_u32(buf)? as usize;
                let mut v: Vec<u16> = Vec::with_capacity(n);
                for _ in 0..n {
                    v.push(read_be_u16(buf)?);
                }
                Ok(Self::Curve(Curve(v)))
            }
            (_, TagTypeSignature::DataType) => {
                let _n = read_be_u32(buf)? as usize;
                Ok(Self::Data(Data(buf.to_owned())))
            },
            (_, TagTypeSignature::DateTimeType) => {
                Ok(Self::DateTime(DateTime(read_date_time(buf)?.unwrap())))
            },
            (_, TagTypeSignature::Float16ArrayType)=> {
                let mut v = Vec::with_capacity(buf.len()/std::mem::size_of::<half::f16>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f16(buf)?)
                }
                Ok(Self::Float16Array(v))
            },
            (_, TagTypeSignature::Float32ArrayType) => {
                let mut v = Vec::with_capacity(buf.len()/std::mem::size_of::<f32>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f32(buf)?)
                }
                Ok(Self::Float32Array(v))
            },
            (_, TagTypeSignature::Float64ArrayType) => {
                let mut v = Vec::with_capacity(buf.len()/std::mem::size_of::<f64>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f64(buf)?)
                }
                Ok(Self::Float64Array(v))
            },
            (_, TagTypeSignature::MeasurementType) => {
                Ok(Self::Measurement(Measurement::try_new(buf)?))
            },
            (_, TagTypeSignature::ParametricCurveType) => {
                Ok(Self::ParametricCurve(ParametricCurve::try_new(buf)?))
            },
            (_, TagTypeSignature::S15Fixed16ArrayType) => {
                Ok(Self::S15Fixed16Array(read_s15fixed16_array(buf, None)?))
            },
            (_, TagTypeSignature::TextType) => {
                Ok(Self::Text(std::str::from_utf8(buf)?.trim_end_matches(char::from(0)).to_owned()))
            },
            (_, TagTypeSignature::TextDescriptionType) => {
                Ok(Self::TextDescription(TextDescription::try_new(buf)?))
            },
            (_, TagTypeSignature::ViewingConditionsType) => {
                Ok(Self::ViewingConditions(ViewingConditions::try_new(buf)?))
            },
            (_, TagTypeSignature::XYZArrayType) => {
                let n = buf.len()/12;
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    if let Some(xyz) = read_xyz(buf)? {
                        v.push(xyz);
                    } else {
                        v.push([0.0, 0.0, 0.0]);
                    }
                }
                Ok(Self::XYZ(XYZ(v)))

            },
            (TagSignature::TechnologyTag, TagTypeSignature::SignatureType) => {
                Ok(Self::Technology(FromPrimitive::from_u32(read_be_u32(buf)?).unwrap_or_default()))
            },
            _  => Ok(Self::Custom(type_signature, buf.to_owned())),
        } 
    }
}



#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum TagTypeSignature {
    UndefinedType                  = 0x00000000,
    ChromaticityType               = 0x6368726D,  /* 'chrm' */
    ColorantOrderType              = 0x636C726F,  /* 'clro' */
    ColorantTableType              = 0x636C7274,  /* 'clrt' */
    CrdInfoType                    = 0x63726469,  /* 'crdi' Removed in V4 */
    CurveType                      = 0x63757276,  /* 'curv' */
    DataType                       = 0x64617461,  /* 'data' */
    DictType                       = 0x64696374,  /* 'dict' */
    DateTimeType                   = 0x6474696D,  /* 'dtim' */
    DeviceSettingsType             = 0x64657673,  /* 'devs' Removed in V4 */
    EmbeddedHeightImageType        = 0x6568696D,  /* 'ehim' */
    EmbeddedNormalImageType        = 0x656e696d,  /* 'enim' */
    Float16ArrayType               = 0x666c3136,  /* 'fl16' */
    Float32ArrayType               = 0x666c3332,  /* 'fl32' */
    Float64ArrayType               = 0x666c3634,  /* 'fl64' */
    GamutBoundaryDescType	       = 0x67626420,  /* 'gbd ' */
    Lut16Type                      = 0x6d667432,  /* 'mft2' */
    Lut8Type                       = 0x6d667431,  /* 'mft1' */
    LutAtoBType                    = 0x6d414220,  /* 'mAB ' */
    LutBtoAType                    = 0x6d424120,  /* 'mBA ' */
    MeasurementType                = 0x6D656173,  /* 'meas' */
    MultiLocalizedUnicodeType      = 0x6D6C7563,  /* 'mluc' */
    MultiProcessElementType        = 0x6D706574,  /* 'mpet' */
    NamedColor2Type                = 0x6E636C32,  /* 'ncl2' use v2-v4*/
    ParametricCurveType            = 0x70617261,  /* 'para' */
    ProfileSequenceDescType        = 0x70736571,  /* 'pseq' */
    ProfileSequceIdType            = 0x70736964,  /* 'psid' */
    ResponseCurveSet16Type         = 0x72637332,  /* 'rcs2' */
    S15Fixed16ArrayType            = 0x73663332,  /* 'sf32' */
    ScreeningType                  = 0x7363726E,  /* 'scrn' Removed in V4 */
    SegmentedCurveType             = 0x63757266,  /* 'curf' */
    SignatureType                  = 0x73696720,  /* 'sig ' */
    SparseMatrixArrayType          = 0x736D6174,  /* 'smat' */
    SpectralViewingConditionsType  = 0x7376636e,  /* 'svcn' */
    SpectralDataInfoType           = 0x7364696e,  /* 'sdin' */
    TagArrayType                   = 0x74617279,  /* 'tary' */
    TagStructType                  = 0x74737472,  /* 'tstr' */
    TextType                       = 0x74657874,  /* 'text' */
    TextDescriptionType            = 0x64657363,  /* 'desc' Removed in V4 */
    U16Fixed16ArrayType            = 0x75663332,  /* 'uf32' */
    UcrBgType                      = 0x62666420,  /* 'bfd ' Removed in V4 */
    UInt16ArrayType                = 0x75693136,  /* 'ui16' */
    UInt32ArrayType                = 0x75693332,  /* 'ui32' */
    UInt64ArrayType                = 0x75693634,  /* 'ui64' */
    UInt8ArrayType                 = 0x75693038,  /* 'ui08' */
    ViewingConditionsType          = 0x76696577,  /* 'view' */
    Utf8TextType                   = 0x75746638,  /* 'utf8' */
    Utf16TextType                  = 0x75743136,  /* 'ut16' */
    /* XYZType                      = 0x58595A20, // 'XYZ ' Name changed to XYZArrayType */ 
    XYZArrayType                   = 0x58595A20,  /* 'XYZ ' */
    ZipUtf8TextType                = 0x7a757438,  /* 'zut8' */
    ZipXmlType                     = 0x5a584d4c,  /* 'ZXML' */      
    EmbeddedProfileType            = 0x49434370,  /* 'ICCp' */
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum StandardIlluminant {
    Unknown                 = 0x00000000,
    D50                     = 0x00000001,
    D65                     = 0x00000002,
    D93                     = 0x00000003,
    F2                      = 0x00000004,
    D55                     = 0x00000005,
    A                       = 0x00000006,
    EquiPowerE              = 0x00000007,  /* Equi-Power (E) */
    F8                      = 0x00000008,

    /* The following illuminants are defined for V5 */
    BlackBody               = 0x00000009,  /* defined by CCT */
    Daylight                = 0x0000000A,  /* defiend by CCT */
    B                       = 0x0000000B,
    C                       = 0x0000000C,
    F1                      = 0x0000000D,
    F3                      = 0x0000000E,
    F4                      = 0x0000000F,
    F5                      = 0x00000010,
    F6                      = 0x00000011,
    F7                      = 0x00000012,
    F9                      = 0x00000013,
    F10                     = 0x00000014,
    F11                     = 0x00000015,
    F12                     = 0x00000016,
}

impl Default for StandardIlluminant {
    fn default() -> Self {
        Self::Unknown
    }
} 

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum TechnologySignature {
    Unknown                        = 0x00000000, 
    DigitalCamera                  = 0x6463616D,  /* 'dcam' */
    FilmScanner                    = 0x6673636E,  /* 'fscn' */
    ReflectiveScanner              = 0x7273636E,  /* 'rscn' */
    InkJetPrinter                  = 0x696A6574,  /* 'ijet' */ 
    ThermalWaxPrinter              = 0x74776178,  /* 'twax' */
    ElectrophotographicPrinter     = 0x6570686F,  /* 'epho' */
    ElectrostaticPrinter           = 0x65737461,  /* 'esta' */
    DyeSublimationPrinter          = 0x64737562,  /* 'dsub' */
    PhotographicPaperPrinter       = 0x7270686F,  /* 'rpho' */
    FilmWriter                     = 0x6670726E,  /* 'fprn' */
    VideoMonitor                   = 0x7669646D,  /* 'vidm' */
    VideoCamera                    = 0x76696463,  /* 'vidc' */
    ProjectionTelevision           = 0x706A7476,  /* 'pjtv' */
    CRTDisplay                     = 0x43525420,  /* 'CRT ' */
    PMDisplay                      = 0x504D4420,  /* 'PMD ' */
    AMDisplay                      = 0x414D4420,  /* 'AMD ' */
    PhotoCD                        = 0x4B504344,  /* 'KPCD' */
    PhotoImageSetter               = 0x696D6773,  /* 'imgs' */
    Gravure                        = 0x67726176,  /* 'grav' */
    OffsetLithography              = 0x6F666673,  /* 'offs' */
    Silkscreen                     = 0x73696C6B,  /* 'silk' */
    Flexography                    = 0x666C6578,  /* 'flex' */
    MotionPictureFilmScanner       = 0x6D706673,  /* 'mpfs' */
    MotionPictureFilmRecorder      = 0x6D706672,  /* 'mpfr' */
    DigitalMotionPictureCamera     = 0x646D7063,  /* 'dmpc' */
    DigitalCinemaProjector         = 0x64636A70,  /* 'dcpj' */
}

impl Default for TechnologySignature {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum StandardObserver {
    Unknown                   = 0x00000000,  /* Unknown observer */
    Cie1931TwoDegrees         = 0x00000001,  /* 1931 two degrees */
    Cie1964TenDegrees         = 0x00000002,  /* 1961 ten degrees */
}

impl Default for StandardObserver {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum Geometry {
    Unknown                   = 0x00000000,  /* Unknown geometry */
    Normal45                  = 0x00000001,  /* 0/45, 45/0 */
    NormalDiffuse             = 0x00000002,  /* 0/d or d/0 */
}

impl Default for Geometry {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum Flare {
    Flare0                            = 0x00000000,  /* 0% flare */
    Flare100                          = 0x00000001,  /* 100% flare */
}

impl Default for Flare {
    fn default() -> Self {
        Self::Flare0
    }
}