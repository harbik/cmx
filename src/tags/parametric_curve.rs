
use crate::util::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ParametricCurve {
    ExponentGamma{g: f32},
    CIE122{g: f32, a: f32, b:f32},
    IEC61966_3{g: f32, a: f32, b:f32, c: f32},
    IEC61966_2_1{g: f32, a: f32, b:f32, c: f32, d: f32},
    SevenParameter{g: f32, a: f32, b:f32, c: f32, d: f32, e: f32, f: f32},
}

impl ParametricCurve{
    pub fn try_new(buf: &mut &[u8]) -> Result<Self> {
        let function_type = read_be_u16(buf)?;
        let _not_used = read_be_u16(buf)?;
        match function_type {
            0 => Ok(Self::ExponentGamma{g:read_s15fixed16(buf)?}),
            1 => Ok(Self::CIE122{
                g:read_s15fixed16(buf)?,
                a:read_s15fixed16(buf)?,
                b:read_s15fixed16(buf)?,
            }),
            2 => Ok(Self::IEC61966_3{
                g:read_s15fixed16(buf)?,
                a:read_s15fixed16(buf)?,
                b:read_s15fixed16(buf)?,
                c:read_s15fixed16(buf)?,
            }),
            3 => Ok(Self::IEC61966_2_1{
                g:read_s15fixed16(buf)?,
                a:read_s15fixed16(buf)?,
                b:read_s15fixed16(buf)?,
                c:read_s15fixed16(buf)?,
                d:read_s15fixed16(buf)?,
            }),
            4 => Ok(Self::SevenParameter{
                g:read_s15fixed16(buf)?,
                a:read_s15fixed16(buf)?,
                b:read_s15fixed16(buf)?,
                c:read_s15fixed16(buf)?,
                d:read_s15fixed16(buf)?,
                e:read_s15fixed16(buf)?,
                f:read_s15fixed16(buf)?,
            }),
            _ => Err("Illegal function type".into())

        }
    }

    pub fn value(&self, x: f32) -> f32 {
        if x<0.0 || x>1.0 { 
            f32::NAN
        } else {
            match *self {
                Self::ExponentGamma{g} => x.powf(g),
                Self::CIE122{g,a,b} => {
                    if x>= -b/a {
                        (a*x + b).powf(g)
                    } else {
                        0.0
                    }
                }
                Self::IEC61966_3{g,a,b, c} => {
                    if x>= -b/a {
                        (a*x + b).powf(g) + c
                    } else {
                       c 
                    }
                }
                Self::IEC61966_2_1{g,a,b, c, d} => {
                    if x>= d {
                        (a*x + b).powf(g)
                    } else {
                        c*x
                    }
                }
                Self::SevenParameter{g,a,b, c, d, e, f} => {
                    if x>= d {
                        (a*x + b).powf(g) + e
                    } else {
                        c*x + f
                    }
                }
            }

        }
    }
}

