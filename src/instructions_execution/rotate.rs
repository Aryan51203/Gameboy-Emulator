use crate::{instructions::RegisterTarget, memory::MemoryBus, registers::Registers};

pub fn rlca(registers: &mut Registers, pc: u16) -> u16 {
    registers.f.carry = registers.a & 0x80 > 1;
    registers.a = (registers.a << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
    pc.wrapping_add(1)
}

pub fn rrca(registers: &mut Registers, pc: u16) -> u16 {
    registers.f.carry = registers.a & 0x01 == 1;
    registers.a = (registers.a >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
    pc.wrapping_add(1)
}

pub fn rla(registers: &mut Registers, pc: u16) -> u16 {
    registers.a = (registers.a << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
    pc.wrapping_add(1)
}

pub fn rra(registers: &mut Registers, pc: u16) -> u16 {
    registers.a = (registers.a >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
    pc.wrapping_add(1)
}

// 16-bit instructions

pub fn rlc(registers: &mut Registers, pc: u16, bus: &mut MemoryBus, target: RegisterTarget) -> u16 {
    match target {
        RegisterTarget::A => {
            registers.f.carry = registers.a & 0x80 > 1;
            registers.a = (registers.a << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::B => {
            registers.f.carry = registers.b & 0x80 > 1;
            registers.b = (registers.b << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::C => {
            registers.f.carry = registers.c & 0x80 > 1;
            registers.c = (registers.c << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::D => {
            registers.f.carry = registers.d & 0x80 > 1;
            registers.d = (registers.d << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::E => {
            registers.f.carry = registers.e & 0x80 > 1;
            registers.e = (registers.e << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::H => {
            registers.f.carry = registers.h & 0x80 > 1;
            registers.h = (registers.h << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::L => {
            registers.f.carry = registers.l & 0x80 > 1;
            registers.l = (registers.l << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::HLI => {
            registers.f.carry = bus.read_byte(registers.get_hl()) & 0x80 > 1;
            bus.set_byte(
                registers.get_hl(),
                (bus.read_byte(registers.get_hl()) << 1)
                    | (if registers.f.carry { 0x01 } else { 0x00 }),
            );
        }
    }
    pc.wrapping_add(2)
}

pub fn rrc(registers: &mut Registers, pc: u16, bus: &mut MemoryBus, target: RegisterTarget) -> u16 {
    match target {
        RegisterTarget::A => {
            registers.f.carry = registers.a & 0x01 == 1;
            registers.a = (registers.a >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::B => {
            registers.f.carry = registers.b & 0x01 == 1;
            registers.b = (registers.b >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::C => {
            registers.f.carry = registers.c & 0x01 == 1;
            registers.c = (registers.c >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::D => {
            registers.f.carry = registers.d & 0x01 == 1;
            registers.d = (registers.d >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::E => {
            registers.f.carry = registers.e & 0x01 == 1;
            registers.e = (registers.e >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::H => {
            registers.f.carry = registers.h & 0x01 == 1;
            registers.h = (registers.h >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::L => {
            registers.f.carry = registers.l & 0x01 == 1;
            registers.l = (registers.l >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::HLI => {
            registers.f.carry = bus.read_byte(registers.get_hl()) & 0x01 == 1;
            bus.set_byte(
                registers.get_hl(),
                (bus.read_byte(registers.get_hl()) >> 1)
                    | (if registers.f.carry { 0x80 } else { 0x00 }),
            );
        }
    }
    pc.wrapping_add(2)
}

pub fn rl(registers: &mut Registers, pc: u16, bus: &mut MemoryBus, target: RegisterTarget) -> u16 {
    match target {
        RegisterTarget::A => {
            registers.a = (registers.a << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::B => {
            registers.b = (registers.b << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::C => {
            registers.c = (registers.c << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::D => {
            registers.d = (registers.d << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::E => {
            registers.e = (registers.e << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::H => {
            registers.h = (registers.h << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::L => {
            registers.l = (registers.l << 1) | (if registers.f.carry { 0x01 } else { 0x00 });
        }
        RegisterTarget::HLI => {
            bus.set_byte(
                registers.get_hl(),
                (bus.read_byte(registers.get_hl()) << 1)
                    | (if registers.f.carry { 0x01 } else { 0x00 }),
            );
        }
    }
    pc.wrapping_add(2)
}

pub fn rr(registers: &mut Registers, pc: u16, bus: &mut MemoryBus, target: RegisterTarget) -> u16 {
    match target {
        RegisterTarget::A => {
            registers.a = (registers.a >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::B => {
            registers.b = (registers.b >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::C => {
            registers.c = (registers.c >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::D => {
            registers.d = (registers.d >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::E => {
            registers.e = (registers.e >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::H => {
            registers.h = (registers.h >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::L => {
            registers.l = (registers.l >> 1) | (if registers.f.carry { 0x80 } else { 0x00 });
        }
        RegisterTarget::HLI => {
            bus.set_byte(
                registers.get_hl(),
                (bus.read_byte(registers.get_hl()) >> 1)
                    | (if registers.f.carry { 0x80 } else { 0x00 }),
            );
        }
    }
    pc.wrapping_add(2)
}
