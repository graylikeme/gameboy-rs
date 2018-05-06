extern crate num_traits;

mod cpu;
mod mmu;
mod bitwise;

use cpu::{ GameboyCPU, LR35902 };

pub struct Gameboy<'a> {
    cpu: &'a mut GameboyCPU,
}

fn main() {
    let mut cpu = LR35902::new();
    let gb = Gameboy { cpu: &mut cpu };
    gb.cpu.step();
}