use zerocopy::{
    BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Ref, Unaligned, U16, U32, U64,
};

use crate::profile::Profile;

use crate::error::Error;
use crate::tags::Tag;

#[derive(FromBytes, IntoBytes, Unaligned, KnownLayout, Immutable, Debug, Clone, Copy)]
#[repr(C)]
pub struct IccHeader {
    pub profile_size: U32<BigEndian>,
    pub cmm_type: U32<BigEndian>,
    pub version: U32<BigEndian>,
    pub device_class: U32<BigEndian>,
    pub color_space: U32<BigEndian>,
    pub pcs: U32<BigEndian>,
    pub creation_year: U16<BigEndian>,
    pub creation_month: U16<BigEndian>,
    pub creation_day: U16<BigEndian>,
    pub creation_hours: U16<BigEndian>,
    pub creation_minutes: U16<BigEndian>,
    pub creation_seconds: U16<BigEndian>,
    pub file_signature: U32<BigEndian>,
    pub primary_platform: U32<BigEndian>,
    pub flags: U32<BigEndian>,
    pub manufacturer: U32<BigEndian>,
    pub model: U32<BigEndian>,
    pub attributes: U64<BigEndian>,
    pub rendering_intent: U32<BigEndian>,
    pub pcs_illuminant: [u8; 12],
    pub creator: U32<BigEndian>,
    pub profile_id: [u8; 16],
    pub reserved: [u8; 28],
}

/*
impl IccHeader {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let r = Ref::<_, IccHeader>::from_bytes(bytes)
            .map_err(|e| Error::HeaderParseError(e.to_string()))?;
        Ok(*r)
    }
}
 */

impl Profile {
    pub fn header(&self) -> Result<IccHeader, Error> {
        let r = Ref::<_, IccHeader>::from_bytes(self.header.as_slice())
            .map_err(|e| Error::HeaderParseError(e.to_string()))?;
        Ok(*r)
    }

    pub fn manufacturer(&self) -> Result<Tag, Error> {
   //     let r = Ref::<_, IccHeader>::from_bytes(self.header.as_slice()) .map_err(|e| Error::HeaderParseError(e.to_string()))?;
        let header = self.header()?;
        let m = header.manufacturer.get();
        Ok(Tag(m))
    }

    pub fn set_manufacturer(self, manufacturer: u32) -> Result<Self, Error> {
        let mut header = self.header()?;
        header.manufacturer = U32::new(manufacturer);
        let new_header = header.as_bytes().to_vec();
        Ok(Self {
            header: new_header,
            tags: self.tags,
            padding: self.padding,
        })
    }

    pub fn size(&self) -> usize {
        let header = self.header().unwrap();
        header.profile_size.get() as usize
    }
    pub fn cmm_type(&self) -> Tag {
        let header = self.header().unwrap();
        Tag(header.cmm_type.get())
    }


}

#[cfg(test)]
mod test{ 
    use crate::{profile::Profile, tags::Tag};

    #[test]
    fn test_header() {
        let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
        let header = profile.header().unwrap();
        let mfg = header.manufacturer.get();
        let mfg_str = Tag(mfg).to_string();
        assert_eq!(mfg_str,"APPL");
    }

    #[test]
    fn test_set_manufacturer() {
        let profile = Profile::from_file("tests/profiles/Display P3.icc").unwrap();
        let signature: Tag = "TEST".parse().unwrap();
        let mfg = u32::from(signature);
        let updated_profile = profile.set_manufacturer(mfg).unwrap();
        let mfg_new = updated_profile.manufacturer().unwrap();
        assert_eq!(mfg_new.to_string(), "TEST");
    }

}