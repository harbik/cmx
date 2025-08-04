use crate::signatures::Signature;

/// Represents the Profile Connection Space (PCS) of an ICC profile.
// The PCS defines the color space used for the profile connection, which is essential for color management.
// The PCS can be XYZ (CIE1931 XYZ), Lab (CIELAB), or Spectral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Apple = 0x4150504C,           // 'APPL' - Apple Computer Inc.
    Microsoft = 0x4D534654,       // 'MSFT' - Microsoft Corporation',
    SiliconGraphics = 0x53474920, // 'SGI ' - Silicon Graphics Inc.
    SunMicrosystems = 0x53554E57, // 'SUN ' - Sun Microsystems Inc.
    All = 0x00000000,           // Not set
}

impl Platform {
    pub fn new(tag: Signature) -> Platform {
        match tag.0 {
            0x4150504C => Platform::Apple,           // 'APPL'
            0x4D534654 => Platform::Microsoft,       // 'MSFT'
            0x53474920 => Platform::SiliconGraphics, // 'SGI '
            0x53554E57 => Platform::SunMicrosystems, // 'SUN '
            _ => Platform::All,
        }
    }
}

impl From<Platform> for Signature {
    fn from(platform: Platform) -> Self {
        Signature(platform as u32)
    }
}
