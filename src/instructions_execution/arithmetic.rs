use crate::instructions::{ArithmeticTarget, ArithmeticTargetLong, IncDecTarget};
use crate::memory::MemoryBus;
use crate::registers::Registers;

pub fn add(
    registers: &mut Registers,
    target: ArithmeticTarget,
    bus: &mut MemoryBus,
    pc: u16,
) -> u16 {
    let (value, pc_increment) = match target {
        ArithmeticTarget::B => (registers.b, 1),
        ArithmeticTarget::C => (registers.c, 1),
        ArithmeticTarget::D => (registers.d, 1),
        ArithmeticTarget::E => (registers.e, 1),
        ArithmeticTarget::H => (registers.h, 1),
        ArithmeticTarget::L => (registers.l, 1),
        ArithmeticTarget::HLI => (bus.read_byte(registers.get_hl()), 1),
        ArithmeticTarget::A => (registers.a, 1),
        ArithmeticTarget::D8 => (bus.read_byte(pc + 1), 2),
    };

    let (new_value, did_overflow) = registers.a.overflowing_add(value);
    registers.f.zero = new_value == 0;
    registers.f.subtract = false;
    registers.f.carry = did_overflow;
    registers.f.half_carry = (registers.a & 0xF) + (value & 0xF) > 0xF;
    registers.a = new_value;
    pc.wrapping_add(pc_increment)
}

pub fn add_long(
    registers: &mut Registers,
    target: ArithmeticTargetLong,
    bus: &mut MemoryBus,
    pc: u16,
    sp: u16,
) -> u16 {
    let (value, pc_increment, is_sp) = match target {
        ArithmeticTargetLong::BC => (registers.get_bc(), 1, false),
        ArithmeticTargetLong::DE => (registers.get_de(), 1, false),
        ArithmeticTargetLong::HL => (registers.get_hl(), 1, false),
        ArithmeticTargetLong::SP => (sp, 1, true),
        ArithmeticTargetLong::S8 => (bus.read_byte(pc + 1) as u16, 2, false),
    };

    let operated_register_value: u16 = if is_sp { sp } else { registers.get_hl() };

    let (new_value, did_overflow) = operated_register_value.overflowing_add(value);
    registers.f.subtract = false;
    registers.f.carry = did_overflow;
    registers.f.half_carry = (operated_register_value & 0xFFF) + (value & 0xFFF) > 0xFFF;

    registers.set_hl(new_value);

    pc.wrapping_add(pc_increment)
}

pub fn add_carry(
    registers: &mut Registers,
    target: ArithmeticTarget,
    bus: &mut MemoryBus,
    pc: u16,
) -> u16 {
    let (value, pc_increment) = match target {
        ArithmeticTarget::B => (registers.b, 1),
        ArithmeticTarget::C => (registers.c, 1),
        ArithmeticTarget::D => (registers.d, 1),
        ArithmeticTarget::E => (registers.e, 1),
        ArithmeticTarget::H => (registers.h, 1),
        ArithmeticTarget::L => (registers.l, 1),
        ArithmeticTarget::HLI => (bus.read_byte(registers.get_hl()), 1),
        ArithmeticTarget::A => (registers.a, 1),
        ArithmeticTarget::D8 => (bus.read_byte(pc + 1), 2),
    };

    let carry = if registers.f.carry { 1 } else { 0 };
    let (new_value, did_overflow) = registers.a.overflowing_add(value + carry);
    registers.f.zero = new_value == 0;
    registers.f.subtract = false;
    registers.f.carry = did_overflow;
    registers.f.half_carry = (registers.a & 0xF) + (value & 0xF) + carry > 0xF;

    registers.a = new_value;
    pc.wrapping_add(pc_increment)
}

pub fn sub(
    registers: &mut Registers,
    target: ArithmeticTarget,
    bus: &mut MemoryBus,
    pc: u16,
) -> u16 {
    let (value, pc_increment) = match target {
        ArithmeticTarget::B => (registers.b, 1),
        ArithmeticTarget::C => (registers.c, 1),
        ArithmeticTarget::D => (registers.d, 1),
        ArithmeticTarget::E => (registers.e, 1),
        ArithmeticTarget::H => (registers.h, 1),
        ArithmeticTarget::L => (registers.l, 1),
        ArithmeticTarget::HLI => (bus.read_byte(registers.get_hl()), 1),
        ArithmeticTarget::A => (registers.a, 1),
        ArithmeticTarget::D8 => (bus.read_byte(pc + 1), 2),
    };

    let (new_value, did_overflow) = registers.a.overflowing_sub(value);
    registers.f.zero = new_value == 0;
    registers.f.subtract = true;
    registers.f.carry = did_overflow;
    registers.f.half_carry = (registers.a & 0xF) < (value & 0xF);

    registers.a = new_value;
    pc.wrapping_add(pc_increment)
}

