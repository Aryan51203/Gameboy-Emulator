use crate::{instructions::RegisterTarget, memory::MemoryBus, registers::Registers};

pub fn sla(registers: &mut Registers, pc: u16, target: RegisterTarget, bus: &mut MemoryBus) -> u16 {
    match target {
        RegisterTarget::A => {
            registers.f.carry = registers.a & 0x7F > 0;
            registers.a = (registers.a << 1) | (registers.a & 0x01);
        }
        RegisterTarget::B => {
            registers.f.carry = registers.b & 0x7F > 0;
            registers.b = (registers.a << 1) | (registers.b & 0x01);
        }
        RegisterTarget::C => {
            registers.f.carry = registers.c & 0x7F > 0;
            registers.c = (registers.c << 1) | (registers.c & 0x01);
        }
        RegisterTarget::D => {
            registers.f.carry = registers.d & 0x7F > 0;
            registers.d = (registers.d << 1) | (registers.d & 0x01);
        }
        RegisterTarget::E => {
            registers.f.carry = registers.e & 0x7F > 0;
            registers.e = (registers.e << 1) | (registers.e & 0x01);
        }
        RegisterTarget::H => {
            registers.f.carry = registers.h & 0x7F > 0;
            registers.h = (registers.h << 1) | (registers.h & 0x01);
        }
        RegisterTarget::L => {
            registers.f.carry = registers.l & 0x7F > 0;
            registers.l = (registers.l << 1) | (registers.l & 0x01);
        }
        RegisterTarget::HLI => {
            registers.f.carry = bus.read_byte(registers.get_hl()) & 0x7F > 0;
            bus.set_byte(
                registers.get_hl(),
                (bus.read_byte(registers.get_hl()) << 1)
                    | (bus.read_byte(registers.get_hl()) & 0x01),
            );
        }
    }
    pc.wrapping_add(2)
}

pub fn sra(registers: &mut Registers, pc: u16, target: RegisterTarget, bus: &mut MemoryBus) -> u16 {
    match target {
        RegisterTarget::A => {
            registers.f.carry = registers.a & 0x01 == 1;
            registers.a = (registers.a >> 1) | (registers.a & 0x80);
        }
        RegisterTarget::B => {
            registers.f.carry = registers.b & 0x01 == 1;
            registers.b = (registers.b >> 1) | (registers.b & 0x80);
        }
        RegisterTarget::C => {
            registers.f.carry = registers.c & 0x01 == 1;
            registers.c = (registers.c >> 1) | (registers.c & 0x80);
        }
        RegisterTarget::D => {
            registers.f.carry = registers.d & 0x01 == 1;
            registers.d = (registers.d >> 1) | (registers.d & 0x80);
        }
        RegisterTarget::E => {
            registers.f.carry = registers.e & 0x01 == 1;
            registers.e = (registers.e >> 1) | (registers.e & 0x80);
        }
        RegisterTarget::H => {
            registers.f.carry = registers.h & 0x01 == 1;
            registers.h = (registers.h >> 1) | (registers.h & 0x80);
        }
        RegisterTarget::L => {
            registers.f.carry = registers.l & 0x01 == 1;
            registers.l = (registers.l >> 1) | (registers.l & 0x80);
        }
        RegisterTarget::HLI => {
            let value = bus.read_byte(registers.get_hl());
            registers.f.carry = value & 0x01 == 1;
            bus.set_byte(registers.get_hl(), (value >> 1) | (value & 0x80));
        }
    }
    pc.wrapping_add(2)
}

pub fn swap(
    registers: &mut Registers,
    pc: u16,
    target: RegisterTarget,
    bus: &mut MemoryBus,
) -> u16 {
    match target {
        RegisterTarget::A => registers.a = (registers.a << 4) | (registers.a >> 4),
        RegisterTarget::B => registers.b = (registers.b << 4) | (registers.b >> 4),
        RegisterTarget::C => registers.c = (registers.c << 4) | (registers.c >> 4),
        RegisterTarget::D => registers.d = (registers.d << 4) | (registers.d >> 4),
        RegisterTarget::E => registers.e = (registers.e << 4) | (registers.e >> 4),
        RegisterTarget::H => registers.h = (registers.h << 4) | (registers.h >> 4),
        RegisterTarget::L => registers.l = (registers.l << 4) | (registers.l >> 4),
        RegisterTarget::HLI => {
            let value = bus.read_byte(registers.get_hl());
            bus.set_byte(registers.get_hl(), (value << 4) | (value >> 4));
        }
    }
    pc.wrapping_add(2)
}

pub fn srl(registers: &mut Registers, pc: u16, target: RegisterTarget, bus: &mut MemoryBus) -> u16 {
    match target {
        RegisterTarget::A => {
            registers.f.carry = registers.a & 0x01 == 1;
            registers.a = (registers.a >> 1) & 0x7F;
        }
        RegisterTarget::B => {
            registers.f.carry = registers.b & 0x01 == 1;
            registers.b = (registers.b >> 1) & 0x7F;
        }
        RegisterTarget::C => {
            registers.f.carry = registers.c & 0x01 == 1;
            registers.c = (registers.c >> 1) & 0x7F;
        }
        RegisterTarget::D => {
            registers.f.carry = registers.d & 0x01 == 1;
            registers.d = (registers.d >> 1) & 0x7F;
        }
        RegisterTarget::E => {
            registers.f.carry = registers.e & 0x01 == 1;
            registers.e = (registers.e >> 1) & 0x7F;
        }
        RegisterTarget::H => {
            registers.f.carry = registers.h & 0x01 == 1;
            registers.h = (registers.h >> 1) & 0x7F;
        }
        RegisterTarget::L => {
            registers.f.carry = registers.l & 0x01 == 1;
            registers.l = (registers.l >> 1) & 0x7F;
        }
        RegisterTarget::HLI => {
            registers.f.carry = bus.read_byte(registers.get_hl()) & 0x01 == 1;
            bus.set_byte(
                registers.get_hl(),
                (bus.read_byte(registers.get_hl())) >> 1 & 0x7F,
            )
        }
    }
    pc.wrapping_add(2)
}
