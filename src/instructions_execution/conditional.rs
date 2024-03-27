use crate::{
    instructions::{ArithmeticTarget, JumpType},
    memory::MemoryBus,
    registers::Registers,
};

pub fn cmp(
    registers: &mut Registers,
    target: ArithmeticTarget,
    pc: u16,
    bus: &mut MemoryBus,
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

    let (new_value, _overflow) = registers.a.overflowing_sub(value);
    registers.f.zero = new_value == 0;
    pc.wrapping_add(pc_increment)
}

pub fn jump(registers: &mut Registers, pc: u16, bus: &MemoryBus, test: JumpType) -> u16 {
    let jump_condition = match test {
        JumpType::NotZero => !registers.f.zero,
        JumpType::Zero => registers.f.zero,
        JumpType::NotCarry => !registers.f.carry,
        JumpType::Carry => registers.f.carry,
        JumpType::Always => true,
    };

    if jump_condition {
        let least_significant_byte = bus.read_byte(pc + 1) as u16;
        let most_significant_byte = bus.read_byte(pc + 2) as u16;
        (most_significant_byte << 8) | least_significant_byte
    } else {
        pc.wrapping_add(3)
    }
}

pub fn jpl(registers: Registers) -> u16 {
    registers.get_hl()
}

pub fn jump_relative(registers: &mut Registers, pc: u16, bus: &MemoryBus, test: JumpType) -> u16 {
    let jump_condition = match test {
        JumpType::NotZero => !registers.f.zero,
        JumpType::Zero => registers.f.zero,
        JumpType::NotCarry => !registers.f.carry,
        JumpType::Carry => registers.f.carry,
        JumpType::Always => true,
    };

    if jump_condition {
        let offset = bus.read_byte(pc + 1);
        pc.wrapping_add(offset as u16)
    } else {
        pc.wrapping_add(2)
    }
}

pub fn call(
    registers: &mut Registers,
    pc: u16,
    bus: &mut MemoryBus,
    test: JumpType,
    sp: &mut u16,
) -> u16 {
    let jump_condition = match test {
        JumpType::NotZero => !registers.f.zero,
        JumpType::Zero => registers.f.zero,
        JumpType::NotCarry => !registers.f.carry,
        JumpType::Carry => registers.f.carry,
        JumpType::Always => true,
    };

    if jump_condition {
        let least_significant_byte = bus.read_byte(pc + 1) as u16;
        let most_significant_byte = bus.read_byte(pc + 2) as u16;
        *sp = (*sp).wrapping_sub(1);
        bus.set_byte(*sp, ((pc.wrapping_add(3) & 0xFF00) >> 8) as u8);
        *sp = (*sp).wrapping_sub(1);
        bus.set_byte(*sp, (pc.wrapping_add(3) & 0x00FF) as u8);
        (most_significant_byte << 8) | least_significant_byte
    } else {
        pc.wrapping_add(3)
    }
}

pub fn ret(
    registers: &mut Registers,
    pc: u16,
    bus: &mut MemoryBus,
    test: JumpType,
    sp: &mut u16,
) -> u16 {
    let jump_condition = match test {
        JumpType::NotZero => !registers.f.zero,
        JumpType::Zero => registers.f.zero,
        JumpType::NotCarry => !registers.f.carry,
        JumpType::Carry => registers.f.carry,
        JumpType::Always => true,
    };
    if jump_condition {
        let lsb = bus.read_byte(*sp) as u16;
        *sp = (*sp).wrapping_add(1);
        let msb = bus.read_byte(*sp) as u16;
        *sp = (*sp).wrapping_add(1);
        (msb << 8) | lsb
    } else {
        pc.wrapping_add(1)
    }
}
