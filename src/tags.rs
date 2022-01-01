
use crate::profile::*;
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
    S15Fixed16Array(Vec<f64>), // 'sf32'
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
            tag_signature,
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

pub enum TagSignature {
    Unknown                           = 0x0isize,
    AToB0Tag                          = 0x41324230,  /* 'A2B0' */ 
    AToB1Tag                          = 0x41324231,  /* 'A2B1' */
    AToB2Tag                          = 0x41324232,  /* 'A2B2' */ 
    AToB3Tag                          = 0x41324233,  /* 'A2B3' */
    AToM0Tag                          = 0x41324d30,  /* 'A2M0' */
//  BlueColorantTag                   = 0x6258595A,  /* 'bXYZ' Renamed to BlueMatrixColumnTag V2->V4? */
    BlueMatrixColumnTag               = 0x6258595A,  /* 'bXYZ' */
    BlueTRCTag                        = 0x62545243,  /* 'bTRC' */
    BrdfColorimetricParameter0Tag     = 0x62637030,  /* 'bcp0' */
    BrdfColorimetricParameter1Tag     = 0x62637031,  /* 'bcp1' */
    BrdfColorimetricParameter2Tag     = 0x62637032,  /* 'bcp2' */
    BrdfColorimetricParameter3Tag     = 0x62637033,  /* 'bcp3' */
    BrdfSpectralParameter0Tag         = 0x62737030,  /* 'bsp0' */
    BrdfSpectralParameter1Tag         = 0x62737031,  /* 'bsp1' */
    BrdfSpectralParameter2Tag         = 0x62737032,  /* 'bsp2' */
    BrdfSpectralParameter3Tag         = 0x62737033,  /* 'bsp3' */
    BRDFAToB0Tag                      = 0x62414230,  /* 'bAB0' */
    BRDFAToB1Tag                      = 0x62414231,  /* 'bAB1' */
    BRDFAToB2Tag                      = 0x62414232,  /* 'bAB2' */
    BRDFAToB3Tag                      = 0x62414233,  /* 'bAB3' */
    BRDFDToB0Tag                      = 0x62444230,  /* 'bDB0' */
    BRDFDToB1Tag                      = 0x62444231,  /* 'bDB1' */
    BRDFDToB2Tag                      = 0x62444232,  /* 'bDB2' */
    BRDFDToB3Tag                      = 0x62444233,  /* 'bDB3' */
    BRDFMToB0Tag                      = 0x624D4230,  /* 'bMB0' */
    BRDFMToB1Tag                      = 0x624D4231,  /* 'bMB1' */
    BRDFMToB2Tag                      = 0x624D4232,  /* 'bMB2' */
    BRDFMToB3Tag                      = 0x624D4233,  /* 'bMB3' */
    BRDFMToS0Tag                      = 0x624D5330,  /* 'bMS0' */
    BRDFMToS1Tag                      = 0x624D5331,  /* 'bMS1' */
    BRDFMToS2Tag                      = 0x624D5332,  /* 'bMS2' */
    BRDFMToS3Tag                      = 0x624D5333,  /* 'bMS3' */
    BToA0Tag                          = 0x42324130,  /* 'B2A0' */
    BToA1Tag                          = 0x42324131,  /* 'B2A1' */
    BToA2Tag                          = 0x42324132,  /* 'B2A2' */
    BToA3Tag                          = 0x42324133,  /* 'B2A3' */
    CalibrationDateTimeTag            = 0x63616C74,  /* 'calt' */
    CharTargetTag                     = 0x74617267,  /* 'targ' */ 
    ChromaticAdaptationTag            = 0x63686164,  /* 'chad' */
    ChromaticityTag                   = 0x6368726D,  /* 'chrm' */
    ColorEncodingParamsTag            = 0x63657074,  /* 'cept' */
    ColorSpaceNameTag                 = 0x63736e6d,  /* 'csnm' */
    ColorantInfoTag                   = 0x636c696e,  /* 'clin' */
    ColorantInfoOutTag                = 0x636c696f,  /* 'clio' */
    ColorantOrderTag                  = 0x636C726F,  /* 'clro' */
    ColorantOrderOutTag               = 0x636c6f6f,  /* 'cloo' */
    ColorantTableTag                  = 0x636C7274,  /* 'clrt' */
    ColorantTableOutTag               = 0x636C6F74,  /* 'clot' */
    ColorimetricIntentImageStateTag   = 0x63696973,  /* 'ciis' */
    CopyrightTag                      = 0x63707274,  /* 'cprt' */
    CrdInfoTag                        = 0x63726469,  /* 'crdi' Removed in V4 */
    CustomToStandardPccTag            = 0x63327370,  /* 'c2sp' */
    CxFTag                            = 0x43784620,  /* 'CxF ' */
    DataTag                           = 0x64617461,  /* 'data' Removed in V4 */
    DateTimeTag                       = 0x6474696D,  /* 'dtim' Removed in V4 */
    DeviceMediaWhitePointTag          = 0x646d7770,  /* 'dmwp' */
    DeviceMfgDescTag                  = 0x646D6E64,  /* 'dmnd' */
    DeviceModelDescTag                = 0x646D6464,  /* 'dmdd' */
    DeviceSettingsTag                 = 0x64657673,  /* 'devs' Removed in V4 */
    DToB0Tag                          = 0x44324230,  /* 'D2B0' */
    DToB1Tag                          = 0x44324231,  /* 'D2B1' */
    DToB2Tag                          = 0x44324232,  /* 'D2B2' */
    DToB3Tag                          = 0x44324233,  /* 'D2B3' */
    BToD0Tag                          = 0x42324430,  /* 'B2D0' */
    BToD1Tag                          = 0x42324431,  /* 'B2D1' */
    BToD2Tag                          = 0x42324432,  /* 'B2D2' */
    BToD3Tag                          = 0x42324433,  /* 'B2D3' */
    GamutTag                          = 0x67616D74,  /* 'gamt' */
    GamutBoundaryDescription0Tag      = 0x67626430,  /* 'gbd0' */
    GamutBoundaryDescription1Tag      = 0x67626431,  /* 'gbd1' */
    GamutBoundaryDescription2Tag      = 0x67626432,  /* 'gbd2' */
    GamutBoundaryDescription3Tag      = 0x67626433,  /* 'gbd3' */
    GrayTRCTag                        = 0x6b545243,  /* 'kTRC' */
//  GreenColorantTag                  = 0x6758595A,  /* 'gXYZ' Renamed to GreenMatrixColumnTag in V4 */
    GreenMatrixColumnTag              = 0x6758595A,  /* 'gXYZ' */
    GreenTRCTag                       = 0x67545243,  /* 'gTRC' */
    LuminanceTag                      = 0x6C756d69,  /* 'lumi' */
    MaterialDefaultValuesTag          = 0x6D647620,  /* 'mdv ' */
    MaterialTypeArrayTag              = 0x6d637461,  /* 'mcta' */
    MToA0Tag                          = 0x4d324130,  /* 'M2A0' */
    MToB0Tag                          = 0x4d324230,  /* 'M2B0' */
    MToB1Tag                          = 0x4d324231,  /* 'M2B1' */
    MToB2Tag                          = 0x4d324232,  /* 'M2B2' */
    MToB3Tag                          = 0x4d324233,  /* 'M2B3' */
    MToS0Tag                          = 0x4d325330,  /* 'M2S0' */
    MToS1Tag                          = 0x4d325331,  /* 'M2S1' */
    MToS2Tag                          = 0x4d325332,  /* 'M2S2' */
    MToS3Tag                          = 0x4d325333,  /* 'M2S3' */
    MeasurementTag                    = 0x6D656173,  /* 'meas' */
    MediaBlackPointTag                = 0x626B7074,  /* 'bkpt' */
    MediaWhitePointTag                = 0x77747074,  /* 'wtpt' */
    MetaDataTag                       = 0x6D657461,  /* 'meta' */
    NamedColorTag                     = 0x6E636f6C,  /* 'ncol' OBSOLETE, use ncl2 */
    NamedColorV5Tag                   = 0x6e6d636C,  /* 'nmcl' use for V5;  GH Added V5 to distinguish from V2 */
    NamedColor2Tag                    = 0x6E636C32,  /* 'ncl2' */
    OutputResponseTag                 = 0x72657370,  /* 'resp' */
    PerceptualRenderingIntentGamutTag = 0x72696730,  /* 'rig0' */
    Preview0Tag                       = 0x70726530,  /* 'pre0' */
    Preview1Tag                       = 0x70726531,  /* 'pre1' */
    Preview2Tag                       = 0x70726532,  /* 'pre2' */
    PrintConditionTag                 = 0x7074636e,  /* 'ptcn' */
    ProfileDescriptionTag             = 0x64657363,  /* 'desc' */
    ProfileSequenceDescTag            = 0x70736571,  /* 'pseq' */
    ProfileSequceIdTag                = 0x70736964,  /* 'psid' */
    Ps2CRD0Tag                        = 0x70736430,  /* 'psd0' Removed in V4 */
    Ps2CRD1Tag                        = 0x70736431,  /* 'psd1' Removed in V4 */
    Ps2CRD2Tag                        = 0x70736432,  /* 'psd2' Removed in V4 */
    Ps2CRD3Tag                        = 0x70736433,  /* 'psd3' Removed in V4 */
    Ps2CSATag                         = 0x70733273,  /* 'ps2s' Removed in V4 */
    Ps2RenderingIntentTag             = 0x70733269,  /* 'ps2i' Removed in V4 */
//  RedColorantTag                    = 0x7258595A,  /* 'rXYZ' Renamed ReadMatrixColumnTag in V4 */
    RedMatrixColumnTag                = 0x7258595A,  /* 'rXYZ' */
    RedTRCTag                         = 0x72545243,  /* 'rTRC' */
    ReferenceNameTag                  = 0x72666e6d,  /* 'rfnm' */
    SaturationRenderingIntentGamutTag = 0x72696732,  /* 'rig2' */
    ScreeningDescTag                  = 0x73637264,  /* 'scrd' Removed in V4 */
    ScreeningTag                      = 0x7363726E,  /* 'scrn' Removed in V4 */
    SpectralDataInfoTag               = 0x7364696e,  /* 'sdin' */
    SpectralWhitePointTag             = 0x73777074,  /* 'swpt' */
    SpectralViewingConditionsTag      = 0x7376636e,  /* 'svcn' */
    StandardToCustomPccTag            = 0x73326370,  /* 's2cp' */
    SurfaceMapTag                     = 0x736D6170,  /* 'smap' */
    TechnologyTag                     = 0x74656368,  /* 'tech' */
    UcrBgTag                          = 0x62666420,  /* 'bfd ' Removed in V4 */
    ViewingCondDescTag                = 0x76756564,  /* 'vued' */
    ViewingConditionsTag              = 0x76696577,  /* 'view' */
    EmbeddedV5ProfileTag              = 0x49434335,  /* 'ICC5' */
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