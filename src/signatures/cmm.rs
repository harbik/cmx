// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use num_derive::{FromPrimitive, ToPrimitive};

/// Color Management Modules, also known as CMMs, are software components that handle color
/// conversions between different color spaces. They are essential in color management systems to ensure
/// that colors are accurately represented across various devices and media.
/// Each CMM is identified by a unique signature, which is a 4-character code that indicates the
/// specific CMM used to create the ICC profile, and to identify the CMM that should be used
/// when interpreting the profile in case custom tags are used.
///
#[derive(Debug, serde::Serialize, PartialEq, strum::Display, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum Cmm {
    Adobe = 0x41444245,             /* 'ADBE' */
    Agfa = 0x41434D53,              /* 'ACMS' */
    Apple = 0x6170706C,             /* 'appl' */
    ColorGear = 0x43434D53,         /* 'CCMS' */
    ColorGearLite = 0x5543434D,     /* 'UCCM' */
    ColorGearC = 0x55434D53,        /* 'UCMS' */
    EFI = 0x45464920,               /* 'EFI ' */
    ExactScan = 0x45584143,         /* 'EXAC' */
    FujiFilm = 0x46462020,          /* 'FF  ' */
    HarlequinRIP = 0x48434d4d,      /* 'HCMM' */
    ArgyllCMS = 0x6172676C,         /* 'argl' */
    Lino = 0x4c696e6f,              /* 'Lino' */
    LogoSync = 0x44676f53,          /* 'LgoS' */
    Heidelberg = 0x48444d20,        /* 'HDM ' */
    LittleCMS = 0x6C636D73,         /* 'lcms' */
    Kodak = 0x4b434d53,             /* 'KCMS' */
    KonicaMinolta = 0x4d434d44,     /* 'MCMD' */
    WindowsCMS = 0x57435320,        /* 'WCS ' */
    Mutoh = 0x5349474E,             /* 'SIGN' */
    OnyxGraphics = 0x4f4e5958,      /* 'ONYX' */
    RefIccMAX = 0x52494343,         /* 'RICC' */
    DemoIccMAX = 0x44494d58,        /* 'DIMX' */
    RolfGierling = 0x52474d53,      /* 'RGMS' */
    SampleICC = 0x53494343,         /* 'SICC' */
    Toshiba = 0x54434D4D,           /* 'TCMM' */
    TheImagingFactory = 0x33324254, /* '32BT' */
    Vivo = 0x7669766F,              /* 'vivo' */
    WareToGo = 0x57544720,          /* 'WTG ' */
    Zoran = 0x7a633030,             /* 'zc00' */
}
