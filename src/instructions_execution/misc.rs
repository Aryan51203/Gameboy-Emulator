use crate::registers::{self, Registers};

pub fn ccf(registers: &mut Registers, pc: u16) -> u16 {
    registers.f.carry = !registers.f.carry;
    pc.wrapping_add(1)
}

pub fn scf(registers: &mut Registers, pc: u16) -> u16 {
    registers.f.carry = true;
    pc.wrapping_add(1)
}

pub fn daa(registers: &mut Registers, pc: u16) -> u16 {
    pc.wrapping_add(1)
} // TODO: Implement the daa function

pub fn cpl(registers: &mut Registers, pc: u16) -> u16 {
    pc.wrapping_add(1)
} // TODO: Implement the cpl function

pub fn reti(registers: &mut Registers, pc: u16) -> u16 {
    pc.wrapping_add(1)
} // TODO: Implement the reti function

pub fn rst(registers: &mut Registers, pc: u16, value: u8) -> u16 {
    pc.wrapping_add(1)
} // TODO: Implement the rst function

pub fn nop(pc: u16) -> u16 {
    pc.wrapping_add(1)
} // TODO: Implement the nop function

pub fn stop(registers: &mut Registers, pc: u16) -> u16 {
    pc.wrapping_add(1)
} // TODO: Implement the stop function

pub fn halt(registers: &mut Registers, pc: u16) -> u16 {
    pc.wrapping_add(1)
} // TODO: Implement the halt function

pub fn di(registers: &mut Registers, pc: u16) -> u16 {
    pc.wrapping_add(1)
} // TODO: Implement the di function

pub fn ei(registers: &mut Registers, pc: u16) -> u16 {
    pc.wrapping_add(1)
} // TODO: Implement the ei function
