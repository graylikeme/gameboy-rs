use hardware::cartridge_reader::CartridgeInfo;

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly,
    MBC1,
    MBC1Ram,
    MBC1RamBattery,
    MBC2,
    MBC2Battery,
    MBC3,
    MBC3Ram,
    MBC3RamBattery,
    MBC3TimerBattery,
    MBC3TimerRamBattery,
    MBC5,
    MBC5Ram,
    MBC5RamBattery,
    Unsupported(u8),
}

impl From<u8> for CartridgeType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => CartridgeType::RomOnly,
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1Ram,
            0x03 => CartridgeType::MBC1RamBattery,
            0x05 => CartridgeType::MBC2,
            0x06 => CartridgeType::MBC2Battery,
            0x0F => CartridgeType::MBC3TimerBattery,
            0x10 => CartridgeType::MBC3TimerRamBattery,
            0x11 => CartridgeType::MBC3,
            0x12 => CartridgeType::MBC3Ram,
            0x13 => CartridgeType::MBC3RamBattery,
            0x19 => CartridgeType::MBC5,
            0x1A => CartridgeType::MBC5Ram,
            0x1B => CartridgeType::MBC5RamBattery,
            unknown => CartridgeType::Unsupported(unknown),
        }
    }
}

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
            unknown_addr => warn!(
                "Attempted to write to unreachable address: {:2X}",
                unknown_addr
            ),
        }
    }
}

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enabled: bool,
    mode: bool, // false = ROM Banking, true = RAM Banking
}

impl MBC1 {
    pub fn new(rom: Vec<u8>, ram_size: usize) -> Self {
        MBC1 {
            rom,
            ram: vec![0; ram_size],
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            mode: false,
        }
    }
}

impl MMU for MBC1 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // Fixed ROM bank 0
            0x0000..=0x3FFF => self.rom[addr as usize],
            
            // Switchable ROM bank 01-7F
            0x4000..=0x7FFF => {
                let bank_offset = self.rom_bank * 0x4000;
                let addr_offset = (addr as usize) - 0x4000;
                self.rom[bank_offset + addr_offset]
            },
            
            // RAM banks (if enabled)
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                }
                let bank = if self.mode { self.ram_bank } else { 0 };
                let offset = bank * 0x2000;
                let addr_offset = (addr as usize) - 0xA000;
                self.ram[offset + addr_offset]
            },
            
            _ => 0xFF,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM Enable
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            },
            
            // ROM Bank Number (lower 5 bits)
            0x2000..=0x3FFF => {
                let mut bank = (value & 0x1F) as usize;
                if bank == 0 {
                    bank = 1;
                }
                self.rom_bank = (self.rom_bank & 0x60) | bank;
            },
            
            // ROM/RAM Bank Number (upper 2 bits)
            0x4000..=0x5FFF => {
                let bank = (value & 0x03) as usize;
                if self.mode {
                    self.ram_bank = bank;
                } else {
                    self.rom_bank = (self.rom_bank & 0x1F) | (bank << 5);
                }
            },
            
            // Banking Mode Select
            0x6000..=0x7FFF => {
                self.mode = (value & 0x01) != 0;
            },
            
            // RAM Banks
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return;
                }
                let bank = if self.mode { self.ram_bank } else { 0 };
                let offset = bank * 0x2000;
                let addr_offset = (addr as usize) - 0xA000;
                self.ram[offset + addr_offset] = value;
            },
            
            _ => warn!("Attempted to write to unreachable address: {:04X}", addr),
        }
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
            0xA000..=0xBFFF => self.ram[(addr - 0xA000) as usize] = value,
            unknown_addr => warn!(
                "Attempted to write to unreachable address: {:2X}",
                unknown_addr
            ),
        }
    }
}

pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rtc: [u8; 5],  // RTC registers: S, M, H, DL, DH
    rom_bank: usize,
    ram_bank: usize,
    ram_rtc_enabled: bool,
    rtc_selected: bool,
    rtc_reg: usize,
}

impl MBC3 {
    pub fn new(rom: Vec<u8>, ram_size: usize) -> Self {
        MBC3 {
            rom,
            ram: vec![0; ram_size],
            rtc: [0; 5],
            rom_bank: 1,
            ram_bank: 0,
            ram_rtc_enabled: false,
            rtc_selected: false,
            rtc_reg: 0,
        }
    }
}

impl MMU for MBC3 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // Fixed ROM bank 0
            0x0000..=0x3FFF => self.rom[addr as usize],
            
            // Switchable ROM bank 01-7F
            0x4000..=0x7FFF => {
                let bank_offset = self.rom_bank * 0x4000;
                let addr_offset = (addr as usize) - 0x4000;
                self.rom[bank_offset + addr_offset]
            },
            
            // RAM banks or RTC register
            0xA000..=0xBFFF => {
                if !self.ram_rtc_enabled {
                    return 0xFF;
                }
                if self.rtc_selected {
                    self.rtc[self.rtc_reg]
                } else {
                    let offset = self.ram_bank * 0x2000;
                    let addr_offset = (addr as usize) - 0xA000;
                    self.ram[offset + addr_offset]
                }
            },
            
            _ => 0xFF,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM/RTC Enable
            0x0000..=0x1FFF => {
                self.ram_rtc_enabled = (value & 0x0F) == 0x0A;
            },
            
            // ROM Bank Number
            0x2000..=0x3FFF => {
                let bank = if value == 0 { 1 } else { value as usize };
                self.rom_bank = bank;
            },
            
            // RAM Bank Number / RTC Register Select
            0x4000..=0x5FFF => {
                match value {
                    0x00..=0x03 => {
                        self.ram_bank = value as usize;
                        self.rtc_selected = false;
                    },
                    0x08..=0x0C => {
                        self.rtc_reg = (value - 0x08) as usize;
                        self.rtc_selected = true;
                    },
                    _ => warn!("Invalid RAM bank / RTC register: {:02X}", value),
                }
            },
            
            // RAM Bank / RTC Register Write
            0xA000..=0xBFFF => {
                if !self.ram_rtc_enabled {
                    return;
                }
                if self.rtc_selected {
                    self.rtc[self.rtc_reg] = value;
                } else {
                    let offset = self.ram_bank * 0x2000;
                    let addr_offset = (addr as usize) - 0xA000;
                    self.ram[offset + addr_offset] = value;
                }
            },
            
            _ => warn!("Attempted to write to unreachable address: {:04X}", addr),
        }
    }
}

