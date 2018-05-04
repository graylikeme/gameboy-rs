mod bitwise;

pub struct LR35902 {
    a: u8,
    f: u8,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16
}

impl LR35902 {
    pub fn get_b(&self) -> u8 {
        bitwise::get_least(self.bc)
    }

    pub fn set_b(&self, to: u8) {
        bitwise::set_least(self.bc, to);
    }

    pub fn get_c(&self) -> u8 {
        bitwise::get_most(self.bc)
    }

    pub fn set_c(&self, to: u8) {
        bitwise::set_most(self.bc, to);
    }

    pub fn get_d(&self) -> u8 {
        bitwise::get_least(self.de)
    }

    pub fn set_d(&self, to: u8) {
        bitwise::set_least(self.de, to);
    }

    pub fn get_e(&self) -> u8 {
        bitwise::get_most(self.de)
    }

    pub fn set_e(&self, to: u8) {
        bitwise::set_most(self.de, to);
    }

    pub fn get_h(&self) -> u8 {
        bitwise::get_least(self.de)
    }

    pub fn set_h(&self, to: u8) {
        bitwise::set_least(self.de, to);
    }

    pub fn get_l(&self) -> u8 {
        bitwise::get_most(self.de)
    }

    pub fn set_l(&self, to: u8) {
        bitwise::set_most(self.de, to);
    }
}

fn main() {

}