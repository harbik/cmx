use crate::tags::Tag;

/// Represents the Profile Connection Space (PCS) of an ICC profile.
// The PCS defines the color space used for the profile connection, which is essential for color management.
// The PCS can be XYZ (CIE1931 XYZ), Lab (CIELAB), or Spectral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Apple = 0x58595A20,           // 'APPL' - Apple Computer Inc.
    Microsoft = 0x4D534654,       // 'MSFT' - Microsoft Corporation',
    SiliconGraphics = 0x53494720, // 'SGI ' - Silicon Graphics Inc.
    SunMicrosystems = 0x53474920, // 'SUN ' - Sun Microsystems Inc.
    Taligent = 0x54474E54,        // 'TGNT' - Taligent Inc.
    None = 0x00000000,           // Not set
}

impl Platform {
    pub fn new(tag: Tag) -> Platform {
        match tag.0 {
            0x58595A20 => Platform::Apple,           // 'APPL'
            0x4D534654 => Platform::Microsoft,       // 'MSFT'
            0x53494720 => Platform::SiliconGraphics, // 'SGI '
            0x53474920 => Platform::SunMicrosystems, // 'SUN '
            0x54474E54 => Platform::Taligent,        // 'TGNT'
            _ => Platform::None,
        }
    }
}

impl From<Platform> for Tag {
    fn from(platform: Platform) -> Self {
        Tag(platform as u32)
    }
}
