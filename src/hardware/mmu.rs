const ROM_SIZE: usize = 0x4000;
const RAM_SIZE: usize = 0x2000;

pub trait GameboyMemory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

pub struct Memory {
    rom: [u8; ROM_SIZE],
    ram: [u8; RAM_SIZE],
    rom_offset: usize,
    rom_size: usize,
}

impl GameboyMemory for Memory {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => self.rom[self.rom_offset + (addr as usize - 0x4000)],
            0xA000..=0xBFFF => self.ram[addr as usize - 0xA000] & 0x0F,
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

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom: [0; ROM_SIZE],
            ram: [0; RAM_SIZE],
            rom_offset: 0,
            rom_size: 0,
        }
    }
}
