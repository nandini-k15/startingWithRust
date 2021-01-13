#[allow(dead_code)]

#[cfg(feature = "BIGNUM")]
use super::bignum;
//#[allow(dead_code)]
use std::convert::TryFrom;
use std::convert::TryInto;
pub const ERR_ERROR_CORRUPTION_DETECTED: i32 = -0x006E;
use crate::ans1::ERR_BUF_TOO_SMALL;
use crate::ans1::IA5_STRING;
use crate::ans1::PRINTABLE_STRING;
use crate::ans1::UTF8_STRING;
use crate::ans1::INTEGER;
/*fn byte_copy(from: &[u8],mut to: &mut [u8]) -> usize {
    to.write(&from).unwrap()
}*/
#[allow(unused)]  

macro_rules! add{
    ($g:expr, $f:expr) => {
        loop{
            if ret=(f) < 0{
            return (ret);}
            else{
            (g) = (g) + ret;}
        }
    }
}
macro_rules! multiply_add{
        ($a:expr , $b:expr , $c:expr)=>{$a * ($b+$c)};
    }
#[allow(unused)]
pub struct SBuffer {
    /// Buffer holding data
    buf: Vec<u8>,
    /// index for keeping track of location to be read
    ptr: usize,
}

impl SBuffer {
    fn copy(&self) -> SBuffer {
        let x = SBuffer {
            buf: self.buf[..].iter().cloned().collect(),
            ptr: self.ptr,
        };
        return x;
    }
}

pub fn fon(){
    let a :i32 =5;
    let b :i32 =6;
    let c :i32 =3;
    multiply_add!(a,b,c);
   // add!(a1,b1);
}
    

//println!("{}",ERR_BUF_TOO_SMALL);
pub fn write_len(p: &mut SBuffer, start: &mut usize, len: &mut usize) -> i32 {
    if len < &mut 0x80 
    {
    if (p.ptr - *start) < 1
     {
        return ERR_BUF_TOO_SMALL;
     }
     p.ptr = p.ptr - 1;
     p.buf[p.ptr] = *len as u8;
     return 1;
    }

     if len <= &mut 0xFF
    {
    if (p.ptr - *start) < 2
     {
        return ERR_BUF_TOO_SMALL;
     }
     p.ptr = p.ptr - 1;
     p.buf[p.ptr] = *len as u8 ;
     p.ptr = p.ptr - 1;
     p.buf[p.ptr] = 0x81;
     return 2;
    }

    return 0;
}

fn write_tag(p: &mut SBuffer, start: &mut usize, tag: u8) -> i32 {
    if (p.ptr - *start) < 1 {
        return ERR_BUF_TOO_SMALL;
    }

    p.ptr = p.ptr - 1;
    p.buf[p.ptr] = tag;

    return 1;
}
fn write_raw_buffer(p: &mut SBuffer, start: &mut usize, buff: usize, size: usize) -> i32 {
    let mut len: usize = 0;

    if p.ptr < *start || (p.ptr - *start) < size {
        return ERR_BUF_TOO_SMALL;
    }

    len = size;
    p.ptr = p.ptr - len;
    //memcpy

    return len as i32;
}

#[cfg(feature = "BIGNUM")]

fn write_mpi(p: &mut SBuffer, start: &mut usize, x: &mut bignum::mpi) -> i32 {
    let mut ret: i32 = ERR_ERROR_CORRUPTION_DETECTED;
    let mut len: usize = 0;

    //write the MPI
    //
    len = bignum::size(x);

    if p.ptr < start || (p.ptr - *start) < len {
        return ERR_BUF_TOO_SMALL;
    }

    p.ptr = p.ptr - len;
    //chk

    if x.s == 1 && p.buf[p.ptr] & 0x80 {
        if (p.ptr - *start) < 1 {
            return ERR_BUF_TOO_SMALL;
        }

        p.ptr = p.ptr - 1;
        p.buf[p.ptr] = 0x00;
        len = len + 1;
    }

   // add!(len, write_len(p, start, len));
    //add!(len,write_tag(p, start, INTEGER));
    add!(len,write_len(p,start,len));
    return len as i32;
    //cleanup
}

fn write_null(p: &mut SBuffer, start: &mut usize) -> i32 {
    let mut ret: i32 = ERR_ERROR_CORRUPTION_DETECTED;
    let mut len: usize = 0;

    return len as i32;
}