pub struct MBC5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enabled: bool,
}

impl MBC5 {
    pub fn new(rom: Vec<u8>, ram_size: usize) -> Self {
        MBC5 {
            rom,
            ram: vec![0; ram_size],
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
        }
    }
}

impl MMU for MBC5 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // Fixed ROM bank 0
            0x0000..=0x3FFF => self.rom[addr as usize],
            
            // Switchable ROM bank 000-1FF
            0x4000..=0x7FFF => {
                let bank_offset = self.rom_bank * 0x4000;
                let addr_offset = (addr as usize) - 0x4000;
                self.rom[bank_offset + addr_offset]
            },
            
            // RAM banks
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                }
                let offset = self.ram_bank * 0x2000;
                let addr_offset = (addr as usize) - 0xA000;
                self.ram[offset + addr_offset]
            },
            
            _ => 0xFF,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM Enable
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            },
            
            // ROM Bank Number (lower 8 bits)
            0x2000..=0x2FFF => {
                self.rom_bank = (self.rom_bank & 0x100) | (value as usize);
            },
            
            // ROM Bank Number (upper 1 bit)
            0x3000..=0x3FFF => {
                self.rom_bank = (self.rom_bank & 0xFF) | (((value & 0x01) as usize) << 8);
            },
            
            // RAM Bank Number
            0x4000..=0x5FFF => {
                self.ram_bank = (value & 0x0F) as usize;
            },
            
            // RAM Banks
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return;
                }
                let offset = self.ram_bank * 0x2000;
                let addr_offset = (addr as usize) - 0xA000;
                self.ram[offset + addr_offset] = value;
            },
            
            _ => warn!("Attempted to write to unreachable address: {:04X}", addr),
        }
    }
}

pub fn get_rom_size(byte: u8) -> usize {
    match byte {
        0x00 => 32 * 1024,     // 32KB (2 banks)
        0x01 => 64 * 1024,     // 64KB (4 banks)
        0x02 => 128 * 1024,    // 128KB (8 banks)
        0x03 => 256 * 1024,    // 256KB (16 banks)
        0x04 => 512 * 1024,    // 512KB (32 banks)
        0x05 => 1024 * 1024,   // 1MB (64 banks)
        0x06 => 2048 * 1024,   // 2MB (128 banks)
        0x07 => 4096 * 1024,   // 4MB (256 banks)
        0x08 => 8192 * 1024,   // 8MB (512 banks)
        _ => 32 * 1024,        // Default to 32KB
    }
}

pub fn get_ram_size(byte: u8) -> usize {
    match byte {
        0x00 => 0,             // No RAM
        0x01 => 2 * 1024,      // 2KB
        0x02 => 8 * 1024,      // 8KB
        0x03 => 32 * 1024,     // 32KB (4 banks of 8KB)
        0x04 => 128 * 1024,    // 128KB (16 banks of 8KB)
        0x05 => 64 * 1024,     // 64KB (8 banks of 8KB)
        _ => 0,                // Default to no RAM
    }
}

pub fn get_mmu(cartridge: CartridgeInfo) -> Option<Box<dyn MMU>> {
    let cartridge_type = CartridgeType::from(cartridge.mem[0x0147]);
    let rom_size = get_rom_size(cartridge.mem[0x0148]);
    let ram_size = get_ram_size(cartridge.mem[0x0149]);

    match cartridge_type {
        CartridgeType::RomOnly => Some(Box::new(MBCNone {
            rom: cartridge.mem,
            ram: vec![0; ram_size],
        })),
        CartridgeType::MBC2 | CartridgeType::MBC2Battery => Some(Box::new(MBC2 {
            rom: cartridge.mem,
            ram: vec![0; 512], // MBC2 has 512x4 bits built-in RAM
            rom_offset: 0x4000,
            rom_size: rom_size,
        })),
        CartridgeType::MBC1 | CartridgeType::MBC1Ram | CartridgeType::MBC1RamBattery => {
            Some(Box::new(MBC1::new(cartridge.mem, ram_size)))
        },
        CartridgeType::MBC3 | CartridgeType::MBC3Ram | CartridgeType::MBC3RamBattery |
        CartridgeType::MBC3TimerBattery | CartridgeType::MBC3TimerRamBattery => {
            Some(Box::new(MBC3::new(cartridge.mem, ram_size)))
        },
        CartridgeType::MBC5 | CartridgeType::MBC5Ram | CartridgeType::MBC5RamBattery => {
            Some(Box::new(MBC5::new(cartridge.mem, ram_size)))
        },
        CartridgeType::Unsupported(t) => {
            warn!("Unsupported cartridge type: 0x{:02X}", t);
            None
        },
    }
}
