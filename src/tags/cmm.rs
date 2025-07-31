#[derive(Debug, serde::Serialize)]
pub enum CmmSignature {
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

impl CmmSignature {
    pub fn new(sig: u32) -> Option<Self> {
        match sig {
            0x00000000 => None,
            0x41444245 => Some(Self::Adobe),
            0x41434D53 => Some(Self::Agfa),
            0x6170706C => Some(Self::Apple),
            0x43434D53 => Some(Self::ColorGear),
            0x5543434D => Some(Self::ColorGearLite),
            0x55434D53 => Some(Self::ColorGearC),
            0x45464920 => Some(Self::EFI),
            0x45584143 => Some(Self::ExactScan),
            0x46462020 => Some(Self::FujiFilm),
            0x48434d4d => Some(Self::HarlequinRIP),
            0x6172676C => Some(Self::ArgyllCMS),
            0x4c696e6f => Some(Self::Lino), // GH 22013
            0x44676f53 => Some(Self::LogoSync),
            0x48444d20 => Some(Self::Heidelberg),
            0x6C636D73 => Some(Self::LittleCMS),
            0x4b434d53 => Some(Self::Kodak),
            0x4d434d44 => Some(Self::KonicaMinolta),
            0x57435320 => Some(Self::WindowsCMS),
            0x5349474E => Some(Self::Mutoh),
            0x4f4e5958 => Some(Self::OnyxGraphics),
            0x52494343 => Some(Self::RefIccMAX),
            0x44494d58 => Some(Self::DemoIccMAX),
            0x52474d53 => Some(Self::RolfGierling),
            0x53494343 => Some(Self::SampleICC),
            0x54434D4D => Some(Self::Toshiba),
            0x33324254 => Some(Self::TheImagingFactory),
            0x7669766F => Some(Self::Vivo),
            0x57544720 => Some(Self::WareToGo),
            0x7a633030 => Some(Self::Zoran),
            n @ _ => Some(Self::Unknown(
                std::str::from_utf8(&n.to_be_bytes()).unwrap().to_string(),
            )),
        }
    }
}
