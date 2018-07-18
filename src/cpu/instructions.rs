use cpu::LR35902;
use hardware::Bus;

pub fn call(cpu: &mut LR35902, bus: &mut Bus) -> u16 {
    let opcode = bus.read_byte(cpu.get_pc());
    cpu.inc_pc();

    match opcode {
        0x00 => 4,
        0x01 => {
            let value = bus.read_word(cpu.get_pc());
            cpu.inc_pc();
            cpu.inc_pc();
            cpu.set_bc(value);
            12
        }
        0x02 => {
            let a_reg = cpu.get_a();
            cpu.set_bc(a_reg as u16);
            8
        }
        0x03 => {
            let bc_reg = cpu.get_bc();
            cpu.set_bc(bc_reg + 1);
            8
        }
        0x04 => {
            let b_reg = cpu.get_b();
            let new_b_reg = b_reg + 1;
            cpu.flags.half_carry = ((b_reg | 0x0F) + 1) >> 4 > 0;
            cpu.flags.sub = false;
            cpu.flags.zero = new_b_reg == 0;
            cpu.set_b(new_b_reg);
            4
        }
        0x05 => {
            let b_reg = cpu.get_b();
            let new_b_reg = b_reg - 1;
            cpu.flags.half_carry = ((b_reg | 0x0F) - 1) >> 3 > 0;
            cpu.flags.sub = true;
            cpu.flags.zero = new_b_reg == 0;
            cpu.set_b(new_b_reg);
            4
        }
        0x06 => {
            let value = bus.read_byte(cpu.get_pc());
            cpu.inc_pc();
            cpu.set_b(value);
            8
        }
        0xCB => call_alt(cpu, bus),
        unknown_op => panic!("Instruction unimplemented: {:2X}", unknown_op),
    }
}

fn call_alt(cpu: &mut LR35902, bus: &mut Bus) -> u16 {
    let opcode = bus.read_byte(cpu.get_pc());
    cpu.inc_pc();

    match opcode {
        unknown_op => panic!("Instruction unimplemented: {:2X}", unknown_op),
    }
}
