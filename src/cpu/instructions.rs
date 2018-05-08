use cpu::LR35902;
use hardware::Bus;

pub struct Instruction {}

impl Instruction {
    pub fn get(opcode: u8) -> Instruction {
        unimplemented!()
    }

    pub fn execute(&self, cpu: &mut LR35902, bus: &mut Bus) {
        let pc = cpu.get_pc();
        cpu.set_pc(pc + 1);
    }
}