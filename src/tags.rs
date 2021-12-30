use crate::profile::{read_be_f16, read_be_f32, read_be_f64, read_be_u32, read_be_u16, read_date_time, read_xyz};
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Tag {
    tag_signature: TagSignature,
    data_signature: TagTypeSignature,
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
    Custom(TagTypeSignature, Vec<u8>), // unknown data type
}

impl Tag {
    pub fn try_new(tag_signature: TagSignature, buf: &mut &[u8]) -> Result<Self, Box< dyn std::error::Error + 'static>> {
        let data_signature = match FromPrimitive::from_u32(read_be_u32(buf)?) {
            Some(c) => c,
            None => TagTypeSignature::UndefinedType,
        };
        let _reserved = read_be_u32(buf)?;
        Ok(Self {
            tag_signature,
            data_signature,
            data: TagData::try_new(data_signature, buf)?,
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
pub struct XYZ(Vec<[f64;3]>);


impl TagData {
    pub fn try_new(sig: TagTypeSignature, buf: &mut &[u8]) -> Result<Self, Box< dyn std::error::Error + 'static>> {
      //  let sig = read_signature(buf)?.ok_or("illegal data type")?;
       // let _reserved = read_be_u32(buf)?;
        match sig {
            TagTypeSignature::ColorantOrderType => Ok(Self::ColorantOrder(ColorantOrder(buf.to_owned()))),
            TagTypeSignature::CurveType => {
                let n = read_be_u32(buf)? as usize;
                let mut v: Vec<u16> = Vec::with_capacity(n);
                for _ in 0..n {
                    v.push(read_be_u16(buf)?);
                }
                Ok(Self::Curve(Curve(v)))
            }
            TagTypeSignature::DataType => {
                let _n = read_be_u32(buf)? as usize;
                Ok(Self::Data(Data(buf.to_owned())))
            },
            TagTypeSignature::DateTimeType=> {
                Ok(Self::DateTime(DateTime(read_date_time(buf)?.unwrap())))
            },
            TagTypeSignature::Float16ArrayType=> {
                let mut v = Vec::with_capacity(buf.len()/std::mem::size_of::<half::f16>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f16(buf)?)
                }
                Ok(Self::Float16Array(v))
            },
            TagTypeSignature::Float32ArrayType => {
                let mut v = Vec::with_capacity(buf.len()/std::mem::size_of::<f32>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f32(buf)?)
                }
                Ok(Self::Float32Array(v))
            },
            TagTypeSignature::Float64ArrayType => {
                let mut v = Vec::with_capacity(buf.len()/std::mem::size_of::<f64>());
                for _ in 0..v.capacity() {
                    v.push(read_be_f64(buf)?)
                }
                Ok(Self::Float64Array(v))
            },
            TagTypeSignature::XYZArrayType => {
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

            }
            _ => Ok(Self::Custom(sig, buf.to_owned())),
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