fn write_oid(p: &mut SBuffer, start: Vec<u8>, oid: &Vec<u8>, oid_len: usize) -> i32 {
    let mut ret: i32 = ERR_ERROR_CORRUPTION_DETECTED;
    let mut len: usize = 0;

    return len as i32;
}

fn write_algorithm_identifier(
    p: &mut SBuffer,
    start: Vec<u8>,
    oid: &Vec<u8>,
    oid_len: usize,
    par_len: usize,
) -> i32 {
    let mut ret: i32 = ERR_ERROR_CORRUPTION_DETECTED;
    let mut len: usize = 0;

    if par_len == 0 {
        //check
        return 0;
    } else {
        len = len + par_len;
    }
    return len as i32;
}

fn write_bool(p: &mut SBuffer, start: &mut usize, boolean: i32) -> i32 {
    let mut ret: i32 = ERR_ERROR_CORRUPTION_DETECTED;
    let mut len: usize = 0;

    if (p.ptr - *start) < 1 {
        return ERR_BUF_TOO_SMALL;
    }

    p.ptr = p.ptr - 1;
    if boolean == 0 {
        p.buf[p.ptr] = 255;
    } else {
        p.buf[p.ptr] = 0;
    }

    len = len + 1;
    return len as i32;
}
//write_tagged_int function
fn write_int(p: &mut SBuffer, start: Vec<u8>, val: i32) -> i32 {
    0
}

fn write_enum(p: &mut SBuffer, start: Vec<u8>, val: i32) -> i32 {
    0
}

fn write_tagged_string(
    p: &mut SBuffer,
    start: &mut usize,
    tag: i32,
    text: &mut usize,
    text_len: usize,
) -> i32 {
    let mut ret: i32 = ERR_ERROR_CORRUPTION_DETECTED;
    let mut len: usize = 0;

    return len as i32;
}

fn write_printable_string(
    p: &mut SBuffer,
    start: &mut usize,
    text: &mut usize,
    text_len: usize,
) -> i32 {
    return write_tagged_string(p, start, PRINTABLE_STRING, text, text_len);
}

fn write_utf8_string(p: &mut SBuffer, start: &mut usize, text: &mut usize, text_len: usize) -> i32 {
    return write_tagged_string(p, start, UTF8_STRING, text, text_len);
}

fn write_ias_string(p: &mut SBuffer, start: &mut usize, text: &mut usize, text_len: usize) -> i32 {
    return write_tagged_string(p, start, IA5_STRING, text, text_len);
}

fn write_bitstring(p: &mut SBuffer, start: &mut usize, buff: &mut Vec<u8>, bits: usize) -> i32 {
    let mut ret: i32 = ERR_ERROR_CORRUPTION_DETECTED;
    let mut len: usize = 0;
    let mut unused_bits: usize;
    let mut byte_len: usize;

    byte_len = (bits + 7) / 8;
    unused_bits = (byte_len * 8) - bits;

    if p.ptr < *start || (p.ptr - *start) < byte_len + 1 {
        return ERR_BUF_TOO_SMALL;
    }

    len = byte_len + 1;

    if byte_len > 0 {
        byte_len = byte_len - 1;
        p.ptr = p.ptr - 1;
        p.buf[p.ptr] = buff[byte_len] & !((0x1 << unused_bits) - 1);
        p.ptr = p.ptr - byte_len;
        //memcpy
    }
    p.ptr = p.ptr - 1;
    p.buf[p.ptr] = unused_bits as u8;

    return len as i32;
}

fn write_named_bitstring(p: &mut SBuffer, start: &mut usize, buff: &Vec<u8>, bits: usize) -> i32 {
    let mut unused_bits: usize;
    let mut byte_len: usize;
    let mut cur_byte: usize;
    let mut cur_byte_shifted: u8;
    let mut bit: u8;

    byte_len = (bits + 7) / 8;
    unused_bits = (byte_len * 8) - bits;

    /*
     * Named bitstrings require that trailing 0s are excluded in the encoding
     * of the bitstring. Trailing 0s are considered part of the 'unused' bits
     * when encoding this value in the first content octet
     */

    /*if bits != 0
    {
        cur_byte = buff + byte_len - 1;
        cur_byte_shifted = cur_byte >> unused_bits;
    }*/

    return 0;
}

fn write_octet_string(p: &mut SBuffer, start: Vec<u8>, buf: &Vec<u8>, size: usize) -> i32 {
    let mut ret: i32 = ERR_ERROR_CORRUPTION_DETECTED;
    let mut len: usize = 0;

    return len as i32;
}
