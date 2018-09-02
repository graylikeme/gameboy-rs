use hardware::cartridge_reader::CartridgeInfo;

pub trait MMU {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

pub struct MBCNone {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl MMU for MBCNone {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0xA000..=0xBFFF => self.ram[(addr - 0xA000) as usize],
            _ => 0xFF,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xA000..=0xBFFF => self.ram[(addr - 0xA000) as usize] = value,
            unknown_addr => println!(
                "Attempted to write to unreachable address: {:2X}",
                unknown_addr
            ),
        }
    }
}

pub struct MBC1 {}

impl MMU for MBC1 {
    fn read(&self, addr: u16) -> u8 {
        unimplemented!()
    }

    fn write(&mut self, addr: u16, value: u8) {
        unimplemented!()
    }
}

pub struct MBC2 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_offset: usize,
    rom_size: usize,
}

impl MMU for MBC2 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => self.rom[self.rom_offset + (addr as usize - 0x4000)],
            0xA000..=0xBFFF => self.ram[(addr - 0xA000) as usize] & 0x0F,
            _ => 0xFF,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000..=0x3FFF => {
                let mut offset = (value & 0x0F) as usize;
                offset %= self.rom_size;
                offset = if offset == 0 { 1 } else { offset };
                self.rom_offset = offset * self.rom_size;
            }
            0xA000..=0xBFFF => self.ram[addr as usize] = value,
            unknown_addr => println!(
                "Attempted to write to unreachable address: {:2X}",
                unknown_addr
            ),
        }
    }
}

pub struct MBC3 {}

impl MMU for MBC3 {
    fn read(&self, addr: u16) -> u8 {
        unimplemented!()
    }

    fn write(&mut self, addr: u16, value: u8) {
        unimplemented!()
    }
}

pub struct MBC5 {}

impl MMU for MBC5 {
    fn read(&self, addr: u16) -> u8 {
        unimplemented!()
    }

    fn write(&mut self, addr: u16, value: u8) {
        unimplemented!()
    }
}

pub fn get_mmu(cartridge: CartridgeInfo) -> Option<Box<MMU>> {
    Some(Box::new(MBCNone {
        rom: vec![0; 0x4000],
        ram: vec![0; 0x2000],
    }))
}
