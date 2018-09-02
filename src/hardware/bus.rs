use mmu::MMU;

pub struct Bus {
    mmu: Box<MMU>,
}

impl Bus {
    pub fn new(mmu: Box<MMU>) -> Bus {
        Bus { mmu }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mmu.read(addr)
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let value = (self.mmu.read(addr) as u16) << 8;
        value | (self.mmu.read(addr) as u16)
    }
}
