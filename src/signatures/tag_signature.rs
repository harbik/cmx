use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[repr(u32)]
pub enum TagSignature {
    AToB0Tag = 0x41324230,                           // 'A2B0'
    AToB1Tag = 0x41324231,                           // 'A2B1'
    AToB2Tag = 0x41324232,                           // 'A2B2'
    AToB3Tag = 0x41324233,                           // 'A2B3'
    AToM0Tag = 0x41324D30,                           // 'A2M0'
    BlueMatrixColumnTag = 0x6258595A,                // 'bXYZ'
    BlueTRCTag = 0x62545243,                         // 'bTRC'
    BrdfColorimetricParameter0Tag = 0x62637030,      // 'bcp0'
    BrdfColorimetricParameter1Tag = 0x62637031,      // 'bcp1'
    BrdfColorimetricParameter2Tag = 0x62637032,      // 'bcp2'
    BrdfColorimetricParameter3Tag = 0x62637033,      // 'bcp3'
    BrdfSpectralParameter0Tag = 0x62737030,          // 'bsp0'
    BrdfSpectralParameter1Tag = 0x62737031,          // 'bsp1'
    BrdfSpectralParameter2Tag = 0x62737032,          // 'bsp2'
    BrdfSpectralParameter3Tag = 0x62737033,          // 'bsp3'
    BRDFAToB0Tag = 0x62414230,                       // 'bAB0'
    BRDFAToB1Tag = 0x62414231,                       // 'bAB1'
    BRDFAToB2Tag = 0x62414232,                       // 'bAB2'
    BRDFAToB3Tag = 0x62414233,                       // 'bAB3'
    BRDFDToB0Tag = 0x62444230,                       // 'bDB0'
    BRDFDToB1Tag = 0x62444231,                       // 'bDB1'
    BRDFDToB2Tag = 0x62444232,                       // 'bDB2'
    BRDFDToB3Tag = 0x62444233,                       // 'bDB3'
    BRDFMToB0Tag = 0x624D4230,                       // 'bMB0'
    BRDFMToB1Tag = 0x624D4231,                       // 'bMB1'
    BRDFMToB2Tag = 0x624D4232,                       // 'bMB2'
    BRDFMToB3Tag = 0x624D4233,                       // 'bMB3'
    BRDFMToS0Tag = 0x624D5330,                       // 'bMS0'
    BRDFMToS1Tag = 0x624D5331,                       // 'bMS1'
    BRDFMToS2Tag = 0x624D5332,                       // 'bMS2'
    BRDFMToS3Tag = 0x624D5333,                       // 'bMS3'
    BToA0Tag = 0x42324130,                           // 'B2A0'
    BToA1Tag = 0x42324131,                           // 'B2A1'
    BToA2Tag = 0x42324132,                           // 'B2A2'
    BToA3Tag = 0x42324133,                           // 'B2A3'
    CalibrationDateTimeTag = 0x63616C74,             // 'calt'
    CharTargetTag = 0x74617267,                      // 'targ'
    ChromaticAdaptationTag = 0x63686164,             // 'chad'
    ChromaticityTag = 0x6368726D,                    // 'chrm'
    ColorEncodingParamsTag = 0x63657074,             // 'cept'
    ColorSpaceNameTag = 0x63736E6D,                  // 'csnm'
    ColorantInfoTag = 0x636C696E,                    // 'clin'
    ColorantInfoOutTag = 0x636C696F,                 // 'clio'
    ColorantOrderTag = 0x636C726F,                   // 'clro'
    ColorantOrderOutTag = 0x636C6F6F,                // 'cloo'
    ColorantTableTag = 0x636C7274,                   // 'clrt'
    ColorantTableOutTag = 0x636C6F74,                // 'clot'
    ColorimetricIntentImageStateTag = 0x63696973,    // 'ciis'
    CopyrightTag = 0x63707274,                       // 'cprt'
    CrdInfoTag = 0x63726469,                         // 'crdi'
    CustomToStandardPccTag = 0x63327370,             // 'c2sp'
    CxFTag = 0x43784620,                             // 'CxF '
    DataTag = 0x64617461,                            // 'data'
    DateTimeTag = 0x6474696D,                        // 'dtim'
    DeviceMediaWhitePointTag = 0x646D7770,           // 'dmwp'
    DeviceMfgDescTag = 0x646D6E64,                   // 'dmnd'
    DeviceModelDescTag = 0x646D6464,                 // 'dmdd'
    DeviceSettingsTag = 0x64657673,                  // 'devs'
    DToB0Tag = 0x44324230,                           // 'D2B0'
    DToB1Tag = 0x44324231,                           // 'D2B1'
    DToB2Tag = 0x44324232,                           // 'D2B2'
    DToB3Tag = 0x44324233,                           // 'D2B3'
    BToD0Tag = 0x42324430,                           // 'B2D0'
    BToD1Tag = 0x42324431,                           // 'B2D1'
    BToD2Tag = 0x42324432,                           // 'B2D2'
    BToD3Tag = 0x42324433,                           // 'B2D3'
    GamutTag = 0x67616D74,                           // 'gamt'
    GamutBoundaryDescription0Tag = 0x67626430,       // 'gbd0'
    GamutBoundaryDescription1Tag = 0x67626431,       // 'gbd1'
    GamutBoundaryDescription2Tag = 0x67626432,       // 'gbd2'
    GamutBoundaryDescription3Tag = 0x67626433,       // 'gbd3'
    GrayTRCTag = 0x6B545243,                         // 'kTRC'
    GreenMatrixColumnTag = 0x6758595A,               // 'gXYZ'
    GreenTRCTag = 0x67545243,                        // 'gTRC'
    LuminanceTag = 0x6C756D69,                       // 'lumi'
    MaterialDefaultValuesTag = 0x6D647620,           // 'mdv '
    MaterialTypeArrayTag = 0x6D637461,               // 'mcta'
    MToA0Tag = 0x4D324130,                           // 'M2A0'
    MToB0Tag = 0x4D324230,                           // 'M2B0'
    MToB1Tag = 0x4D324231,                           // 'M2B1'
    MToB2Tag = 0x4D324232,                           // 'M2B2'
    MToB3Tag = 0x4D324233,                           // 'M2B3'
    MToS0Tag = 0x4D325330,                           // 'M2S0'
    MToS1Tag = 0x4D325331,                           // 'M2S1'
    MToS2Tag = 0x4D325332,                           // 'M2S2'
    MToS3Tag = 0x4D325333,                           // 'M2S3'
    MeasurementTag = 0x6D656173,                     // 'meas'
    MediaBlackPointTag = 0x626B7074,                 // 'bkpt'
    MediaWhitePointTag = 0x77747074,                 // 'wtpt'
    MetaDataTag = 0x6D657461,                        // 'meta'
    NamedColorTag = 0x6E636F6C,                      // 'ncol'
    NamedColorV5Tag = 0x6E6D636C,                    // 'nmcl'
    NamedColor2Tag = 0x6E636C32,                     // 'ncl2'
    OutputResponseTag = 0x72657370,                  // 'resp'
    PerceptualRenderingIntentGamutTag = 0x72696730,  // 'rig0'
    Preview0Tag = 0x70726530,                        // 'pre0'
    Preview1Tag = 0x70726531,                        // 'pre1'
    Preview2Tag = 0x70726532,                        // 'pre2'
    PrintConditionTag = 0x7074636E,                  // 'ptcn'
    ProfileDescriptionTag = 0x64657363,              // 'desc'
    ProfileSequenceDescTag = 0x70736571,             // 'pseq'
    ProfileSequceIdTag = 0x70736964,                 // 'psid'
    Ps2CRD0Tag = 0x70736430,                         // 'psd0'
    Ps2CRD1Tag = 0x70736431,                         // 'psd1'
    Ps2CRD2Tag = 0x70736432,                         // 'psd2'
    Ps2CRD3Tag = 0x70736433,                         // 'psd3'
    Ps2CSATag = 0x70733273,                          // 'ps2s'
    Ps2RenderingIntentTag = 0x70733269,              // 'ps2i'
    RedMatrixColumnTag = 0x7258595A,                 // 'rXYZ'
    RedTRCTag = 0x72545243,                          // 'rTRC'
    ReferenceNameTag = 0x72666E6D,                   // 'rfnm'
    SaturationRenderingIntentGamutTag = 0x72696732,  // 'rig2'
    ScreeningDescTag = 0x73637264,                   // 'scrd'
    ScreeningTag = 0x7363726E,                       // 'scrn'
    SpectralDataInfoTag = 0x7364696E,                // 'sdin'
    SpectralWhitePointTag = 0x73777074,              // 'swpt'
    SpectralViewingConditionsTag = 0x7376636E,       // 'svcn'
    StandardToCustomPccTag = 0x73326370,             // 's2cp'
    SurfaceMapTag = 0x736D6170,                      // 'smap'
    TechnologyTag = 0x74656368,                      // 'tech'
    UcrBgTag = 0x62666420,                           // 'bfd '
    ViewingCondDescTag = 0x76756564,                 // 'vued'
    ViewingConditionsTag = 0x76696577,               // 'view'
    EmbeddedV5ProfileTag = 0x49434335,               // 'ICC5'
    MakeAndModelTag = 0x6D6D6F64,                    // 'mmod'
    MultilocalizedDescriptionStringTag = 0x6473636D, // 'dscm'
    NativeDisplayInfoTag = 0x6E64696E,               // 'ndin'
    VcgtTag = 0x76636774,                            // 'vcgt'
    VcgpTag = 0x76636770,                            // 'vcgp'
    AbsToRelTransSpaceTag = 0x61727473,              // 'arts'
    Unknown(u32),
}

