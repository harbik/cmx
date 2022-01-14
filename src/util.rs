
use crate::tag_signatures::TagSignature;
use chrono::{DateTime, Datelike, Timelike, Utc};

pub fn zero_as_none<T: num::Num + num::Zero>(v: T) -> Option<T> {
    if v.is_zero() {
        None
    } else {
        Some(v)
    }
}

pub fn read_be_f16(input: &mut &[u8]) -> Result<half::f16, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<half::f16>());
    *input = rest;
    Ok(half::f16::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_f32(input: &mut &[u8]) -> Result<f32, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<f32>());
    *input = rest;
    Ok(f32::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_f64(input: &mut &[u8]) -> Result<f64, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<f64>());
    *input = rest;
    Ok(f64::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_u8(input: &mut &[u8]) -> Result<u8, Box<dyn std::error::Error + 'static>> {
    let (byte, rest) = input.split_at(std::mem::size_of::<u8>());
    *input = rest;
    Ok(byte[0])
}

pub fn read_be_u16(input: &mut &[u8]) -> Result<u16, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
    *input = rest;
    Ok(u16::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_u32(input: &mut &[u8]) -> Result<u32, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    Ok(u32::from_be_bytes(int_bytes.try_into()?))
}


pub fn read_be_i32(input: &mut &[u8]) -> Result<i32, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<i32>());
    *input = rest;
    Ok(i32::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_u64(input: &mut &[u8]) -> Result<u64, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
    *input = rest;
    Ok(u64::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_be_u128(input: &mut &[u8]) -> Result<u128, Box<dyn std::error::Error + 'static>> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u128>());
    *input = rest;
    Ok(u128::from_be_bytes(int_bytes.try_into()?))
}

pub fn read_version(input: &mut &[u8]) -> Result<[u8;3], Box<dyn std::error::Error + 'static>> {
    let (version, rest) = input.split_at(std::mem::size_of::<[u8;4]>());
    *input = rest;
    Ok([version[0], version[1]>>4_u8, version[1]&0x0F_u8])
}


pub fn read_date_time(icc_buf: &mut &[u8]) -> Result <Option<chrono::DateTime<chrono::Utc>>, Box<dyn std::error::Error + 'static>> {
    let year = read_be_u16(icc_buf)?;
    let month = read_be_u16(icc_buf)?;
    let day = read_be_u16(icc_buf)?;
    let hour = read_be_u16(icc_buf)?;
    let minute = read_be_u16(icc_buf)?;
    let second = read_be_u16(icc_buf)?;
    if year == 0 && month == 0 && day == 0 {
        Ok(None)
    } else {
        let d = chrono::NaiveDate::from_ymd(year as i32, month as u32, day as u32);
        let t = chrono::NaiveTime::from_hms(hour as u32, minute as u32, second as u32);
        let dt = chrono::NaiveDateTime::new(d,t);
        Ok(Some(chrono::DateTime::from_utc(dt, chrono::Utc)))
    }
}

pub fn datetime_to_be_bytes(dt: Option<DateTime<Utc>>) -> [u8;12] {
    match dt {
        None => [0;12],
        Some(dt) => {
            let year = dt.date().year() as u16;
            let month = dt.date().month() as u16;
            let day = dt.date().day() as u16;
            let hour = dt.time().hour() as u16;
            let minute = dt.time().minute() as u16;
            let second = dt.time().second() as u16;
            let mut v: Vec<u8> = Vec::with_capacity(12);
            v.extend(year.to_be_bytes());
            v.extend(month.to_be_bytes());
            v.extend(day.to_be_bytes());
            v.extend(hour.to_be_bytes());
            v.extend(minute.to_be_bytes());
            v.extend(second.to_be_bytes());
            v.try_into().unwrap() // should not fail as generated from a datetime
        }
    }
}


pub fn read_signature(icc_buf: &mut &[u8]) -> Result<Option<String>, Box<dyn std::error::Error + 'static>>{
    let (s, rest) = icc_buf.split_at(std::mem::size_of::<[u8;4]>());
    *icc_buf = rest;
    if s[0]!=0 && s[1]!=0 && s[2]!=0 && s[3]!=0 {
        Ok(Some(std::str::from_utf8(s)?.to_owned()))
    } else {
        Ok(None)
    }
}

