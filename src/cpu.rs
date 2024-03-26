use crate::instructions::{
    ArithmeticTarget, Instruction, JumpType, RegisterTarget, StackRegisters,
};
use crate::memory::MemoryBus;
use crate::registers::Registers;

use crate::instructions_execution::{arithmetic, load, logical, rotate, shift};

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
                let jump_condition = match test {
                    JumpType::NotZero => !self.registers.f.zero,
                    JumpType::Zero => self.registers.f.zero,
                    JumpType::NotCarry => !self.registers.f.carry,
                    JumpType::Carry => self.registers.f.carry,
                    JumpType::Always => true,
                };
                self.jump(jump_condition)
            }

            Instruction::JPL => self.registers.get_hl(),

            Instruction::JR(test) => {
                let jump_condition = match test {
                    JumpType::NotZero => !self.registers.f.zero,
                    JumpType::Zero => self.registers.f.zero,
                    JumpType::NotCarry => !self.registers.f.carry,
                    JumpType::Carry => self.registers.f.carry,
                    JumpType::Always => true,
                };
                self.jump_relative(jump_condition)
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
            Instruction::CMP(target) => match target {
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let (new_value, _overflow) = self.registers.a.overflowing_sub(value);
                    self.registers.f.zero = new_value == 0;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let (new_value, _overflow) = self.registers.a.overflowing_sub(value);
                    self.registers.f.zero = new_value == 0;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let (new_value, _overflow) = self.registers.a.overflowing_sub(value);
                    self.registers.f.zero = new_value == 0;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let (new_value, _overflow) = self.registers.a.overflowing_sub(value);
                    self.registers.f.zero = new_value == 0;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let (new_value, _overflow) = self.registers.a.overflowing_sub(value);
                    self.registers.f.zero = new_value == 0;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let (new_value, _overflow) = self.registers.a.overflowing_sub(value);
                    self.registers.f.zero = new_value == 0;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HLI => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let (new_value, _overflow) = self.registers.a.overflowing_sub(value);
                    self.registers.f.zero = new_value == 0;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let (new_value, _overflow) = self.registers.a.overflowing_sub(value);
                    self.registers.f.zero = new_value == 0;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D8 => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let (new_value, _overflow) = self.registers.a.overflowing_sub(value);
                    self.registers.f.zero = new_value == 0;
                    self.pc.wrapping_add(2)
                }
            },

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

            // RLCA instruction
            Instruction::RLCA => rotate::rlca(&mut self.registers, self.pc),

            // RRCA instruction
            Instruction::RRCA => rotate::rrca(&mut self.registers, self.pc),

            // RLA instruction
            Instruction::RLA => rotate::rla(&mut self.registers, self.pc),

            // RRA instruction
            Instruction::RRA => rotate::rra(&mut self.registers, self.pc),

            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpType::NotZero => !self.registers.f.zero,
                    JumpType::Zero => self.registers.f.zero,
                    JumpType::NotCarry => !self.registers.f.carry,
                    JumpType::Carry => self.registers.f.carry,
                    JumpType::Always => true,
                };
                self.call(jump_condition)
            }

            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpType::NotZero => !self.registers.f.zero,
                    JumpType::Zero => self.registers.f.zero,
                    JumpType::NotCarry => !self.registers.f.carry,
                    JumpType::Carry => self.registers.f.carry,
                    JumpType::Always => true,
                };
                self.ret(jump_condition)
            }

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

            /* Load instructions */
            Instruction::LD(ld_type) => load::load(
                &mut self.registers,
                &mut self.bus,
                self.pc,
                ld_type,
                &mut self.sp,
            ),

            /* Stack instructions */
            Instruction::POP(target) => {
                match target {
                    StackRegisters::AF => {}
                    StackRegisters::BC => {}
                    StackRegisters::DE => {}
                    StackRegisters::HL => {}
                }
                self.pc.wrapping_add(1)
            }

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
                match target {
                    RegisterTarget::A => {
                        self.registers.f.zero = self.registers.a & 0x01 << bit == 0
                    }
                    RegisterTarget::B => {
                        self.registers.f.zero = self.registers.b & 0x01 << bit == 0
                    }
                    RegisterTarget::C => {
                        self.registers.f.zero = self.registers.c & 0x01 << bit == 0
                    }
                    RegisterTarget::D => {
                        self.registers.f.zero = self.registers.d & 0x01 << bit == 0
                    }
                    RegisterTarget::E => {
                        self.registers.f.zero = self.registers.e & 0x01 << bit == 0
                    }
                    RegisterTarget::H => {
                        self.registers.f.zero = self.registers.h & 0x01 << bit == 0
                    }
                    RegisterTarget::L => {
                        self.registers.f.zero = self.registers.l & 0x01 << bit == 0
                    }
                    RegisterTarget::HLI => {
                        self.registers.f.zero =
                            self.bus.read_byte(self.registers.get_hl()) & 0x01 << bit == 0
                    }
                }
                self.pc.wrapping_add(2)
            }

            /* Reset bit 'bit' in register to 0. */
            Instruction::RES(bit, target) => {
                let value: u8 = 0xFF ^ (0x01 << bit);
                match target {
                    RegisterTarget::A => self.registers.b = self.registers.a & value,
                    RegisterTarget::B => self.registers.b = self.registers.b & value,
                    RegisterTarget::C => self.registers.b = self.registers.c & value,
                    RegisterTarget::D => self.registers.b = self.registers.d & value,
                    RegisterTarget::E => self.registers.b = self.registers.e & value,
                    RegisterTarget::H => self.registers.b = self.registers.h & value,
                    RegisterTarget::L => self.registers.b = self.registers.l & value,
                    RegisterTarget::HLI => self.bus.set_byte(
                        self.registers.get_hl(),
                        self.bus.read_byte(self.registers.get_hl()) & value,
                    ),
                }
                self.pc.wrapping_add(2)
            }

            /* Set bit 'bit; in register to 1 */
            Instruction::SET(bit, target) => {
                match target {
                    RegisterTarget::A => self.registers.b = self.registers.a | 0x01 << bit,
                    RegisterTarget::B => self.registers.b = self.registers.b | 0x01 << bit,
                    RegisterTarget::C => self.registers.b = self.registers.c | 0x01 << bit,
                    RegisterTarget::D => self.registers.b = self.registers.d | 0x01 << bit,
                    RegisterTarget::E => self.registers.b = self.registers.e | 0x01 << bit,
                    RegisterTarget::H => self.registers.b = self.registers.h | 0x01 << bit,
                    RegisterTarget::L => self.registers.b = self.registers.l | 0x01 << bit,
                    RegisterTarget::HLI => self.bus.set_byte(
                        self.registers.get_hl(),
                        self.bus.read_byte(self.registers.get_hl()) | 0x01 << bit,
                    ),
                }
                self.pc.wrapping_add(2)
            }
            _ => {
                /* Support more instructions */
                self.pc
            }
        }
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn jump_relative(&self, should_jump: bool) -> u16 {
        if should_jump {
            let offset = self.bus.read_byte(self.pc + 1);
            self.pc.wrapping_add(offset as u16)
        } else {
            self.pc.wrapping_add(2)
        }
    }

    // Call and Return
    fn call(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            self.push(self.pc.wrapping_add(3));
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn ret(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    // Stack functions
    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.set_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.set_byte(self.sp, (value & 0x00FF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (msb << 8) | lsb
    }
}
