pub mod mmu;

use self::mmu::GameboyMemory;

pub struct Bus {
    mmu: Box<GameboyMemory>,
}

impl Bus {
    pub fn new(mmu: Box<GameboyMemory>) -> Bus {
        Bus { mmu }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        self.mmu.read(addr)
    }
}
