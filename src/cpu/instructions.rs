use cpu::LR35902;
use hardware::Bus;

pub fn call(cpu: &mut LR35902, bus: &mut Bus) -> u16 {
    let opcode = bus.read_mem(cpu.get_pc());
    cpu.inc_pc();

    match opcode {
        0xCB => { call_alt(cpu, bus) }
        unknown_op => panic!("Instruction unimplemented: {:2X}", unknown_op)
    }
}

fn call_alt(cpu: &mut LR35902, bus: &mut Bus) -> u16 {
    let opcode = bus.read_mem(cpu.get_pc());
    cpu.inc_pc();

    match opcode {
        unknown_op => panic!("Instruction unimplemented: {:2X}", unknown_op)
    }
}
