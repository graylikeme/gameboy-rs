use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct CartridgeInfo {
    pub mem: Vec<u8>,
    pub size: usize,
}

pub fn read(cartridge_path: String) -> Result<CartridgeInfo, io::Error> {
    let mut mem: Vec<u8> = Vec::new();
    File::open(cartridge_path).and_then(|mut fp| {
        fp.read_to_end(&mut mem)
            .map(|size| CartridgeInfo { mem, size })
    })
}
