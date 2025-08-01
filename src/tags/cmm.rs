use crate::tags::Tag;

/// Color Management Modules, also known as CMMs, are software components that handle color
/// conversions between different color spaces. They are essential in color management systems to ensure
/// that colors are accurately represented across various devices and media.
/// Each CMM is identified by a unique signature, which is a 4-character code that indicates the
/// specific CMM used to create the ICC profile, and to identify the CMM that should be used
/// when interpreting the profile in case custom tags are used.
/// 
#[derive(Debug, serde::Serialize)]
pub enum Cmm {
    Adobe,             /* 'ADBE' */
    Agfa,              /* 'ACMS' */
    Apple,             /* 'appl' */
    ColorGear,         /* 'CCMS' */
    ColorGearLite,     /* 'UCCM' */
    ColorGearC,        /* 'UCMS' */
    EFI,               /* 'EFI ' */
    ExactScan,         /* 'EXAC' */
    FujiFilm,          /* 'FF  ' */
    HarlequinRIP,      /* 'HCMM' */
    ArgyllCMS,         /* 'argl' */
    Lino,              /* 'Lino' */
    LogoSync,          /* 'LgoS' */
    Heidelberg,        /* 'HDM ' */
    LittleCMS,         /* 'lcms' */
    Kodak,             /* 'KCMS' */
    KonicaMinolta,     /* 'MCML' */
    WindowsCMS,        /* 'WCS ' */
    Mutoh,             /* 'SIGN' */
    OnyxGraphics,      /* 'ONYX' */
    RefIccMAX,         /* 'RIMX' */
    DemoIccMAX,        /* 'DIMX' */
    RolfGierling,      /* 'RGMS' */
    SampleICC,         /* 'SICC' */
    Toshiba,           /* 'TCMM' */
    TheImagingFactory, /* '32BT' */
    Vivo,              /* 'VIVO' */
    WareToGo,          /* 'WTG ' */
    Zoran,             /* 'zc00' */
    Unknown(String),
}

/// Converts a Cmm enum to a Tag value.
impl From<Cmm> for Tag {
    fn from(sig: Cmm) -> Self {
        let val = match sig {
            Cmm::Adobe => 0x41444245,
            Cmm::Agfa => 0x41434D53,
            Cmm::Apple => 0x6170706C,
            Cmm::ColorGear => 0x43434D53,
            Cmm::ColorGearLite => 0x5543434D,
            Cmm::ColorGearC => 0x55434D53,
            Cmm::EFI => 0x45464920,
            Cmm::ExactScan => 0x45584143,
            Cmm::FujiFilm => 0x46462020,
            Cmm::HarlequinRIP => 0x48434d4d,
            Cmm::ArgyllCMS => 0x6172676C,
            Cmm::Lino => 0x4c696e6f, // GH 22013
            Cmm::LogoSync => 0x44676f53,
            Cmm::Heidelberg => 0x48444d20,
            Cmm::LittleCMS => 0x6C636D73,
            Cmm::Kodak => 0x4b434d53,
            Cmm::KonicaMinolta => 0x4d434d44,
            Cmm::WindowsCMS => 0x57435320,
            Cmm::Mutoh => 0x5349474E,
            Cmm::OnyxGraphics => 0x4f4e5958,
            Cmm::RefIccMAX => 0x52494343,
            Cmm::DemoIccMAX => 0x44494d58,
            Cmm::RolfGierling => 0x52474d53,
            Cmm::SampleICC => 0x53494343,
            Cmm::Toshiba => 0x54434D4D,
            Cmm::TheImagingFactory => 0x33324254,
            Cmm::Vivo => 0x7669766F,
            Cmm::WareToGo => 0x57544720,
            Cmm::Zoran => 0x7a633030,
            Cmm::Unknown(s) => {
                // Convert the string to bytes, ensuring it is 4 bytes long
                let mut bytes = [0u8; 4];
                let bytes_from_str = s.as_bytes();
                for (i, &byte) in bytes_from_str.iter().take(4).enumerate() {
                    bytes[i] = byte;
                }
                u32::from_be_bytes(bytes)
            }
        };
        Tag(val)
    }
}
    

impl Cmm {
    pub fn new(sig: Tag) -> Self {
        match sig.0 {
            0x41444245 => Self::Adobe,
            0x41434D53 => Self::Agfa,
            0x6170706C => Self::Apple,
            0x43434D53 => Self::ColorGear,
            0x5543434D => Self::ColorGearLite,
            0x55434D53 => Self::ColorGearC,
            0x45464920 => Self::EFI,
            0x45584143 => Self::ExactScan,
            0x46462020 => Self::FujiFilm,
            0x48434d4d => Self::HarlequinRIP,
            0x6172676C => Self::ArgyllCMS,
            0x4c696e6f => Self::Lino,
            0x44676f53 => Self::LogoSync,
            0x48444d20 => Self::Heidelberg,
            0x6C636D73 => Self::LittleCMS,
            0x4b434d53 => Self::Kodak,
            0x4d434d44 => Self::KonicaMinolta,
            0x57435320 => Self::WindowsCMS,
            0x5349474E => Self::Mutoh,
            0x4f4e5958 => Self::OnyxGraphics,
            0x52494343 => Self::RefIccMAX,
            0x44494d58 => Self::DemoIccMAX,
            0x52474d53 => Self::RolfGierling,
            0x53494343 => Self::SampleICC,
            0x54434D4D => Self::Toshiba,
            0x33324254 => Self::TheImagingFactory,
            0x7669766F => Self::Vivo,
            0x57544720 => Self::WareToGo,
            0x7a633030 => Self::Zoran,
            _ => Self::Unknown(String::from_utf8_lossy(&sig.0.to_be_bytes()).to_string()),
        }
    }
}