impl TagSignature {
    /// Creates a new `TagSignature` from a u32 value.
    pub fn new(tag: u32) -> Self {
        match tag {
            0x41324230 => Self::AToB0Tag,
            0x41324231 => Self::AToB1Tag,
            0x41324232 => Self::AToB2Tag,
            0x41324233 => Self::AToB3Tag,
            0x41324D30 => Self::AToM0Tag,
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
            0x63736E6D => Self::ColorSpaceNameTag,
            0x636C696E => Self::ColorantInfoTag,
            0x636C696F => Self::ColorantInfoOutTag,
            0x636C726F => Self::ColorantOrderTag,
            0x636C6F6F => Self::ColorantOrderOutTag,
            0x636C7274 => Self::ColorantTableTag,
            0x636C6F74 => Self::ColorantTableOutTag,
            0x63696973 => Self::ColorimetricIntentImageStateTag,
            0x63707274 => Self::CopyrightTag,
            0x63726469 => Self::CrdInfoTag,
            0x63327370 => Self::CustomToStandardPccTag,
            0x43784620 => Self::CxFTag,
            0x64617461 => Self::DataTag,
            0x6474696D => Self::DateTimeTag,
            0x646D7770 => Self::DeviceMediaWhitePointTag,
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
            0x6B545243 => Self::GrayTRCTag,
            0x6758595A => Self::GreenMatrixColumnTag,
            0x67545243 => Self::GreenTRCTag,
            0x6C756D69 => Self::LuminanceTag,
            0x6D647620 => Self::MaterialDefaultValuesTag,
            0x6D637461 => Self::MaterialTypeArrayTag,
            0x4D324130 => Self::MToA0Tag,
            0x4D324230 => Self::MToB0Tag,
            0x4D324231 => Self::MToB1Tag,
            0x4D324232 => Self::MToB2Tag,
            0x4D324233 => Self::MToB3Tag,
            0x4D325330 => Self::MToS0Tag,
            0x4D325331 => Self::MToS1Tag,
            0x4D325332 => Self::MToS2Tag,
            0x4D325333 => Self::MToS3Tag,
            0x6D656173 => Self::MeasurementTag,
            0x626B7074 => Self::MediaBlackPointTag,
            0x77747074 => Self::MediaWhitePointTag,
            0x6D657461 => Self::MetaDataTag,
            0x6E636F6C => Self::NamedColorTag,
            0x6E6D636C => Self::NamedColorV5Tag,
            0x6E636C32 => Self::NamedColor2Tag,
            0x72657370 => Self::OutputResponseTag,
            0x72696730 => Self::PerceptualRenderingIntentGamutTag,
            0x70726530 => Self::Preview0Tag,
            0x70726531 => Self::Preview1Tag,
            0x70726532 => Self::Preview2Tag,
            0x7074636E => Self::PrintConditionTag,
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
            0x72666E6D => Self::ReferenceNameTag,
            0x72696732 => Self::SaturationRenderingIntentGamutTag,
            0x73637264 => Self::ScreeningDescTag,
            0x7363726E => Self::ScreeningTag,
            0x7364696E => Self::SpectralDataInfoTag,
            0x73777074 => Self::SpectralWhitePointTag,
            0x7376636E => Self::SpectralViewingConditionsTag,
            0x73326370 => Self::StandardToCustomPccTag,
            0x736D6170 => Self::SurfaceMapTag,
            0x74656368 => Self::TechnologyTag,
            0x62666420 => Self::UcrBgTag,
            0x76756564 => Self::ViewingCondDescTag,
            0x76696577 => Self::ViewingConditionsTag,
            0x49434335 => Self::EmbeddedV5ProfileTag,
            0x6D6D6F64 => Self::MakeAndModelTag,
            0x6473636D => Self::MultilocalizedDescriptionStringTag,
            0x6E64696E => Self::NativeDisplayInfoTag,
            0x76636774 => Self::VcgtTag,
            0x76636770 => Self::VcgpTag,
            0x61727473 => Self::AbsToRelTransSpaceTag,
            _ => Self::Unknown(tag),
        }
    }
}

