use cpu::LR35902;
use hardware::bus::Bus;

macro_rules! half_carry_add_u8 {
    ($x:expr, $y:expr) => {
        (($x & 0x0F) + ($y & 0x0F)) & 0x10 == 0x10
    };
}

macro_rules! half_carry_sub_u8 {
    ($x:expr, $y:expr) => {
        ($x & 0x0F) < ($y & 0x0F)
    };
}

type GetRegFn = fn(&LR35902) -> u8;
type SetRegFn = fn(&mut LR35902, u8);
type SetRegFn16 = fn(&mut LR35902, u16);

// 8-bit load instructions
fn ld_r_n(cpu: &mut LR35902, bus: &mut Bus, set_reg: SetRegFn) -> u16 {
    let value = bus.read_byte(cpu.get_pc());
    cpu.inc_pc();
    set_reg(cpu, value);
    8
}

fn ld_r_r(cpu: &mut LR35902, get_reg: GetRegFn, set_reg: SetRegFn) -> u16 {
    let value = get_reg(cpu);
    set_reg(cpu, value);
    4
}

fn ld_r_hl(cpu: &mut LR35902, bus: &mut Bus, set_reg: SetRegFn) -> u16 {
    let addr = cpu.get_hl();
    let value = bus.read_byte(addr);
    set_reg(cpu, value);
    8
}

fn ld_hl_r(cpu: &mut LR35902, bus: &mut Bus, get_reg: GetRegFn) -> u16 {
    let addr = cpu.get_hl();
    let value = get_reg(cpu);
    bus.write_byte(addr, value);
    8
}

// 16-bit load instructions
fn ld_rr_nn(cpu: &mut LR35902, bus: &mut Bus, set_reg: SetRegFn16) -> u16 {
    let value = bus.read_word(cpu.get_pc());
    cpu.inc_pc();
    cpu.inc_pc();
    set_reg(cpu, value);
    12
}

// 8-bit arithmetic
fn add_a(cpu: &mut LR35902, value: u8) {
    let a = cpu.get_a();
    let result = a.wrapping_add(value);
    cpu.flags.zero = result == 0;
    cpu.flags.sub = false;
    cpu.flags.half_carry = half_carry_add_u8!(a, value);
    cpu.flags.carry = (a as u16 + value as u16) > 0xFF;
    cpu.set_a(result);
}

fn sub_a(cpu: &mut LR35902, value: u8) {
    let a = cpu.get_a();
    let result = a.wrapping_sub(value);
    cpu.flags.zero = result == 0;
    cpu.flags.sub = true;
    cpu.flags.half_carry = half_carry_sub_u8!(a, value);
    cpu.flags.carry = a < value;
    cpu.set_a(result);
}

fn and_a(cpu: &mut LR35902, value: u8) {
    let a = cpu.get_a();
    let result = a & value;
    cpu.flags.zero = result == 0;
    cpu.flags.sub = false;
    cpu.flags.half_carry = true;
    cpu.flags.carry = false;
    cpu.set_a(result);
}

fn or_a(cpu: &mut LR35902, value: u8) {
    let a = cpu.get_a();
    let result = a | value;
    cpu.flags.zero = result == 0;
    cpu.flags.sub = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = false;
    cpu.set_a(result);
}

fn xor_a(cpu: &mut LR35902, value: u8) {
    let a = cpu.get_a();
    let result = a ^ value;
    cpu.flags.zero = result == 0;
    cpu.flags.sub = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = false;
    cpu.set_a(result);
}

fn cp_a(cpu: &mut LR35902, value: u8) {
    let a = cpu.get_a();
    let result = a.wrapping_sub(value);
    cpu.flags.zero = result == 0;
    cpu.flags.sub = true;
    cpu.flags.half_carry = half_carry_sub_u8!(a, value);
    cpu.flags.carry = a < value;
}

fn inc_r(cpu: &mut LR35902, get_reg: GetRegFn, set_reg: SetRegFn) -> u16 {
    let value = get_reg(cpu);
    let result = value.wrapping_add(1);
    cpu.flags.zero = result == 0;
    cpu.flags.sub = false;
    cpu.flags.half_carry = half_carry_add_u8!(value, 1);
    set_reg(cpu, result);
    4
}