pub fn sub_carry(
    registers: &mut Registers,
    target: ArithmeticTarget,
    bus: &mut MemoryBus,
    pc: u16,
) -> u16 {
    let (value, pc_increment) = match target {
        ArithmeticTarget::B => (registers.b, 1),
        ArithmeticTarget::C => (registers.c, 1),
        ArithmeticTarget::D => (registers.d, 1),
        ArithmeticTarget::E => (registers.e, 1),
        ArithmeticTarget::H => (registers.h, 1),
        ArithmeticTarget::L => (registers.l, 1),
        ArithmeticTarget::HLI => (bus.read_byte(registers.get_hl()), 1),
        ArithmeticTarget::A => (registers.a, 1),
        ArithmeticTarget::D8 => (bus.read_byte(pc + 1), 2),
    };

    let carry = if registers.f.carry { 1 } else { 0 };
    let (new_value, did_overflow) = registers.a.overflowing_sub(value + carry);

    registers.f.zero = new_value == 0;
    registers.f.subtract = true;
    registers.f.carry = did_overflow;
    registers.f.half_carry = (registers.a & 0xF) < (value & 0xF) + carry;

    registers.a = new_value;
    pc.wrapping_add(pc_increment)
}

pub fn exec_inc_dec(
    registers: &mut Registers,
    target: IncDecTarget,
    bus: &mut MemoryBus,
    pc: u16,
    sp: &mut u16,
    is_inc: bool,
) -> u16 {
    match target {
        IncDecTarget::B => {
            let value = registers.b;
            let new_value = inc_dec(value, is_inc, registers);
            registers.b = new_value;
            pc.wrapping_add(1)
        }
        IncDecTarget::C => {
            let value = registers.c;
            let new_value = inc_dec(value, is_inc, registers);
            registers.c = new_value;
            pc.wrapping_add(1)
        }
        IncDecTarget::D => {
            let value = registers.d;
            let new_value = inc_dec(value, is_inc, registers);
            registers.d = new_value;
            pc.wrapping_add(1)
        }
        IncDecTarget::E => {
            let value = registers.e;
            let new_value = inc_dec(value, is_inc, registers);
            registers.e = new_value;
            pc.wrapping_add(1)
        }
        IncDecTarget::H => {
            let value = registers.h;
            let new_value = inc_dec(value, is_inc, registers);
            registers.h = new_value;
            pc.wrapping_add(1)
        }
        IncDecTarget::L => {
            let value = registers.l;
            let new_value = inc_dec(value, is_inc, registers);
            registers.l = new_value;
            pc.wrapping_add(1)
        }
        IncDecTarget::HLI => {
            let value = bus.read_byte(registers.get_hl());
            let new_value = inc_dec(value, is_inc, registers);
            bus.set_byte(registers.get_hl(), new_value);
            pc.wrapping_add(1)
        }
        IncDecTarget::A => {
            let value = registers.a;
            let new_value = inc_dec(value, is_inc, registers);
            registers.a = new_value;
            pc.wrapping_add(1)
        }
        IncDecTarget::BC => {
            let value = registers.get_bc();
            let new_value = inc_dec_long(value, is_inc, registers);
            registers.set_bc(new_value);
            pc.wrapping_add(1)
        }
        IncDecTarget::DE => {
            let value = registers.get_de();
            let new_value = inc_dec_long(value, is_inc, registers);
            registers.set_de(new_value);
            pc.wrapping_add(1)
        }
        IncDecTarget::HL => {
            let value = registers.get_hl();
            let new_value = inc_dec_long(value, is_inc, registers);
            registers.set_hl(new_value);
            pc.wrapping_add(1)
        }
        IncDecTarget::SP => {
            let value = *sp;
            let new_value = inc_dec_long(value, is_inc, registers);
            *sp = new_value;
            pc.wrapping_add(1)
        }
    }
}

fn inc_dec(value: u8, is_inc: bool, registers: &mut Registers) -> u8 {
    let (new_value, did_overflow) = if is_inc {
        value.overflowing_add(1)
    } else {
        value.overflowing_sub(1)
    };
    registers.f.zero = new_value == 0;
    registers.f.subtract = !is_inc;
    registers.f.carry = did_overflow;

    if is_inc {
        registers.f.half_carry = (value & 0xF) + 1 > 0xF;
    } else {
        registers.f.half_carry = (value & 0xF) < 1;
    }
    new_value
}

fn inc_dec_long(value: u16, is_inc: bool, registers: &mut Registers) -> u16 {
    let (new_value, did_overflow) = if is_inc {
        value.overflowing_add(1)
    } else {
        value.overflowing_sub(1)
    };
    registers.f.subtract = !is_inc;
    registers.f.carry = did_overflow;

    if is_inc {
        registers.f.half_carry = (value & 0xFFF) + 1 > 0xFFF;
    } else {
        registers.f.half_carry = (value & 0xFFF) < 1;
    }
    new_value
}
