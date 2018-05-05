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