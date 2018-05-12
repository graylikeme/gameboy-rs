pub mod mmu;

use self::mmu::MMU;

pub struct Bus {
    mmu: Box<MMU>,
}

impl Bus {
    pub fn new(mmu: Box<MMU>) -> Bus {
        Bus { mmu }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        self.mmu.read(addr)
    }
}
