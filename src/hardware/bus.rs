use mmu::MMU;

pub struct Bus {
    mmu: Box<dyn MMU>,
}

impl Bus {
    pub fn new(mmu: Box<dyn MMU>) -> Bus {
        Bus { mmu }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mmu.read(addr)
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let high = (self.mmu.read(addr) as u16) << 8;
        let low = self.mmu.read(addr + 1) as u16;
        high | low
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.mmu.write(addr, value);
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        let high = (value >> 8) as u8;
        let low = (value & 0xFF) as u8;
        self.mmu.write(addr, high);
        self.mmu.write(addr + 1, low);
    }
}
