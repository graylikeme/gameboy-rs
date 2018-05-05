use std::mem::size_of;
use num_traits::Unsigned;
use num_traits::int::PrimInt;
use num_traits::cast::NumCast;

pub fn set_least(of: u16, to: u8) -> u16 {
    (of & 0xFF00) | to as u16
}

pub fn get_least(of: u16) -> u8 {
    of as u8
}

pub fn set_most(of: u16, to: u8) -> u16 {
    (of & 0x00FF) | (to as u16) << 8
}

pub fn get_most(of: u16) -> u8 {
    (of >> 8) as u8
}

pub fn flip_bit<T>(at: usize, of: T) -> Option<T>
    where T: Unsigned + PrimInt + NumCast {
    if size_of::<T>()*8 <= at {
        return None
    }

    let num: Option<T> = NumCast::from(0x01);
    if  let Some(n) = num {
        return Some((n << at) ^ of)
    }

    None
}