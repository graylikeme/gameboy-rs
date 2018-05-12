pub trait GameboyMemory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

pub struct Memory {
}

impl GameboyMemory for Memory {
    fn read(&self, addr: u16) -> u8 {
        unimplemented!()
    }

    fn write(&mut self, addr: u16, value: u8) {
        unimplemented!()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {}
    }
}
