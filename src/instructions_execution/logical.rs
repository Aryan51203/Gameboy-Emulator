use crate::instructions::ArithmeticTarget;
use crate::memory::MemoryBus;
use crate::registers::Registers;

pub fn and(
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
    let new_value = registers.a & value;
    registers.f.zero = new_value == 0;
    registers.f.subtract = false;
    registers.f.carry = false;
    registers.f.half_carry = false; // CHECK THIS ONCE
    registers.a = new_value;
    pc.wrapping_add(pc_increment)
}

pub fn or(
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
    let new_value = registers.a | value;
    registers.f.zero = new_value == 0;
    registers.f.subtract = false;
    registers.f.carry = false;
    registers.f.half_carry = false; // CHECK THIS ONCE
    registers.a = new_value;
    pc.wrapping_add(pc_increment)
}

pub fn xor(
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
    let new_value = registers.a ^ value;
    registers.f.zero = new_value == 0;
    registers.f.subtract = false;
    registers.f.carry = false;
    registers.f.half_carry = false; // CHECK THIS ONCE
    registers.a = new_value;
    pc.wrapping_add(pc_increment)
}
