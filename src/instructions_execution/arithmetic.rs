use crate::instructions::{ArithmeticTarget, ArithmeticTargetLong};
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
