use crate::common::*;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Vcgp {
    tbd: Vec<u8> // can not find any information about this tag
}

impl Vcgp {
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        Ok(Vcgp{
            tbd: read_vec(buf, buf.len())?
        })
    }
}