impl std::fmt::Display for TagSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = self.to_u32();
        let bytes = value.to_be_bytes();
        let s = String::from_utf8_lossy(&bytes);
        if s.is_ascii() && s.len() == 4 {
            write!(f, "{}", s)
        } else {
            write!(f, "{:08X}", value)
        }
    }
}

impl TagSignature {
    pub fn to_u32(&self) -> u32 {
        match self {
            TagSignature::AToB0Tag => 0x41324230,
            TagSignature::AToB1Tag => 0x41324231,
            TagSignature::AToB2Tag => 0x41324232,
            TagSignature::AToB3Tag => 0x41324233,
            TagSignature::AToM0Tag => 0x41324D30,
            TagSignature::BlueMatrixColumnTag => 0x6258595A,
            TagSignature::BlueTRCTag => 0x62545243,
            TagSignature::BrdfColorimetricParameter0Tag => 0x62637030,
            TagSignature::BrdfColorimetricParameter1Tag => 0x62637031,
            TagSignature::BrdfColorimetricParameter2Tag => 0x62637032,
            TagSignature::BrdfColorimetricParameter3Tag => 0x62637033,
            TagSignature::BrdfSpectralParameter0Tag => 0x62737030,
            TagSignature::BrdfSpectralParameter1Tag => 0x62737031,
            TagSignature::BrdfSpectralParameter2Tag => 0x62737032,
            TagSignature::BrdfSpectralParameter3Tag => 0x62737033,
            TagSignature::BRDFAToB0Tag => 0x62414230,
            TagSignature::BRDFAToB1Tag => 0x62414231,
            TagSignature::BRDFAToB2Tag => 0x62414232,
            TagSignature::BRDFAToB3Tag => 0x62414233,
            TagSignature::BRDFDToB0Tag => 0x62444230,
            TagSignature::BRDFDToB1Tag => 0x62444231,
            TagSignature::BRDFDToB2Tag => 0x62444232,
            TagSignature::BRDFDToB3Tag => 0x62444233,
            TagSignature::BRDFMToB0Tag => 0x624D4230,
            TagSignature::BRDFMToB1Tag => 0x624D4231,
            TagSignature::BRDFMToB2Tag => 0x624D4232,
            TagSignature::BRDFMToB3Tag => 0x624D4233,
            TagSignature::BRDFMToS0Tag => 0x624D5330,
            TagSignature::BRDFMToS1Tag => 0x624D5331,
            TagSignature::BRDFMToS2Tag => 0x624D5332,
            TagSignature::BRDFMToS3Tag => 0x624D5333,
            TagSignature::BToA0Tag => 0x42324130,
            TagSignature::BToA1Tag => 0x42324131,
            TagSignature::BToA2Tag => 0x42324132,
            TagSignature::BToA3Tag => 0x42324133,
            TagSignature::CalibrationDateTimeTag => 0x63616C74,
            TagSignature::CharTargetTag => 0x74617267,
            TagSignature::ChromaticAdaptationTag => 0x63686164,
            TagSignature::ChromaticityTag => 0x6368726D,
            TagSignature::ColorEncodingParamsTag => 0x63657074,
            TagSignature::ColorSpaceNameTag => 0x63736E6D,
            TagSignature::ColorantInfoTag => 0x636C696E,
            TagSignature::ColorantInfoOutTag => 0x636C696F,
            TagSignature::ColorantOrderTag => 0x636C726F,
            TagSignature::ColorantOrderOutTag => 0x636C6F6F,
            TagSignature::ColorantTableTag => 0x636C7274,
            TagSignature::ColorantTableOutTag => 0x636C6F74,
            TagSignature::ColorimetricIntentImageStateTag => 0x63696973,
            TagSignature::CopyrightTag => 0x63707274,
            TagSignature::CrdInfoTag => 0x63726469,
            TagSignature::CustomToStandardPccTag => 0x63327370,
            TagSignature::CxFTag => 0x43784620,
            TagSignature::DataTag => 0x64617461,
            TagSignature::DateTimeTag => 0x6474696D,
            TagSignature::DeviceMediaWhitePointTag => 0x646D7770,
            TagSignature::DeviceMfgDescTag => 0x646D6E64,
            TagSignature::DeviceModelDescTag => 0x646D6464,
            TagSignature::DeviceSettingsTag => 0x64657673,
            TagSignature::DToB0Tag => 0x44324230,
            TagSignature::DToB1Tag => 0x44324231,
            TagSignature::DToB2Tag => 0x44324232,
            TagSignature::DToB3Tag => 0x44324233,
            TagSignature::BToD0Tag => 0x42324430,
            TagSignature::BToD1Tag => 0x42324431,
            TagSignature::BToD2Tag => 0x42324432,
            TagSignature::BToD3Tag => 0x42324433,
            TagSignature::GamutTag => 0x67616D74,
            TagSignature::GamutBoundaryDescription0Tag => 0x67626430,
            TagSignature::GamutBoundaryDescription1Tag => 0x67626431,
            TagSignature::GamutBoundaryDescription2Tag => 0x67626432,
            TagSignature::GamutBoundaryDescription3Tag => 0x67626433,
            TagSignature::GrayTRCTag => 0x6B545243,
            TagSignature::GreenMatrixColumnTag => 0x6758595A,
            TagSignature::GreenTRCTag => 0x67545243,
            TagSignature::LuminanceTag => 0x6C756D69,
            TagSignature::MaterialDefaultValuesTag => 0x6D647620,
            TagSignature::MaterialTypeArrayTag => 0x6D637461,
            TagSignature::MToA0Tag => 0x4D324130,
            TagSignature::MToB0Tag => 0x4D324230,
            TagSignature::MToB1Tag => 0x4D324231,
            TagSignature::MToB2Tag => 0x4D324232,
            TagSignature::MToB3Tag => 0x4D324233,
            TagSignature::MToS0Tag => 0x4D325330,
            TagSignature::MToS1Tag => 0x4D325331,
            TagSignature::MToS2Tag => 0x4D325332,
            TagSignature::MToS3Tag => 0x4D325333,
            TagSignature::MeasurementTag => 0x6D656173,
            TagSignature::MediaBlackPointTag => 0x626B7074,
            TagSignature::MediaWhitePointTag => 0x77747074,
            TagSignature::MetaDataTag => 0x6D657461,
            TagSignature::NamedColorTag => 0x6E636F6C,
            TagSignature::NamedColorV5Tag => 0x6E6D636C,
            TagSignature::NamedColor2Tag => 0x6E636C32,
            TagSignature::OutputResponseTag => 0x72657370,
            TagSignature::PerceptualRenderingIntentGamutTag => 0x72696730,
            TagSignature::Preview0Tag => 0x70726530,
            TagSignature::Preview1Tag => 0x70726531,
            TagSignature::Preview2Tag => 0x70726532,
            TagSignature::PrintConditionTag => 0x7074636E,
            TagSignature::ProfileDescriptionTag => 0x64657363,
            TagSignature::ProfileSequenceDescTag => 0x70736571,
            TagSignature::ProfileSequceIdTag => 0x70736964,
            TagSignature::Ps2CRD0Tag => 0x70736430,
            TagSignature::Ps2CRD1Tag => 0x70736431,
            TagSignature::Ps2CRD2Tag => 0x70736432,
            TagSignature::Ps2CRD3Tag => 0x70736433,
            TagSignature::Ps2CSATag => 0x70733273,
            TagSignature::Ps2RenderingIntentTag => 0x70733269,
            TagSignature::RedMatrixColumnTag => 0x7258595A,
            TagSignature::RedTRCTag => 0x72545243,
            TagSignature::ReferenceNameTag => 0x72666E6D,
            TagSignature::SaturationRenderingIntentGamutTag => 0x72696732,
            TagSignature::ScreeningDescTag => 0x73637264,
            TagSignature::ScreeningTag => 0x7363726E,
            TagSignature::SpectralDataInfoTag => 0x7364696E,
            TagSignature::SpectralWhitePointTag => 0x73777074,
            TagSignature::SpectralViewingConditionsTag => 0x7376636E,
            TagSignature::StandardToCustomPccTag => 0x73326370,
            TagSignature::SurfaceMapTag => 0x736D6170,
            TagSignature::TechnologyTag => 0x74656368,
            TagSignature::UcrBgTag => 0x62666420,
            TagSignature::ViewingCondDescTag => 0x76756564,
            TagSignature::ViewingConditionsTag => 0x76696577,
            TagSignature::EmbeddedV5ProfileTag => 0x49434335,
            TagSignature::MakeAndModelTag => 0x6D6D6F64,
            TagSignature::MultilocalizedDescriptionStringTag => 0x6473636D,
            TagSignature::NativeDisplayInfoTag => 0x6E64696E,
            TagSignature::VcgtTag => 0x76636774,
            TagSignature::VcgpTag => 0x76636770,
            TagSignature::AbsToRelTransSpaceTag => 0x61727473,
            TagSignature::Unknown(tag) => *tag,
        }
    }
}