pub fn read_tag_signature(icc_buf: &mut &[u8]) -> Result<TagSignature, Box<dyn std::error::Error + 'static>>{
    let s = read_be_u32(icc_buf)?;
    /*
    match FromPrimitive::from_u32(s) {
        Some(tag_sig) => Ok(tag_sig),
        None => Ok(TagSignature::Unknown),
        //None => Err(format!("Unknown tag {:?} found", std::str::from_utf8(&s.to_be_bytes())).into()),
    }
    */
    Ok(TagSignature::new(s))
    
}

pub fn read_xyz(icc_buf: &mut &[u8]) -> Result< Option<[f64;3]>, Box<dyn std::error::Error + 'static>> {
    let x_i32 = read_be_i32(icc_buf)?;
    let y_i32 = read_be_i32(icc_buf)?;
    let z_i32 = read_be_i32(icc_buf)?;
    if x_i32 == 0 && y_i32 == 0 && z_i32 == 0 {
        Ok(None)
    } else {
        let x = x_i32 as f64/65536.0;
        let y = y_i32 as f64/65536.0;
        let z = z_i32 as f64/65536.0;
        Ok(Some([x,y,z]))
    }
}


pub fn xyz_to_be_bytes(xyz: Option<[f64;3]>) -> [u8;12] {
    match xyz {
       None =>  [0;12],
       Some([x,y,z]) => {
        let mut v: Vec<u8> = Vec::with_capacity(12);
        v.extend(((x * 655536.0) as i32).to_be_bytes());
        v.extend(((y * 655536.0) as i32).to_be_bytes());
        v.extend(((z * 655536.0) as i32).to_be_bytes());
        v.truncate(12);
        v.try_into().unwrap()

       }
    }
}

pub fn read_mcs(icc_buf: &mut &[u8]) -> Result<Option<u16>, Box<dyn std::error::Error + 'static>> {
    let sig = read_be_u16(icc_buf)?;
    let n = read_be_u16(icc_buf)?;
    if sig==0 || n==0 {
        Ok(None)
    } else {
        Ok(Some(n))
    }
}

pub fn mcs_to_be_bytes(n_mcs: Option<u16>) -> [u8;4] {
    let n = n_mcs.unwrap_or(0) as u32;
    if n == 0 {
        [0, 0, 0, 0]
    } else {
        (0x6d63u32 << 2 | n).to_be_bytes()
    }
}


pub fn read_vec(input: &mut &[u8], n: usize) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
    if n>input.len() {
        return Err("request exceeds buffer length".into())
    }
    let (bytes, rest) = input.split_at(n);
    *input = rest;
    Ok(bytes.to_vec())
}

pub fn read_vec_u16(input: &mut &[u8], n: usize) -> Result<Vec<u16>, Box<dyn std::error::Error + 'static>> {
    if n>input.len() {
        return Err("request exceeds buffer length".into())
    }
    let (mut bytes, rest) = input.split_at(n);
    *input = rest;
    let mut v = Vec::with_capacity(n/2);
    for _ in 0..n/2 {
        v.push(read_be_u16(&mut bytes)?);
    }
    Ok(v)
}

pub fn read_ascii_string(buf: &mut &[u8], n: usize) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let v = read_vec(buf,n)?;
    Ok(std::str::from_utf8(&v)?.trim_end_matches(char::from(0)).to_owned())
}

pub fn read_unicode_string(buf: &mut &[u8], n: usize) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let v = read_vec_u16(buf,n/2)?;
    Ok(String::from_utf16(&v)?.trim_end_matches(char::from(0)).to_owned())
}

pub fn read_s15fixed16(buf: &mut &[u8]) -> Result<f32, Box<dyn std::error::Error + 'static>> {
    let v_i32 = read_be_i32(buf)?;
    let v = v_i32 as f32/65536.0;
    Ok(v)
}

pub fn read_s15fixed16_array(buf: &mut &[u8], n: Option<usize>) -> Result<Vec<f32>, Box<dyn std::error::Error + 'static>> {
    let n = n.unwrap_or(buf.len());
    if n>buf.len() {
        return Err("request exceeds buffer length".into())
    }
    let mut v = Vec::with_capacity(n/4);
    for _ in 0..n/4 {
        v.push(read_s15fixed16(buf)?);
    }
    Ok(v)

}
