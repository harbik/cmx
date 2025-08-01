use crate::tags::Tag;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DeviceClass {
    InputDevice        = 0x73636E72, // 'scnr'
    DisplayDevice      = 0x6D6E7472, // 'mntr'
    OutputDevice       = 0x70727472, // 'prtr'
    LinkDevice         = 0x6C696E6B, // 'link'
    AbstractDevice     = 0x61627374, // 'abst'
    ColorSpaceDevice   = 0x73706163, // 'spac'
    NamedColorDevice   = 0x6E6D636C, // 'nmcl'
    // ICC.2 (ICCmax) additions:
    SpectralDevice     = 0x73706563, // 'spec'
    Unknown(u32),
}

impl DeviceClass {
    pub fn new(tag: Tag) -> Self {
        match tag.0 {
            0x73636E72 => DeviceClass::InputDevice,
            0x6D6E7472 => DeviceClass::DisplayDevice,
            0x70727472 => DeviceClass::OutputDevice,
            0x6C696E6B => DeviceClass::LinkDevice,
            0x61627374 => DeviceClass::AbstractDevice,
            0x73706163 => DeviceClass::ColorSpaceDevice,
            0x6E6D636C => DeviceClass::NamedColorDevice,
            0x73706563 => DeviceClass::SpectralDevice,
            other => DeviceClass::Unknown(other),
        }
    }
}