/// Defines Zero-Sized Types (ZSTs) for tag signatures and implements
/// the `From` trait to convert them into their runtime `TagSignature` enum variant.
/// This is used when adding a Tag into the Profile's tag map, using the with_tag method, which uses
/// a TagSignature as key, and a TagType instance as value.
macro_rules! define_tag_zsts {
    // The macro accepts one or more comma-separated identifiers.
    ($($zst_name:ident),+ $(,)?) => {
        // The `$(...)+` block repeats the implementation for each identifier provided.
        $(
            #[doc = concat!("A Zero-Sized Type representing the `", stringify!($zst_name), "` tag signature.")]
            #[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
            pub struct $zst_name;

            impl From<$zst_name> for TagSignature {
                fn from(_: $zst_name) -> Self {
                    TagSignature::$zst_name
                }
            }
        )+
    };
}

// Use the macro to generate all the ZSTs and their impls
define_tag_zsts!(
    AToB0Tag,
    AToB1Tag,
    AToB2Tag,
    AToB3Tag,
    AToM0Tag,
    BlueMatrixColumnTag,
    BlueTRCTag,
    BrdfColorimetricParameter0Tag,
    BrdfColorimetricParameter1Tag,
    BrdfColorimetricParameter2Tag,
    BrdfColorimetricParameter3Tag,
    BrdfSpectralParameter0Tag,
    BrdfSpectralParameter1Tag,
    BrdfSpectralParameter2Tag,
    BrdfSpectralParameter3Tag,
    BRDFAToB0Tag,
    BRDFAToB1Tag,
    BRDFAToB2Tag,
    BRDFAToB3Tag,
    BRDFDToB0Tag,
    BRDFDToB1Tag,
    BRDFDToB2Tag,
    BRDFDToB3Tag,
    BRDFMToB0Tag,
    BRDFMToB1Tag,
    BRDFMToB2Tag,
    BRDFMToB3Tag,
    BRDFMToS0Tag,
    BRDFMToS1Tag,
    BRDFMToS2Tag,
    BRDFMToS3Tag,
    BToA0Tag,
    BToA1Tag,
    BToA2Tag,
    BToA3Tag,
    CalibrationDateTimeTag,
    CharTargetTag,
    ChromaticAdaptationTag,
    ChromaticityTag,
    ColorEncodingParamsTag,
    ColorSpaceNameTag,
    ColorantInfoTag,
    ColorantInfoOutTag,
    ColorantOrderTag,
    ColorantOrderOutTag,
    ColorantTableTag,
    ColorantTableOutTag,
    ColorimetricIntentImageStateTag,
    CopyrightTag,
    CrdInfoTag,
    CustomToStandardPccTag,
    CxFTag,
    DataTag,
    DateTimeTag,
    DeviceMediaWhitePointTag,
    DeviceMfgDescTag,
    DeviceModelDescTag,
    DeviceSettingsTag,
    DToB0Tag,
    DToB1Tag,
    DToB2Tag,
    DToB3Tag,
    BToD0Tag,
    BToD1Tag,
    BToD2Tag,
    BToD3Tag,
    GamutTag,
    GamutBoundaryDescription0Tag,
    GamutBoundaryDescription1Tag,
    GamutBoundaryDescription2Tag,
    GamutBoundaryDescription3Tag,
    GrayTRCTag,
    GreenMatrixColumnTag,
    GreenTRCTag,
    LuminanceTag,
    MaterialDefaultValuesTag,
    MaterialTypeArrayTag,
    MToA0Tag,
    MToB0Tag,
    MToB1Tag,
    MToB2Tag,
    MToB3Tag,
    MToS0Tag,
    MToS1Tag,
    MToS2Tag,
    MToS3Tag,
    MeasurementTag,
    MediaBlackPointTag,
    MediaWhitePointTag,
    MetaDataTag,
    NamedColorTag,
    NamedColorV5Tag,
    NamedColor2Tag,
    OutputResponseTag,
    PerceptualRenderingIntentGamutTag,
    Preview0Tag,
    Preview1Tag,
    Preview2Tag,
    PrintConditionTag,
    ProfileDescriptionTag,
    ProfileSequenceDescTag,
    ProfileSequceIdTag,
    Ps2CRD0Tag,
    Ps2CRD1Tag,
    Ps2CRD2Tag,
    Ps2CRD3Tag,
    Ps2CSATag,
    Ps2RenderingIntentTag,
    RedMatrixColumnTag,
    RedTRCTag,
    ReferenceNameTag,
    SaturationRenderingIntentGamutTag,
    ScreeningDescTag,
    ScreeningTag,
    SpectralDataInfoTag,
    SpectralWhitePointTag,
    SpectralViewingConditionsTag,
    StandardToCustomPccTag,
    SurfaceMapTag,
    TechnologyTag,
    UcrBgTag,
    ViewingCondDescTag,
    ViewingConditionsTag,
    EmbeddedV5ProfileTag,
    MakeAndModelTag,
    MultilocalizedDescriptionStringTag,
    NativeDisplayInfoTag,
    VcgtTag,
    VcgpTag,
    AbsToRelTransSpaceTag,
    
);