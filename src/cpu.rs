use crate::instructions::Instruction;
use crate::memory::MemoryBus;
use crate::registers::Registers;

use crate::instructions_execution::{arithmetic, bit, conditional, load, logical, rotate, shift};

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}:{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unknown instruction found for: {}", description);
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            // JUMP Instructions
            Instruction::JP(test) => {
                conditional::jump(&mut self.registers, self.pc, &mut self.bus, test)
            }

            Instruction::JPL => conditional::jpl(self.registers),

            Instruction::JR(test) => {
                conditional::jump_relative(&mut self.registers, self.pc, &mut self.bus, test)
            }

            // addition instructions
            Instruction::ADD(target) => {
                arithmetic::add(&mut self.registers, target, &mut self.bus, self.pc)
            }

            // long addition instructions
            Instruction::ADDL(target) => {
                arithmetic::add_long(&mut self.registers, target, &mut self.bus, self.pc, self.sp)
            }

            // addition with carry instructions
            Instruction::ADC(target) => {
                arithmetic::add_carry(&mut self.registers, target, &mut self.bus, self.pc)
            }

            // Subtraction instructions
            Instruction::SUB(target) => {
                arithmetic::sub(&mut self.registers, target, &mut self.bus, self.pc)
            }

            // Subtraction with carry instructions
            Instruction::SBC(target) => {
                arithmetic::sub_carry(&mut self.registers, target, &mut self.bus, self.pc)
            }

            // Increment instructions
            Instruction::INC(target) => arithmetic::exec_inc_dec(
                &mut self.registers,
                target,
                &mut self.bus,
                self.pc,
                &mut self.sp,
                true,
            ),

            // Decrement Instructions
            Instruction::DEC(target) => arithmetic::exec_inc_dec(
                &mut self.registers,
                target,
                &mut self.bus,
                self.pc,
                &mut self.sp,
                false,
            ),

            // Logical AND instructions
            Instruction::AND(target) => {
                logical::and(&mut self.registers, target, &mut self.bus, self.pc)
            }

            // Logical OR instructions
            Instruction::OR(target) => {
                logical::or(&mut self.registers, target, &mut self.bus, self.pc)
            }

            // Logical XOR instructions
            Instruction::XOR(target) => {
                logical::xor(&mut self.registers, target, &mut self.bus, self.pc)
            }

            // Compare instructions
            Instruction::CMP(target) => {
                conditional::cmp(&mut self.registers, target, self.pc, &mut self.bus)
            }

            // RLCA instruction
            Instruction::RLCA => rotate::rlca(&mut self.registers, self.pc),

            // RRCA instruction
            Instruction::RRCA => rotate::rrca(&mut self.registers, self.pc),

            // RLA instruction
            Instruction::RLA => rotate::rla(&mut self.registers, self.pc),

            // RRA instruction
            Instruction::RRA => rotate::rra(&mut self.registers, self.pc),

            /* Load instructions */
            Instruction::LD(ld_type) => load::load(
                &mut self.registers,
                &mut self.bus,
                self.pc,
                ld_type,
                &mut self.sp,
            ),

            /* Prefixed Instructions (16-bit instructions) */

            /* Rotate the contents of register to the left. That is, the contents of bit 0 are copied to bit 1, and the previous contents of bit 1 (before the copy operation) are copied to bit 2. The same operation is repeated in sequence for the rest of the register. The contents of bit 7 are placed in both the CY flag and bit 0 of register. */
            Instruction::RLC(target) => {
                rotate::rlc(&mut self.registers, self.pc, &mut self.bus, target)
            }

            /* Rotate the contents of register to the right. That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy operation) are copied to bit 5. The same operation is repeated in sequence for the rest of the register. The contents of bit 0 are placed in both the CY flag and bit 7 of register. */
            Instruction::RRC(target) => {
                rotate::rrc(&mut self.registers, self.pc, &mut self.bus, target)
            }

            /* Rotate the contents of register to the left. That is, the contents of bit 0 are copied to bit 1, and the previous contents of bit 1 (before the copy operation) are copied to bit 2. The same operation is repeated in sequence for the rest of the register. The previous contents of the carry (CY) flag are copied to bit 0 of register. */
            Instruction::RL(target) => {
                rotate::rl(&mut self.registers, self.pc, &mut self.bus, target)
            }

            /* Rotate the contents of register to the right. That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy operation) are copied to bit 5. The same operation is repeated in sequence for the rest of the register. The previous contents of the carry (CY) flag are copied to bit 7 of register. */
            Instruction::RR(target) => {
                rotate::rr(&mut self.registers, self.pc, &mut self.bus, target)
            }

            /* Shift the contents of register to the left. That is, the contents of bit 0 are copied to bit 1, and the previous contents of bit 1 (before the copy operation) are copied to bit 2. The same operation is repeated in sequence for the rest of the register. The contents of bit 7 are copied to the CY flag, and bit 0 of register is reset to 0. */
            Instruction::SLA(target) => {
                shift::sla(&mut self.registers, self.pc, target, &mut self.bus)
            }

            /* Shift the contents of register to the right. That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy operation) are copied to bit 5. The same operation is repeated in sequence for the rest of the register. The contents of bit 0 are copied to the CY flag, and bit 7 of register is unchanged. */
            Instruction::SRA(target) => {
                shift::sra(&mut self.registers, self.pc, target, &mut self.bus)
            }

            /* Shift the contents of the lower-order four bits (0-3) of register to the higher-order four bits (4-7) of the register, and shift the higher-order four bits to the lower-order four bits. */
            Instruction::SWAP(target) => {
                shift::swap(&mut self.registers, self.pc, target, &mut self.bus)
            }

            /* Shift the contents of register to the right. That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy operation) are copied to bit 5. The same operation is repeated in sequence for the rest of the register. The contents of bit 0 are copied to the CY flag, and bit 7 of register is reset to 0. */
            Instruction::SRL(target) => {
                shift::srl(&mut self.registers, self.pc, target, &mut self.bus)
            }

            /* Copy the complement of the contents of bit 'bit' in register to the Z flag of the program status word (PSW). */
            Instruction::BIT(bit, target) => {
                bit::bit(&mut self.registers, target, &mut self.bus, self.pc, bit)
            }

            /* Reset bit 'bit' in register to 0. */
            Instruction::RES(bit, target) => {
                bit::res(&mut self.registers, target, &mut self.bus, self.pc, bit)
            }

            /* Set bit 'bit; in register to 1 */
            Instruction::SET(bit, target) => {
                bit::set(&mut self.registers, target, &mut self.bus, self.pc, bit)
            }

            /* Stack instructions */
            Instruction::PUSH(target) => self.pc, // TODO: Implement Push

            Instruction::POP(target) => self.pc.wrapping_add(1), // TODO: Implement Pop

            Instruction::CCF => {
                self.registers.f.carry = !self.registers.f.carry;
                self.pc.wrapping_add(1)
            }

            Instruction::SCF => {
                self.registers.f.carry = true;
                self.pc.wrapping_add(1)
            }

            Instruction::DAA => {
                // TO BE IMPLEMENTED
                self.pc.wrapping_add(1)
            }

            Instruction::CPL => {
                self.registers.a = !self.registers.a; // TODO: Check if this is correct and flags will be set correctly
                self.pc.wrapping_add(1)
            }

            Instruction::CALL(test) => conditional::call(
                &mut self.registers,
                self.pc,
                &mut self.bus,
                test,
                &mut self.sp,
            ),

            Instruction::RET(test) => conditional::ret(
                &mut self.registers,
                self.pc,
                &mut self.bus,
                test,
                &mut self.sp,
            ),

            Instruction::RETI => {
                // TODO: Implement reti
                self.pc.wrapping_add(1)
            }

            Instruction::RST(value) => {
                // TODO: Implement rst
                self.pc
            }

            // Miscellanoes instructions
            Instruction::NOP => self.pc.wrapping_add(1),

            Instruction::STOP => {
                // TODO: Implement stop
                self.pc
            }

            Instruction::HALT => {
                // TODO: Implement halt
                self.pc
            }

            Instruction::DI => {
                // TODO: Implement di
                self.pc
            }

            Instruction::EI => {
                // TODO: Implement ei
                self.pc
            }
        }
    }
}
