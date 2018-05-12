use num_traits::{NumCast, PrimInt, Unsigned};
use std::mem::size_of;

#[allow(dead_code)]
pub fn set_least(of: u16, to: u8) -> u16 {
    (of & 0xFF00) | to as u16
}

#[allow(dead_code)]
pub fn get_least(of: u16) -> u8 {
    of as u8
}

#[allow(dead_code)]
pub fn set_most(of: u16, to: u8) -> u16 {
    (of & 0x00FF) | (to as u16) << 8
}

#[allow(dead_code)]
pub fn get_most(of: u16) -> u8 {
    (of >> 8) as u8
}

#[allow(dead_code)]
pub fn flip_bit<T>(at: usize, of: T) -> Option<T>
where
    T: Unsigned + PrimInt + NumCast,
{
    if size_of::<T>() * 8 <= at {
        return None;
    }

    let num: Option<T> = NumCast::from(0x01);
    num.map(|n| (n << at) ^ of)
}

#[allow(dead_code)]
pub fn set_bit<T>(at: usize, of: T) -> Option<T>
where
    T: Unsigned + PrimInt + NumCast,
{
    if size_of::<T>() * 8 <= at {
        return None;
    }

    let num: Option<T> = NumCast::from(0x01);
    num.map(|n| (n << at) | of)
}

#[allow(dead_code)]
pub fn unset_bit<T>(at: usize, of: T) -> Option<T>
where
    T: Unsigned + PrimInt + NumCast,
{
    if size_of::<T>() * 8 <= at {
        return None;
    }

    let num: Option<T> = NumCast::from(0x01);
    num.map(|n| !(n << at) & of)
}

#[allow(dead_code)]
pub fn isset_bit<T>(at: usize, of: T) -> Option<bool>
where
    T: Unsigned + PrimInt + NumCast,
{
    if size_of::<T>() * 8 <= at {
        return None;
    }

    let num: Option<T> = NumCast::from(0x01);
    num.map(|n| ((of >> at) & n).is_one())
}
