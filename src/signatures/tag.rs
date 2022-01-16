//use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(PartialEq, Clone, Debug, Serialize)]
pub enum TagSignature {
    VendorTag(String),
    AToB0Tag                          ,  /* 'A2B0' */ 
    AToB1Tag                          ,  /* 'A2B1' */
    AToB2Tag                          ,  /* 'A2B2' */ 
    AToB3Tag                          ,  /* 'A2B3' */
    AToM0Tag                          ,  /* 'A2M0' */
//  BlueColorantTag                   ,  /* 'bXYZ' Renamed to BlueMatrixColumnTag V2->V4? */
    BlueMatrixColumnTag               ,  /* 'bXYZ' */
    BlueTRCTag                        ,  /* 'bTRC' */
    BrdfColorimetricParameter0Tag     ,  /* 'bcp0' */
    BrdfColorimetricParameter1Tag     ,  /* 'bcp1' */
    BrdfColorimetricParameter2Tag     ,  /* 'bcp2' */
    BrdfColorimetricParameter3Tag     ,  /* 'bcp3' */
    BrdfSpectralParameter0Tag         ,  /* 'bsp0' */
    BrdfSpectralParameter1Tag         ,  /* 'bsp1' */
    BrdfSpectralParameter2Tag         ,  /* 'bsp2' */
    BrdfSpectralParameter3Tag         ,  /* 'bsp3' */
    BRDFAToB0Tag                      ,  /* 'bAB0' */
    BRDFAToB1Tag                      ,  /* 'bAB1' */
    BRDFAToB2Tag                      ,  /* 'bAB2' */
    BRDFAToB3Tag                      ,  /* 'bAB3' */
    BRDFDToB0Tag                      ,  /* 'bDB0' */
    BRDFDToB1Tag                      ,  /* 'bDB1' */
    BRDFDToB2Tag                      ,  /* 'bDB2' */
    BRDFDToB3Tag                      ,  /* 'bDB3' */
    BRDFMToB0Tag                      ,  /* 'bMB0' */
    BRDFMToB1Tag                      ,  /* 'bMB1' */
    BRDFMToB2Tag                      ,  /* 'bMB2' */
    BRDFMToB3Tag                      ,  /* 'bMB3' */
    BRDFMToS0Tag                      ,  /* 'bMS0' */
    BRDFMToS1Tag                      ,  /* 'bMS1' */
    BRDFMToS2Tag                      ,  /* 'bMS2' */
    BRDFMToS3Tag                      ,  /* 'bMS3' */
    BToA0Tag                          ,  /* 'B2A0' */
    BToA1Tag                          ,  /* 'B2A1' */
    BToA2Tag                          ,  /* 'B2A2' */
    BToA3Tag                          ,  /* 'B2A3' */
    CalibrationDateTimeTag            ,  /* 'calt' */
    CharTargetTag                     ,  /* 'targ' */ 
    ChromaticAdaptationTag            ,  /* 'chad' */
    ChromaticityTag                   ,  /* 'chrm' */
    ColorEncodingParamsTag            ,  /* 'cept' */
    ColorSpaceNameTag                 ,  /* 'csnm' */
    ColorantInfoTag                   ,  /* 'clin' */
    ColorantInfoOutTag                ,  /* 'clio' */
    ColorantOrderTag                  ,  /* 'clro' */
    ColorantOrderOutTag               ,  /* 'cloo' */
    ColorantTableTag                  ,  /* 'clrt' */
    ColorantTableOutTag               ,  /* 'clot' */
    ColorimetricIntentImageStateTag   ,  /* 'ciis' */
    CopyrightTag                      ,  /* 'cprt' */
    CrdInfoTag                        ,  /* 'crdi' Removed in V4 */
    CustomToStandardPccTag            ,  /* 'c2sp' */
    CxFTag                            ,  /* 'CxF ' */
    DataTag                           ,  /* 'data' Removed in V4 */
    DateTimeTag                       ,  /* 'dtim' Removed in V4 */
    DeviceMediaWhitePointTag          ,  /* 'dmwp' */
    DeviceMfgDescTag                  ,  /* 'dmnd' */
    DeviceModelDescTag                ,  /* 'dmdd' */
    DeviceSettingsTag                 ,  /* 'devs' Removed in V4 */
    DToB0Tag                          ,  /* 'D2B0' */
    DToB1Tag                          ,  /* 'D2B1' */
    DToB2Tag                          ,  /* 'D2B2' */
    DToB3Tag                          ,  /* 'D2B3' */
    BToD0Tag                          ,  /* 'B2D0' */
    BToD1Tag                          ,  /* 'B2D1' */
    BToD2Tag                          ,  /* 'B2D2' */
    BToD3Tag                          ,  /* 'B2D3' */
    GamutTag                          ,  /* 'gamt' */
    GamutBoundaryDescription0Tag      ,  /* 'gbd0' */
    GamutBoundaryDescription1Tag      ,  /* 'gbd1' */
    GamutBoundaryDescription2Tag      ,  /* 'gbd2' */
    GamutBoundaryDescription3Tag      ,  /* 'gbd3' */
    GrayTRCTag                        ,  /* 'kTRC' */
//  GreenColorantTag                  ,  /* 'gXYZ' Renamed to GreenMatrixColumnTag in V4 */
    GreenMatrixColumnTag              ,  /* 'gXYZ' */
    GreenTRCTag                       ,  /* 'gTRC' */
    LuminanceTag                      ,  /* 'lumi' */
    MaterialDefaultValuesTag          ,  /* 'mdv ' */
    MaterialTypeArrayTag              ,  /* 'mcta' */
    MToA0Tag                          ,  /* 'M2A0' */
    MToB0Tag                          ,  /* 'M2B0' */
    MToB1Tag                          ,  /* 'M2B1' */
    MToB2Tag                          ,  /* 'M2B2' */
    MToB3Tag                          ,  /* 'M2B3' */
    MToS0Tag                          ,  /* 'M2S0' */
    MToS1Tag                          ,  /* 'M2S1' */
    MToS2Tag                          ,  /* 'M2S2' */
    MToS3Tag                          ,  /* 'M2S3' */
    MeasurementTag                    ,  /* 'meas' */
    MediaBlackPointTag                ,  /* 'bkpt' */
    MediaWhitePointTag                ,  /* 'wtpt' */
    MetaDataTag                       ,  /* 'meta' */
    NamedColorTag                     ,  /* 'ncol' OBSOLETE, use ncl2 */
    NamedColorV5Tag                   ,  /* 'nmcl' use for V5;  GH Added V5 to distinguish from V2 */
    NamedColor2Tag                    ,  /* 'ncl2' */
    OutputResponseTag                 ,  /* 'resp' */
    PerceptualRenderingIntentGamutTag ,  /* 'rig0' */
    Preview0Tag                       ,  /* 'pre0' */
    Preview1Tag                       ,  /* 'pre1' */
    Preview2Tag                       ,  /* 'pre2' */
    PrintConditionTag                 ,  /* 'ptcn' */
    ProfileDescriptionTag             ,  /* 'desc' */
    ProfileSequenceDescTag            ,  /* 'pseq' */
    ProfileSequceIdTag                ,  /* 'psid' */
    Ps2CRD0Tag                        ,  /* 'psd0' Removed in V4 */
    Ps2CRD1Tag                        ,  /* 'psd1' Removed in V4 */
    Ps2CRD2Tag                        ,  /* 'psd2' Removed in V4 */
    Ps2CRD3Tag                        ,  /* 'psd3' Removed in V4 */
    Ps2CSATag                         ,  /* 'ps2s' Removed in V4 */
    Ps2RenderingIntentTag             ,  /* 'ps2i' Removed in V4 */
//  RedColorantTag                    ,  /* 'rXYZ' Renamed ReadMatrixColumnTag in V4 */
    RedMatrixColumnTag                ,  /* 'rXYZ' */
    RedTRCTag                         ,  /* 'rTRC' */
    ReferenceNameTag                  ,  /* 'rfnm' */
    SaturationRenderingIntentGamutTag ,  /* 'rig2' */
    ScreeningDescTag                  ,  /* 'scrd' Removed in V4 */
    ScreeningTag                      ,  /* 'scrn' Removed in V4 */
    SpectralDataInfoTag               ,  /* 'sdin' */
    SpectralWhitePointTag             ,  /* 'swpt' */
    SpectralViewingConditionsTag      ,  /* 'svcn' */
    StandardToCustomPccTag            ,  /* 's2cp' */
    SurfaceMapTag                     ,  /* 'smap' */
    TechnologyTag                     ,  /* 'tech' */
    UcrBgTag                          ,  /* 'bfd ' Removed in V4 */
    ViewingCondDescTag                ,  /* 'vued' */
    ViewingConditionsTag              ,  /* 'view' */
    EmbeddedV5ProfileTag              ,  /* 'ICC5' */

