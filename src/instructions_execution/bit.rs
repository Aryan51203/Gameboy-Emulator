use crate::{instructions::RegisterTarget, memory::MemoryBus, registers::Registers};

pub fn bit(
    registers: &mut Registers,
    target: RegisterTarget,
    bus: &mut MemoryBus,
    pc: u16,
    bit: u8,
) -> u16 {
    let value = match target {
        RegisterTarget::A => registers.a,
        RegisterTarget::B => registers.b,
        RegisterTarget::C => registers.c,
        RegisterTarget::D => registers.d,
        RegisterTarget::E => registers.e,
        RegisterTarget::H => registers.h,
        RegisterTarget::L => registers.l,
        RegisterTarget::HLI => bus.read_byte(registers.get_hl()),
    };
    registers.f.zero = value & 0x01 << bit == 0;
    pc.wrapping_add(2)
}

pub fn set(
    registers: &mut Registers,
    target: RegisterTarget,
    bus: &mut MemoryBus,
    pc: u16,
    bit: u8,
) -> u16 {
    match target {
        RegisterTarget::A => registers.a = registers.a | 0x01 << bit,
        RegisterTarget::B => registers.b = registers.b | 0x01 << bit,
        RegisterTarget::C => registers.c = registers.c | 0x01 << bit,
        RegisterTarget::D => registers.d = registers.d | 0x01 << bit,
        RegisterTarget::E => registers.e = registers.e | 0x01 << bit,
        RegisterTarget::H => registers.h = registers.h | 0x01 << bit,
        RegisterTarget::L => registers.l = registers.l | 0x01 << bit,
        RegisterTarget::HLI => bus.set_byte(
            registers.get_hl(),
            bus.read_byte(registers.get_hl()) | 0x01 << bit,
        ),
    }
    pc.wrapping_add(2)
}

pub fn res(
    registers: &mut Registers,
    target: RegisterTarget,
    bus: &mut MemoryBus,
    pc: u16,
    bit: u8,
) -> u16 {
    let value: u8 = 0xFF ^ (0x01 << bit);
    match target {
        RegisterTarget::A => registers.a = registers.a & value,
        RegisterTarget::B => registers.b = registers.b & value,
        RegisterTarget::C => registers.c = registers.c & value,
        RegisterTarget::D => registers.d = registers.d & value,
        RegisterTarget::E => registers.e = registers.e & value,
        RegisterTarget::H => registers.h = registers.h & value,
        RegisterTarget::L => registers.l = registers.l & value,
        RegisterTarget::HLI => bus.set_byte(
            registers.get_hl(),
            bus.read_byte(registers.get_hl()) & value,
        ),
    }
    pc.wrapping_add(2)
}
