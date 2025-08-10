use crate::tags::MultiLocalizedUnicodeType;

use isocountry::CountryCode;
use isolang::Language;
use zerocopy::{BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, U16, U32};

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
pub struct MultiLocalizedUnicodeRecord {
    language: U16<BigEndian>, // First record language code: in accordance with the language code specified in ISO 639-1
    country: U16<BigEndian>, // First record country code: in accordance with the country code specified in ISO 3166-1 
    length: U32<BigEndian>, //  length in bytes of the string 
    offset: U32<BigEndian>, // offset in bytes from the start of the MultiLocalizedUnicode tag to the start of the string

}

pub struct MultiLocalizedUnicodeEntry{
    pub language: Language,
    pub country: Option<CountryCode>,
    pub value: String,
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
pub struct MultiLocalizedUnicodeMapHeader {
    type_signature: U32<BigEndian>,
    reserved: [u8; 4],
    number_of_records: U32<BigEndian>,
    record_size: U32<BigEndian>, // always 12
    
}
#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
pub struct MultiLocalizedUnicodeMapRecordsTable {
    records: [MultiLocalizedUnicodeRecord],
}
     
impl MultiLocalizedUnicodeType {
    pub fn try_ref_from_bytes(&self) -> Vec<MultiLocalizedUnicodeEntry> {
        let r = MultiLocalizedUnicodeMapHeader::try_ref_from_bytes(&self.0[0..16]).unwrap();
        let n = r.number_of_records.get() as usize;
        let record_size = r.record_size.get() as usize;
        let table_end = n * record_size + 16;
        let table = MultiLocalizedUnicodeMapRecordsTable::try_ref_from_bytes(&self.0[16..table_end]).unwrap();
        let mut entries = Vec::with_capacity(n);
        for r in &table.records {
            let language = Language::from_639_1(r.language.get().to_string().as_str()).unwrap();
            let country = CountryCode::for_alpha2_caseless(r.country.get().to_string().as_str()).ok();
            let offset = r.offset.get() as usize;
            let length = r.length.get() as usize;
            let value_bytes = &self.0[offset..offset + length];
            let value = String::from_utf16(&value_bytes.chunks(2).map(|x| u16::from_be_bytes([x[0], x[1]])).collect::<Vec<u16>>()).unwrap();
            entries.push(MultiLocalizedUnicodeEntry { language, country, value });
        }
        entries
    }

    pub fn try_mut_from_bytes(&mut self) -> &mut MultiLocalizedUnicodeMapHeader {
        MultiLocalizedUnicodeMapHeader::try_mut_from_bytes(&mut self.0).unwrap()
    }
}
     

/*
     
pub struct MultiLocalizedUnicodeValues(Vec<(Option<CountryCode>, Language, String)>);

impl MultiLocalizedUnicodeValues {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let n = read_be_u32(buf)? as usize;
        let mut pos = Vec::with_capacity(n);
        let twelve = read_be_u32(buf)?;
        if twelve != 12 {
            return Err("Incorrect multilocalized record length".into());
        }
        for _ in 0..n {
            let lang = read_ascii_string(buf, 2)?;
            let mut country = read_ascii_string(buf, 2)?;
            if country == "FU" {
                country = String::from("FR")
            }; // found in Generic CMYK Profile MacOS
            if country == "PO" {
                country = String::from("PT")
            }; // found in Generic CMYK Profile
            let length = read_be_u32(buf)? as usize;
            let start = (read_be_u32(buf)? - (16 + 12 * n as u32)) as usize;
            pos.push((lang, country, start, length));
        }
        let data = read_vec_u16(buf, buf.len())?;
        let mut mlu = Vec::new();
        for (lang, country, start, length) in pos {
            mlu.push((
                CountryCode::for_alpha2_caseless(country.as_str()).ok(),
                Language::from_639_1(lang.as_str()).unwrap(),
                String::from_utf16(&data[start / 2..start / 2 + length / 2])?,
            ));
        }

        Ok(Self(mlu))
    }
}
    */