use crate::{instructions::StackRegisters, memory::MemoryBus, registers::Registers};

pub fn push(
    registers: &mut Registers,
    target: StackRegisters,
    bus: &mut MemoryBus,
    pc: u16,
) -> u16 {
    pc
}

pub fn pop(registers: &mut Registers, target: StackRegisters, bus: &mut MemoryBus, pc: u16) -> u16 {
    pc
}

// TODO: Implement the push and pop functions