fn dec_r(cpu: &mut LR35902, get_reg: GetRegFn, set_reg: SetRegFn) -> u16 {
    let value = get_reg(cpu);
    let result = value.wrapping_sub(1);
    cpu.flags.zero = result == 0;
    cpu.flags.sub = true;
    cpu.flags.half_carry = half_carry_sub_u8!(value, 1);
    set_reg(cpu, result);
    4
}

// 16-bit arithmetic
fn add_hl(cpu: &mut LR35902, value: u16) {
    let hl = cpu.get_hl();
    let result = hl.wrapping_add(value);
    cpu.flags.sub = false;
    cpu.flags.half_carry = (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
    cpu.flags.carry = (hl as u32 + value as u32) > 0xFFFF;
    cpu.set_hl(result);
}

// Control flow
fn jr_cc_n(cpu: &mut LR35902, bus: &mut Bus, condition: bool) -> u16 {
    let offset = bus.read_byte(cpu.get_pc()) as i8;
    cpu.inc_pc();
    if condition {
        let pc = cpu.get_pc();
        cpu.set_pc((pc as i32 + offset as i32) as u16);
        12
    } else {
        8
    }
}

fn call_nn(cpu: &mut LR35902, bus: &mut Bus) -> u16 {
    let addr = bus.read_word(cpu.get_pc());
    cpu.inc_pc();
    cpu.inc_pc();
    let sp = cpu.get_sp().wrapping_sub(2);
    cpu.set_sp(sp);
    bus.write_word(sp, cpu.get_pc());
    cpu.set_pc(addr);
    24
}

fn ret(cpu: &mut LR35902, bus: &mut Bus) -> u16 {
    let sp = cpu.get_sp();
    let addr = bus.read_word(sp);
    cpu.set_sp(sp.wrapping_add(2));
    cpu.set_pc(addr);
    16
}

pub fn call(cpu: &mut LR35902, bus: &mut Bus) -> u16 {
    let opcode = bus.read_byte(cpu.get_pc());
    cpu.inc_pc();

    match opcode {
        // 8-bit loads
        0x00 => 4, // NOP
        0x01 => ld_rr_nn(cpu, bus, LR35902::set_bc as SetRegFn16),
        0x02 => { let a = cpu.get_a(); bus.write_byte(cpu.get_bc(), a); 8 },
        0x03 => { let bc = cpu.get_bc(); cpu.set_bc(bc.wrapping_add(1)); 8 },
        0x04 => inc_r(cpu, LR35902::get_b as GetRegFn, LR35902::set_b as SetRegFn),
        0x05 => dec_r(cpu, LR35902::get_b as GetRegFn, LR35902::set_b as SetRegFn),
        0x06 => ld_r_n(cpu, bus, LR35902::set_b as SetRegFn),
        0x0A => { let a = bus.read_byte(cpu.get_bc()); cpu.set_a(a); 8 },
        0x0E => ld_r_n(cpu, bus, LR35902::set_c as SetRegFn),
        
        0x11 => ld_rr_nn(cpu, bus, LR35902::set_de as SetRegFn16),
        0x12 => { let a = cpu.get_a(); bus.write_byte(cpu.get_de(), a); 8 },
        0x16 => ld_r_n(cpu, bus, LR35902::set_d as SetRegFn),
        0x1A => { let a = bus.read_byte(cpu.get_de()); cpu.set_a(a); 8 },
        0x1E => ld_r_n(cpu, bus, LR35902::set_e as SetRegFn),
        
        0x21 => ld_rr_nn(cpu, bus, LR35902::set_hl as SetRegFn16),
        0x22 => { 
            let a = cpu.get_a(); 
            let hl = cpu.get_hl();
            bus.write_byte(hl, a); 
            cpu.set_hl(hl.wrapping_add(1)); 
            8 
        },
        0x26 => ld_r_n(cpu, bus, LR35902::set_h as SetRegFn),
        0x2A => { 
            let hl = cpu.get_hl();
            let a = bus.read_byte(hl); 
            cpu.set_a(a);
            cpu.set_hl(hl.wrapping_add(1));
            8 
        },
        0x2E => ld_r_n(cpu, bus, LR35902::set_l as SetRegFn),
        
        0x31 => ld_rr_nn(cpu, bus, LR35902::set_sp as SetRegFn16),
        0x32 => { 
            let a = cpu.get_a(); 
            let hl = cpu.get_hl();
            bus.write_byte(hl, a); 
            cpu.set_hl(hl.wrapping_sub(1)); 
            8 
        },
        0x36 => {
            let value = bus.read_byte(cpu.get_pc());
            cpu.inc_pc();
            bus.write_byte(cpu.get_hl(), value);
            12
        },
        0x3A => { 
            let hl = cpu.get_hl();
            let a = bus.read_byte(hl); 
            cpu.set_a(a);
            cpu.set_hl(hl.wrapping_sub(1));
            8 
        },
        0x3C => inc_r(cpu, LR35902::get_a as GetRegFn, LR35902::set_a as SetRegFn),  // INC A
        0x3D => dec_r(cpu, LR35902::get_a as GetRegFn, LR35902::set_a as SetRegFn),  // DEC A
        0x3E => ld_r_n(cpu, bus, LR35902::set_a as SetRegFn),

        0x0C => inc_r(cpu, LR35902::get_c as GetRegFn, LR35902::set_c as SetRegFn),  // INC C
        0x0D => dec_r(cpu, LR35902::get_c as GetRegFn, LR35902::set_c as SetRegFn),  // DEC C
        0x14 => inc_r(cpu, LR35902::get_d as GetRegFn, LR35902::set_d as SetRegFn),  // INC D
        0x15 => dec_r(cpu, LR35902::get_d as GetRegFn, LR35902::set_d as SetRegFn),  // DEC D
        0x1C => inc_r(cpu, LR35902::get_e as GetRegFn, LR35902::set_e as SetRegFn),  // INC E
        0x1D => dec_r(cpu, LR35902::get_e as GetRegFn, LR35902::set_e as SetRegFn),  // DEC E
        0x24 => inc_r(cpu, LR35902::get_h as GetRegFn, LR35902::set_h as SetRegFn),  // INC H
        0x25 => dec_r(cpu, LR35902::get_h as GetRegFn, LR35902::set_h as SetRegFn),  // DEC H
        0x2C => inc_r(cpu, LR35902::get_l as GetRegFn, LR35902::set_l as SetRegFn),  // INC L
        0x2D => dec_r(cpu, LR35902::get_l as GetRegFn, LR35902::set_l as SetRegFn),  // DEC L

        // 8-bit arithmetic
        0x80..=0x87 => {
            let value = match opcode & 0x07 {
                0x0 => cpu.get_b(),
                0x1 => cpu.get_c(),
                0x2 => cpu.get_d(),
                0x3 => cpu.get_e(),
                0x4 => cpu.get_h(),
                0x5 => cpu.get_l(),
                0x6 => bus.read_byte(cpu.get_hl()),
                0x7 => cpu.get_a(),
                _ => unreachable!()
            };
            add_a(cpu, value);
            if (opcode & 0x07) == 0x06 { 8 } else { 4 }
        },

        0x90..=0x97 => {
            let value = match opcode & 0x07 {
                0x0 => cpu.get_b(),
                0x1 => cpu.get_c(),
                0x2 => cpu.get_d(),
                0x3 => cpu.get_e(),
                0x4 => cpu.get_h(),
                0x5 => cpu.get_l(),
                0x6 => bus.read_byte(cpu.get_hl()),
                0x7 => cpu.get_a(),
                _ => unreachable!()
            };
            sub_a(cpu, value);
            if (opcode & 0x07) == 0x06 { 8 } else { 4 }
        },

        0xA0..=0xA7 => {
            let value = match opcode & 0x07 {
                0x0 => cpu.get_b(),
                0x1 => cpu.get_c(),
                0x2 => cpu.get_d(),
                0x3 => cpu.get_e(),
                0x4 => cpu.get_h(),
                0x5 => cpu.get_l(),
                0x6 => bus.read_byte(cpu.get_hl()),
                0x7 => cpu.get_a(),
                _ => unreachable!()
            };
            and_a(cpu, value);
            if (opcode & 0x07) == 0x06 { 8 } else { 4 }
        },

        0xB0..=0xB7 => {
            let value = match opcode & 0x07 {
                0x0 => cpu.get_b(),
                0x1 => cpu.get_c(),
                0x2 => cpu.get_d(),
                0x3 => cpu.get_e(),
                0x4 => cpu.get_h(),
                0x5 => cpu.get_l(),
                0x6 => bus.read_byte(cpu.get_hl()),
                0x7 => cpu.get_a(),
                _ => unreachable!()
            };
            or_a(cpu, value);
            if (opcode & 0x07) == 0x06 { 8 } else { 4 }
        },

        // Control flow
        // Stack operations
        0xC1 => { let value = bus.read_word(cpu.get_sp()); cpu.set_sp(cpu.get_sp().wrapping_add(2)); cpu.set_bc(value); 12 },
        0xC5 => { let sp = cpu.get_sp().wrapping_sub(2); cpu.set_sp(sp); bus.write_word(sp, cpu.get_bc()); 16 },
        0xD1 => { let value = bus.read_word(cpu.get_sp()); cpu.set_sp(cpu.get_sp().wrapping_add(2)); cpu.set_de(value); 12 },
        0xD5 => { let sp = cpu.get_sp().wrapping_sub(2); cpu.set_sp(sp); bus.write_word(sp, cpu.get_de()); 16 },
        0xE1 => { let value = bus.read_word(cpu.get_sp()); cpu.set_sp(cpu.get_sp().wrapping_add(2)); cpu.set_hl(value); 12 },
        0xE5 => { let sp = cpu.get_sp().wrapping_sub(2); cpu.set_sp(sp); bus.write_word(sp, cpu.get_hl()); 16 },
        0xF1 => { let value = bus.read_word(cpu.get_sp()); cpu.set_sp(cpu.get_sp().wrapping_add(2)); cpu.set_af(value); 12 },
        0xF5 => { let sp = cpu.get_sp().wrapping_sub(2); cpu.set_sp(sp); bus.write_word(sp, cpu.get_af()); 16 },

        // 16-bit arithmetic
        0x09 => { let value = cpu.get_bc(); add_hl(cpu, value); 8 },
        0x19 => { let value = cpu.get_de(); add_hl(cpu, value); 8 },
        0x29 => { let value = cpu.get_hl(); add_hl(cpu, value); 8 },
        0x39 => { let value = cpu.get_sp(); add_hl(cpu, value); 8 },

        // Control flow
        0x18 => jr_cc_n(cpu, bus, true),  // JR n
        0x20 => jr_cc_n(cpu, bus, !cpu.flags.zero),  // JR NZ,n
        0x28 => jr_cc_n(cpu, bus, cpu.flags.zero),   // JR Z,n
        0x30 => jr_cc_n(cpu, bus, !cpu.flags.carry), // JR NC,n
        0x38 => jr_cc_n(cpu, bus, cpu.flags.carry),  // JR C,n

        0xC0 => if !cpu.flags.zero { ret(cpu, bus) } else { 8 },  // RET NZ
        0xC2 => {  // JP NZ,nn
            let addr = bus.read_word(cpu.get_pc());
            cpu.inc_pc();
            cpu.inc_pc();
            if !cpu.flags.zero {
                cpu.set_pc(addr);
                16
            } else {
                12
            }
        },
        0xC3 => {  // JP nn
            let addr = bus.read_word(cpu.get_pc());
            cpu.inc_pc();
            cpu.inc_pc();
            cpu.set_pc(addr);
            16
        },
        0xC4 => {  // CALL NZ,nn
            let addr = bus.read_word(cpu.get_pc());
            cpu.inc_pc();
            cpu.inc_pc();
            if !cpu.flags.zero {
                let sp = cpu.get_sp().wrapping_sub(2);
                cpu.set_sp(sp);
                bus.write_word(sp, cpu.get_pc());
                cpu.set_pc(addr);
                24
            } else {
                12
            }
        },
        0xC8 => if cpu.flags.zero { ret(cpu, bus) } else { 8 },   // RET Z
        0xC9 => ret(cpu, bus),  // RET
        0xCA => {  // JP Z,nn
            let addr = bus.read_word(cpu.get_pc());
            cpu.inc_pc();
            cpu.inc_pc();
            if cpu.flags.zero {
                cpu.set_pc(addr);
                16
            } else {
                12
            }
        },
        0xCC => {  // CALL Z,nn
            let addr = bus.read_word(cpu.get_pc());
            cpu.inc_pc();
            cpu.inc_pc();
            if cpu.flags.zero {
                let sp = cpu.get_sp().wrapping_sub(2);
                cpu.set_sp(sp);
                bus.write_word(sp, cpu.get_pc());
                cpu.set_pc(addr);
                24
            } else {
                12
            }
        },
        0xCD => call_nn(cpu, bus),  // CALL nn

        // Interrupt handling
        0xF3 => { cpu.ime_reg = 0; 4 },  // DI
        0xFB => { cpu.ime_reg = 1; 4 },  // EI

        0xCB => call_alt(cpu, bus),

        unknown_op => panic!("Instruction unimplemented: {:2X}", unknown_op),
    }
}

fn call_alt(cpu: &mut LR35902, bus: &mut Bus) -> u16 {
    let opcode = bus.read_byte(cpu.get_pc());
    cpu.inc_pc();

    match opcode {
        // Rotates and shifts
        0x00..=0x07 => { // RLC r
            let value = match opcode & 0x07 {
                0x0 => cpu.get_b(),
                0x1 => cpu.get_c(),
                0x2 => cpu.get_d(),
                0x3 => cpu.get_e(),
                0x4 => cpu.get_h(),
                0x5 => cpu.get_l(),
                0x6 => bus.read_byte(cpu.get_hl()),
                0x7 => cpu.get_a(),
                _ => unreachable!()
            };
            let carry = (value & 0x80) != 0;
            let result = (value << 1) | (if carry { 1 } else { 0 });
            
            cpu.flags.zero = result == 0;
            cpu.flags.sub = false;
            cpu.flags.half_carry = false;
            cpu.flags.carry = carry;

            match opcode & 0x07 {
                0x0 => (LR35902::set_b as SetRegFn)(cpu, result),
                0x1 => (LR35902::set_c as SetRegFn)(cpu, result),
                0x2 => (LR35902::set_d as SetRegFn)(cpu, result),
                0x3 => (LR35902::set_e as SetRegFn)(cpu, result),
                0x4 => (LR35902::set_h as SetRegFn)(cpu, result),
                0x5 => (LR35902::set_l as SetRegFn)(cpu, result),
                0x6 => bus.write_byte(cpu.get_hl(), result),
                0x7 => (LR35902::set_a as SetRegFn)(cpu, result),
                _ => unreachable!()
            };
            
            if (opcode & 0x07) == 0x06 { 16 } else { 8 }
        },

        // Bit operations
        0x40..=0x7F => { // BIT n,r
            let bit = (opcode - 0x40) / 8;
            let value = match opcode & 0x07 {
                0x0 => cpu.get_b(),
                0x1 => cpu.get_c(),
                0x2 => cpu.get_d(),
                0x3 => cpu.get_e(),
                0x4 => cpu.get_h(),
                0x5 => cpu.get_l(),
                0x6 => bus.read_byte(cpu.get_hl()),
                0x7 => cpu.get_a(),
                _ => unreachable!()
            };
            
            cpu.flags.zero = (value & (1 << bit)) == 0;
            cpu.flags.sub = false;
            cpu.flags.half_carry = true;
            
            if (opcode & 0x07) == 0x06 { 12 } else { 8 }
        },

        // SET/RES operations
        0x80..=0xFF => {
            let is_set = opcode >= 0xC0;
            let bit = ((opcode & 0x7F) - 0x40) / 8;
            let value = match opcode & 0x07 {
                0x0 => cpu.get_b(),
                0x1 => cpu.get_c(),
                0x2 => cpu.get_d(),
                0x3 => cpu.get_e(),
                0x4 => cpu.get_h(),
                0x5 => cpu.get_l(),
                0x6 => bus.read_byte(cpu.get_hl()),
                0x7 => cpu.get_a(),
                _ => unreachable!()
            };
            
            let result = if is_set {
                value | (1 << bit)
            } else {
                value & !(1 << bit)
            };

            match opcode & 0x07 {
                0x0 => (LR35902::set_b as SetRegFn)(cpu, result),
                0x1 => (LR35902::set_c as SetRegFn)(cpu, result),
                0x2 => (LR35902::set_d as SetRegFn)(cpu, result),
                0x3 => (LR35902::set_e as SetRegFn)(cpu, result),
                0x4 => (LR35902::set_h as SetRegFn)(cpu, result),
                0x5 => (LR35902::set_l as SetRegFn)(cpu, result),
                0x6 => bus.write_byte(cpu.get_hl(), result),
                0x7 => (LR35902::set_a as SetRegFn)(cpu, result),
                _ => unreachable!()
            };
            
            if (opcode & 0x07) == 0x06 { 16 } else { 8 }
        },

        unknown_op => panic!("CB-prefixed instruction unimplemented: {:2X}", unknown_op),
    }
}
