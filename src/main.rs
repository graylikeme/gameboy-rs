extern crate num_traits;

mod cpu;
mod hardware;
mod bitwise;

use cpu::{ GameboyCPU, LR35902 };
use hardware::{
    Bus,
    mmu::{ GameboyMemory, Memory }
};

pub struct Gameboy {
    cpu: Box<GameboyCPU>,
    bus: Box<Bus>,
}

impl Gameboy {
    pub fn start(&mut self) {
        loop {
            self.cpu.step(&mut (*self.bus));
        }
    }
}

fn main() {
    let cpu = Box::new(LR35902::new());
    let mmu = Box::new(Memory::new());
    let bus = Box::new(Bus::new(mmu));

    let mut gb = Gameboy { cpu, bus };
    gb.start();
}