mod instructions;

use bitwise;
use hardware::Bus;

pub trait GameboyCPU {
    fn step(&mut self, bus: &mut Bus);
}

pub struct LR35902 {
    // General purpose registers
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,

    // Interrupt registers
    if_reg: u8,
    ie_reg: u8,
    ime_reg: u8,
}

impl GameboyCPU for LR35902 {
    fn step(&mut self, bus: &mut Bus) {
        let opcode = bus.read_byte(self.pc);
        let cycles = instructions::call(self, bus);
    }
}

impl LR35902 {
    pub fn new() -> LR35902 {
        LR35902 {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            if_reg: 0,
            ie_reg: 0,
            ime_reg: 0,
        }
    }

    pub fn get_a(&self) -> u8 {
        bitwise::get_most(self.af)
    }

    pub fn set_a(&self, to: u8) {
        bitwise::set_most(self.af, to);
    }

    pub fn get_f(&self) -> u8 {
        bitwise::get_least(self.af)
    }

    pub fn set_f(&self, to: u8) {
        bitwise::set_least(self.af, to);
    }

    pub fn get_b(&self) -> u8 {
        bitwise::get_most(self.bc)
    }

    pub fn set_b(&self, to: u8) {
        bitwise::set_most(self.bc, to);
    }

    pub fn get_c(&self) -> u8 {
        bitwise::get_least(self.bc)
    }

    pub fn set_c(&self, to: u8) {
        bitwise::set_least(self.bc, to);
    }

    pub fn get_d(&self) -> u8 {
        bitwise::get_most(self.de)
    }

    pub fn set_d(&self, to: u8) {
        bitwise::set_most(self.de, to);
    }

    pub fn get_e(&self) -> u8 {
        bitwise::get_least(self.de)
    }

    pub fn set_e(&self, to: u8) {
        bitwise::set_least(self.de, to);
    }

    pub fn get_h(&self) -> u8 {
        bitwise::get_most(self.hl)
    }

    pub fn set_h(&self, to: u8) {
        bitwise::set_most(self.hl, to);
    }

    pub fn get_l(&self) -> u8 {
        bitwise::get_least(self.hl)
    }

    pub fn set_l(&self, to: u8) {
        bitwise::set_least(self.hl, to);
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, to: u16) {
        self.sp = to;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, to: u16) {
        self.pc = to;
    }

    pub fn inc_pc(&mut self) {
        self.pc += 1;
    }

    pub fn dec_pc(&mut self) {
        self.pc -= 1;
    }

    pub fn get_hl(&self) -> u16 {
        self.hl
    }

    pub fn set_hl(&mut self, to: u16) {
        self.hl = to;
    }

    pub fn get_de(&self) -> u16 {
        self.de
    }

    pub fn set_de(&mut self, to: u16) {
        self.de = to;
    }

    pub fn get_bc(&self) -> u16 {
        self.bc
    }

    pub fn set_bc(&mut self, to: u16) {
        self.bc = to;
    }

    pub fn get_af(&self) -> u16 {
        self.af
    }

    pub fn set_af(&mut self, to: u16) {
        self.af = to;
    }
}
