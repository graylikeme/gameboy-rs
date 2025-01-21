extern crate num_traits;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


mod bitwise;
mod cpu;
mod hardware;

use cpu::{GameboyCPU, LR35902};
use hardware::{bus::Bus, cartridge_reader, mmu};
use std::process::exit;

pub struct Gameboy {
    cpu: Box<dyn GameboyCPU>,
    bus: Box<Bus>,
}

impl Gameboy {
    pub fn start(&mut self) {
        loop {
            self.cpu.step(&mut *self.bus);
        }
    }
}

fn main() {
    pretty_env_logger::init_timed();

    let cartridge = cartridge_reader::read("roms/cpu_instrs.gb".to_owned())
        .unwrap_or_else(|err| {
            error!("Failed to load a game cartridge: {}", err.to_string());
            exit(1);
        });

    info!("Cartridge type: {:?}", mmu::CartridgeType::from(cartridge.mem[0x0147]));
    info!("ROM size: {}KB", mmu::get_rom_size(cartridge.mem[0x0148]) / 1024);
    info!("RAM size: {}KB", mmu::get_ram_size(cartridge.mem[0x0149]) / 1024);

    let mmu = mmu::get_mmu(cartridge).unwrap_or_else(|| {
        error!("Cartridge uses an unsupported mmu type");
        exit(1);
    });

    let bus = Box::new(Bus::new(mmu));
    let cpu = Box::new(LR35902::new());

    let mut gb = Gameboy { cpu, bus };
    gb.start();
}