    // PRIVATE TAGS

    // Apple
    MakeAndModelTag                   , /* mmod */
    MultilocalizedDescriptionStringTag, /* dscm */
    NativeDisplayInfoTag              ,  /* 'ndin' */
    VcgtTag                           ,  /* 'vcgt' Not icc, defacto industry standard */
    VcgpTag                           ,  /* 'vcgp' Not icc, defacto industry standard */

    // ArgyllCMS
    AbsToRelTransSpaceTag             , /* arts */
}


impl TagSignature {
    pub fn new(sig: u32) -> Self {
        match sig {
            0x41324230 => Self::AToB0Tag,
            0x41324231 => Self::AToB1Tag,
            0x41324232 => Self::AToB2Tag,
            0x41324233 => Self::AToB3Tag,
            0x41324d30 => Self::AToM0Tag,
            0x6258595A => Self::BlueMatrixColumnTag,
            0x62545243 => Self::BlueTRCTag,
            0x62637030 => Self::BrdfColorimetricParameter0Tag,
            0x62637031 => Self::BrdfColorimetricParameter1Tag,
            0x62637032 => Self::BrdfColorimetricParameter2Tag,
            0x62637033 => Self::BrdfColorimetricParameter3Tag,
            0x62737030 => Self::BrdfSpectralParameter0Tag,
            0x62737031 => Self::BrdfSpectralParameter1Tag,
            0x62737032 => Self::BrdfSpectralParameter2Tag,
            0x62737033 => Self::BrdfSpectralParameter3Tag,
            0x62414230 => Self::BRDFAToB0Tag,
            0x62414231 => Self::BRDFAToB1Tag,
            0x62414232 => Self::BRDFAToB2Tag,
            0x62414233 => Self::BRDFAToB3Tag,
            0x62444230 => Self::BRDFDToB0Tag,
            0x62444231 => Self::BRDFDToB1Tag,
            0x62444232 => Self::BRDFDToB2Tag,
            0x62444233 => Self::BRDFDToB3Tag,
            0x624D4230 => Self::BRDFMToB0Tag,
            0x624D4231 => Self::BRDFMToB1Tag,
            0x624D4232 => Self::BRDFMToB2Tag,
            0x624D4233 => Self::BRDFMToB3Tag,
            0x624D5330 => Self::BRDFMToS0Tag,
            0x624D5331 => Self::BRDFMToS1Tag,
            0x624D5332 => Self::BRDFMToS2Tag,
            0x624D5333 => Self::BRDFMToS3Tag,
            0x42324130 => Self::BToA0Tag,
            0x42324131 => Self::BToA1Tag,
            0x42324132 => Self::BToA2Tag,
            0x42324133 => Self::BToA3Tag,
            0x63616C74 => Self::CalibrationDateTimeTag,
            0x74617267 => Self::CharTargetTag,
            0x63686164 => Self::ChromaticAdaptationTag,
            0x6368726D => Self::ChromaticityTag,
            0x63657074 => Self::ColorEncodingParamsTag,
            0x63736e6d => Self::ColorSpaceNameTag,
            0x636c696e => Self::ColorantInfoTag,
            0x636c696f => Self::ColorantInfoOutTag,
            0x636C726F => Self::ColorantOrderTag,
            0x636c6f6f => Self::ColorantOrderOutTag,
            0x636C7274 => Self::ColorantTableTag,
            0x636C6F74 => Self::ColorantTableOutTag,
            0x63696973 => Self::ColorimetricIntentImageStateTag,
            0x63707274 => Self::CopyrightTag,
            0x63726469 => Self::CrdInfoTag,
            0x63327370 => Self::CustomToStandardPccTag,
            0x43784620 => Self::CxFTag,
            0x64617461 => Self::DataTag,
            0x6474696D => Self::DateTimeTag,
            0x646d7770 => Self::DeviceMediaWhitePointTag,
            0x646D6E64 => Self::DeviceMfgDescTag,
            0x646D6464 => Self::DeviceModelDescTag,
            0x64657673 => Self::DeviceSettingsTag,
            0x44324230 => Self::DToB0Tag,
            0x44324231 => Self::DToB1Tag,
            0x44324232 => Self::DToB2Tag,
            0x44324233 => Self::DToB3Tag,
            0x42324430 => Self::BToD0Tag,
            0x42324431 => Self::BToD1Tag,
            0x42324432 => Self::BToD2Tag,
            0x42324433 => Self::BToD3Tag,
            0x67616D74 => Self::GamutTag,
            0x67626430 => Self::GamutBoundaryDescription0Tag,
            0x67626431 => Self::GamutBoundaryDescription1Tag,
            0x67626432 => Self::GamutBoundaryDescription2Tag,
            0x67626433 => Self::GamutBoundaryDescription3Tag,
            0x6b545243 => Self::GrayTRCTag,
            0x6758595A => Self::GreenMatrixColumnTag,
            0x67545243 => Self::GreenTRCTag,
            0x6C756d69 => Self::LuminanceTag,
            0x6D647620 => Self::MaterialDefaultValuesTag,
            0x6d637461 => Self::MaterialTypeArrayTag,
            0x4d324130 => Self::MToA0Tag,
            0x4d324230 => Self::MToB0Tag,
            0x4d324231 => Self::MToB1Tag,
            0x4d324232 => Self::MToB2Tag,
            0x4d324233 => Self::MToB3Tag,
            0x4d325330 => Self::MToS0Tag,
            0x4d325331 => Self::MToS1Tag,
            0x4d325332 => Self::MToS2Tag,
            0x4d325333 => Self::MToS3Tag,
            0x6D656173 => Self::MeasurementTag,
            0x626B7074 => Self::MediaBlackPointTag,
            0x77747074 => Self::MediaWhitePointTag,
            0x6D657461 => Self::MetaDataTag,
            0x6E636f6C => Self::NamedColorTag,
            0x6e6d636C => Self::NamedColorV5Tag,
            0x6E636C32 => Self::NamedColor2Tag,
            0x72657370 => Self::OutputResponseTag,
            0x72696730 => Self::PerceptualRenderingIntentGamutTag,
            0x70726530 => Self::Preview0Tag,
            0x70726531 => Self::Preview1Tag,
            0x70726532 => Self::Preview2Tag,
            0x7074636e => Self::PrintConditionTag,
            0x64657363 => Self::ProfileDescriptionTag,
            0x70736571 => Self::ProfileSequenceDescTag,
            0x70736964 => Self::ProfileSequceIdTag,
            0x70736430 => Self::Ps2CRD0Tag,
            0x70736431 => Self::Ps2CRD1Tag,
            0x70736432 => Self::Ps2CRD2Tag,
            0x70736433 => Self::Ps2CRD3Tag,
            0x70733273 => Self::Ps2CSATag,
            0x70733269 => Self::Ps2RenderingIntentTag,
            0x7258595A => Self::RedMatrixColumnTag,
            0x72545243 => Self::RedTRCTag,
            0x72666e6d => Self::ReferenceNameTag,
            0x72696732 => Self::SaturationRenderingIntentGamutTag,
            0x73637264 => Self::ScreeningDescTag,
            0x7363726E => Self::ScreeningTag,
            0x7364696e => Self::SpectralDataInfoTag,
            0x73777074 => Self::SpectralWhitePointTag,
            0x7376636e => Self::SpectralViewingConditionsTag,
            0x73326370 => Self::StandardToCustomPccTag,
            0x736D6170 => Self::SurfaceMapTag,
            0x74656368 => Self::TechnologyTag,
            0x62666420 => Self::UcrBgTag,
            0x76756564 => Self::ViewingCondDescTag,
            0x76696577 => Self::ViewingConditionsTag,
            0x49434335 => Self::EmbeddedV5ProfileTag,

            // PRIVATE TAGS

            // Apple
            0x6d6d6f64 => Self::MakeAndModelTag,
            0x6473636d => Self::MultilocalizedDescriptionStringTag, 
            0x6e64696e => Self::NativeDisplayInfoTag,
            0x76636770 => Self::VcgpTag, /* Video Card Graphics Table  */
            0x76636774 => Self::VcgtTag, /* Video Card Graphics Table  */


            // ArgyllCMS
            0x61727473 => Self::AbsToRelTransSpaceTag, // https://www.argyllcms.com/doc/ArgyllCMS_arts_tag.html

            _ => Self::VendorTag(std::str::from_utf8(&sig.to_be_bytes()).unwrap().to_owned())

        }
    }
}

#[test]
fn test_str_to_u32(){
    let s = "vcgp";
    let v = u32::from_be_bytes(s.as_bytes().try_into().unwrap());
    println!("{:?} {:?} {:x?}", s, v, v);


}
