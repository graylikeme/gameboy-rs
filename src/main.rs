extern crate num_traits;

mod bitwise;
mod cpu;
mod hardware;

use cpu::{GameboyCPU, LR35902};
use hardware::{bus::Bus, cartridge_reader, mmu};
use std::process::exit;

pub struct Gameboy {
    cpu: Box<GameboyCPU>,
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
    let cartridge = cartridge_reader::read("".to_owned()).unwrap_or_else(|err| {
        println!("Failed to load a game cartridge: {:?}", err);
        exit(1);
    });

    let mmu = mmu::get_mmu(cartridge).unwrap_or_else(|| {
        println!("Cartridge uses an unsupported mmu type");
        exit(1);
    });

    let bus = Box::new(Bus::new(mmu));
    let cpu = Box::new(LR35902::new());

    let mut gb = Gameboy { cpu, bus };
    gb.start();
}
