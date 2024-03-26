use crate::instructions::{
    ArithmeticTarget, ArithmeticTargetLong, ByteAddress, IncDecTarget, Indirect, Instruction,
    JumpType, LoadByteSource, LoadByteTarget, LoadType, LoadWordSource, LoadWordTarget,
    RegisterTarget, StackRegisters,
};
use crate::memory::MemoryBus;
use crate::registers::Registers;

use crate::instructions_execution::arithmetic;

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

            // Logical AND instructions
            Instruction::AND(target) => match target {
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HLI => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D8 => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
            },

            // Logical OR instructions
            Instruction::OR(target) => match target {
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HLI => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D8 => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
            },

            // Logical XOR instructions
            Instruction::XOR(target) => match target {
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HLI => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D8 => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
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
            Instruction::RLCA => {
                self.registers.f.carry = self.registers.a & 0x80 > 1;
                self.registers.a =
                    (self.registers.a << 1) | (if self.registers.f.carry { 0x01 } else { 0x00 });
                self.pc.wrapping_add(1)
            }

            // RRCA instruction
            Instruction::RRCA => {
                self.registers.f.carry = self.registers.a & 0x01 == 1;
                self.registers.a =
                    (self.registers.a >> 1) | (if self.registers.f.carry { 0x80 } else { 0x00 });
                self.pc.wrapping_add(1)
            }

            // RLA instruction
            Instruction::RLA => {
                self.registers.a =
                    (self.registers.a << 1) | (if self.registers.f.carry { 0x01 } else { 0x00 });
                self.pc.wrapping_add(1)
            }

            // RRA instruction
            Instruction::RRA => {
                self.registers.a =
                    (self.registers.a >> 1) | (if self.registers.f.carry { 0x80 } else { 0x00 });
                self.pc.wrapping_add(1)
            }

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
            Instruction::LD(ld_type) => {
                // match ld_type {
                //     LoadType::Byte(target, source) => {}
                //     LoadType::Word(target, source) => {}
                // }

                match ld_type {
                    LoadType::Byte(LoadByteTarget, LoadByteSource) => {
                        let source_value: u8;

                        match LoadByteSource {
                            LoadByteSource::A => source_value = self.registers.a,
                            LoadByteSource::B => source_value = self.registers.b,
                            LoadByteSource::C => source_value = self.registers.c,
                            LoadByteSource::D => source_value = self.registers.d,
                            LoadByteSource::E => source_value = self.registers.e,
                            LoadByteSource::H => source_value = self.registers.h,
                            LoadByteSource::L => source_value = self.registers.l,
                            LoadByteSource::HLI => {
                                source_value = self.bus.read_byte(self.registers.get_hl())
                            }
                            LoadByteSource::D8 => source_value = self.bus.read_byte(self.pc + 1),
                        }
                        match LoadByteTarget {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::HLI => {
                                self.bus.set_byte(self.registers.get_hl(), source_value)
                            }
                        }
                    }

                    LoadType::Word(LoadWordTarget, LoadWordSource) => {
                        let source_value: u16;

                        match LoadWordSource {
                            LoadWordSource::BC => source_value = self.registers.get_bc(),
                            LoadWordSource::DE => source_value = self.registers.get_de(),
                            LoadWordSource::HL => source_value = self.registers.get_hl(),
                            LoadWordSource::SP => source_value = self.sp,
                            LoadWordSource::D16 => {
                                let lower_byte = self.bus.read_byte(self.pc + 1) as u16;
                                let upper_byte = self.bus.read_byte(self.pc + 2) as u16;
                                source_value = (upper_byte << 8) | lower_byte;
                            }
                        }

                        match LoadWordTarget {
                            LoadWordTarget::BC => self.registers.set_bc(source_value),
                            LoadWordTarget::DE => self.registers.set_de(source_value),
                            LoadWordTarget::HL => self.registers.set_hl(source_value),
                            LoadWordTarget::SP => self.sp = source_value,
                        }
                    }

                    LoadType::AFromIndirect(target) => match target {
                        Indirect::BCI => {
                            self.registers.a = self.bus.read_byte(self.registers.get_bc())
                        }
                        Indirect::DEI => {
                            self.registers.a = self.bus.read_byte(self.registers.get_de())
                        }
                        Indirect::HLINC => {
                            self.registers.a = self.bus.read_byte(self.registers.get_hl());
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_add(1));
                        }
                        Indirect::HLDEC => {
                            self.registers.a = self.bus.read_byte(self.registers.get_hl());
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_sub(1));
                        }
                    },

                    LoadType::IndirectFromA(target) => match target {
                        Indirect::BCI => {
                            self.bus.set_byte(self.registers.get_bc(), self.registers.a)
                        }
                        Indirect::DEI => {
                            self.bus.set_byte(self.registers.get_de(), self.registers.a)
                        }
                        Indirect::HLINC => {
                            self.bus.set_byte(self.registers.get_hl(), self.registers.a);
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_add(1));
                        }
                        Indirect::HLDEC => {
                            self.bus.set_byte(self.registers.get_hl(), self.registers.a);
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_sub(1));
                        }
                    },
                    LoadType::AFromByteAddress(target) => match target {
                        ByteAddress::A8 => {
                            let address = 0xFF00 | self.bus.read_byte(self.pc + 1) as u16;
                            self.registers.a = self.bus.read_byte(address);
                        }
                        ByteAddress::C => {
                            let address = 0xFF00 | self.registers.c as u16;
                            self.registers.a = self.bus.read_byte(address);
                        }
                        ByteAddress::A16 => {
                            let lower_byte = self.bus.read_byte(self.pc + 1) as u16;
                            let upper_byte = self.bus.read_byte(self.pc + 2) as u16;
                            let address = upper_byte << 8 | lower_byte;
                            self.registers.a = self.bus.read_byte(address);
                        }
                    },

                    LoadType::ByteAddressFromA(target) => match target {
                        ByteAddress::A8 => {
                            let address = 0xFF00 | self.bus.read_byte(self.pc + 1) as u16;
                            self.bus.set_byte(address, self.registers.a);
                        }
                        ByteAddress::C => {
                            let address = 0xFF00 | self.registers.c as u16;
                            self.bus.set_byte(address, self.registers.a);
                        }
                        ByteAddress::A16 => {
                            let lower_byte = self.bus.read_byte(self.pc + 1) as u16;
                            let upper_byte = self.bus.read_byte(self.pc + 2) as u16;
                            let address = upper_byte << 8 | lower_byte;
                            self.bus.set_byte(address, self.registers.a);
                        }
                    },

                    LoadType::SPToAddress => {
                        let lower_byte = self.bus.read_byte(self.pc + 1) as u16;
                        let upper_byte = self.bus.read_byte(self.pc + 2) as u16;
                        let address = upper_byte << 8 | lower_byte;
                        self.bus.set_byte(address, self.sp as u8);
                        self.bus.set_byte(address + 1, (self.sp >> 8) as u8);
                    }
                }
                self.pc.wrapping_add(1)
            }
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
                match target {
                    RegisterTarget::A => {
                        self.registers.f.carry = self.registers.a & 0x80 > 1;
                        self.registers.a = (self.registers.a << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::B => {
                        self.registers.f.carry = self.registers.b & 0x80 > 1;
                        self.registers.b = (self.registers.b << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::C => {
                        self.registers.f.carry = self.registers.c & 0x80 > 1;
                        self.registers.c = (self.registers.c << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::D => {
                        self.registers.f.carry = self.registers.d & 0x80 > 1;
                        self.registers.d = (self.registers.d << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::E => {
                        self.registers.f.carry = self.registers.e & 0x80 > 1;
                        self.registers.e = (self.registers.e << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::H => {
                        self.registers.f.carry = self.registers.h & 0x80 > 1;
                        self.registers.h = (self.registers.h << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::L => {
                        self.registers.f.carry = self.registers.l & 0x80 > 1;
                        self.registers.l = (self.registers.l << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::HLI => {
                        self.registers.f.carry =
                            self.bus.read_byte(self.registers.get_hl()) & 0x80 > 1;
                        self.bus.set_byte(
                            self.registers.get_hl(),
                            (self.bus.read_byte(self.registers.get_hl()) << 1)
                                | (if self.registers.f.carry { 0x01 } else { 0x00 }),
                        );
                    }
                }
                self.pc.wrapping_add(2)
            }

            /* Rotate the contents of register to the right. That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy operation) are copied to bit 5. The same operation is repeated in sequence for the rest of the register. The contents of bit 0 are placed in both the CY flag and bit 7 of register. */
            Instruction::RRC(target) => {
                match target {
                    RegisterTarget::A => {
                        self.registers.f.carry = self.registers.a & 0x01 == 1;
                        self.registers.a = (self.registers.a >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::B => {
                        self.registers.f.carry = self.registers.b & 0x01 == 1;
                        self.registers.b = (self.registers.b >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::C => {
                        self.registers.f.carry = self.registers.c & 0x01 == 1;
                        self.registers.c = (self.registers.c >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::D => {
                        self.registers.f.carry = self.registers.d & 0x01 == 1;
                        self.registers.d = (self.registers.d >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::E => {
                        self.registers.f.carry = self.registers.e & 0x01 == 1;
                        self.registers.e = (self.registers.e >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::H => {
                        self.registers.f.carry = self.registers.h & 0x01 == 1;
                        self.registers.h = (self.registers.h >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::L => {
                        self.registers.f.carry = self.registers.l & 0x01 == 1;
                        self.registers.l = (self.registers.l >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::HLI => {
                        self.registers.f.carry =
                            self.bus.read_byte(self.registers.get_hl()) & 0x01 == 1;
                        self.bus.set_byte(
                            self.registers.get_hl(),
                            (self.bus.read_byte(self.registers.get_hl()) >> 1)
                                | (if self.registers.f.carry { 0x80 } else { 0x00 }),
                        );
                    }
                }
                self.pc.wrapping_add(2)
            }

            /* Rotate the contents of register to the left. That is, the contents of bit 0 are copied to bit 1, and the previous contents of bit 1 (before the copy operation) are copied to bit 2. The same operation is repeated in sequence for the rest of the register. The previous contents of the carry (CY) flag are copied to bit 0 of register. */
            Instruction::RL(target) => {
                match target {
                    RegisterTarget::A => {
                        self.registers.a = (self.registers.a << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::B => {
                        self.registers.b = (self.registers.b << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::C => {
                        self.registers.c = (self.registers.c << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::D => {
                        self.registers.d = (self.registers.d << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::E => {
                        self.registers.e = (self.registers.e << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::H => {
                        self.registers.h = (self.registers.h << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::L => {
                        self.registers.l = (self.registers.l << 1)
                            | (if self.registers.f.carry { 0x01 } else { 0x00 });
                    }
                    RegisterTarget::HLI => {
                        self.bus.set_byte(
                            self.registers.get_hl(),
                            (self.bus.read_byte(self.registers.get_hl()) << 1)
                                | (if self.registers.f.carry { 0x01 } else { 0x00 }),
                        );
                    }
                }
                self.pc.wrapping_add(2)
            }

            /* Rotate the contents of register to the right. That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy operation) are copied to bit 5. The same operation is repeated in sequence for the rest of the register. The previous contents of the carry (CY) flag are copied to bit 7 of register. */
            Instruction::RR(target) => {
                match target {
                    RegisterTarget::A => {
                        self.registers.a = (self.registers.a >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::B => {
                        self.registers.b = (self.registers.b >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::C => {
                        self.registers.c = (self.registers.c >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::D => {
                        self.registers.d = (self.registers.d >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::E => {
                        self.registers.e = (self.registers.e >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::H => {
                        self.registers.h = (self.registers.h >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::L => {
                        self.registers.l = (self.registers.l >> 1)
                            | (if self.registers.f.carry { 0x80 } else { 0x00 });
                    }
                    RegisterTarget::HLI => {
                        self.bus.set_byte(
                            self.registers.get_hl(),
                            (self.bus.read_byte(self.registers.get_hl()) >> 1)
                                | (if self.registers.f.carry { 0x80 } else { 0x00 }),
                        );
                    }
                }
                self.pc.wrapping_add(2)
            }

            /* Shift the contents of register to the left. That is, the contents of bit 0 are copied to bit 1, and the previous contents of bit 1 (before the copy operation) are copied to bit 2. The same operation is repeated in sequence for the rest of the register. The contents of bit 7 are copied to the CY flag, and bit 0 of register is reset to 0. */
            Instruction::SLA(target) => {
                match target {
                    RegisterTarget::A => {
                        self.registers.f.carry = self.registers.a & 0x7F > 0;
                        self.registers.a = (self.registers.a << 1) | (self.registers.a & 0x01);
                    }
                    RegisterTarget::B => {
                        self.registers.f.carry = self.registers.b & 0x7F > 0;
                        self.registers.b = (self.registers.a << 1) | (self.registers.b & 0x01);
                    }
                    RegisterTarget::C => {
                        self.registers.f.carry = self.registers.c & 0x7F > 0;
                        self.registers.c = (self.registers.c << 1) | (self.registers.c & 0x01);
                    }
                    RegisterTarget::D => {
                        self.registers.f.carry = self.registers.d & 0x7F > 0;
                        self.registers.d = (self.registers.d << 1) | (self.registers.d & 0x01);
                    }
                    RegisterTarget::E => {
                        self.registers.f.carry = self.registers.e & 0x7F > 0;
                        self.registers.e = (self.registers.e << 1) | (self.registers.e & 0x01);
                    }
                    RegisterTarget::H => {
                        self.registers.f.carry = self.registers.h & 0x7F > 0;
                        self.registers.h = (self.registers.h << 1) | (self.registers.h & 0x01);
                    }
                    RegisterTarget::L => {
                        self.registers.f.carry = self.registers.l & 0x7F > 0;
                        self.registers.l = (self.registers.l << 1) | (self.registers.l & 0x01);
                    }
                    RegisterTarget::HLI => {
                        self.registers.f.carry =
                            self.bus.read_byte(self.registers.get_hl()) & 0x7F > 0;
                        self.bus.set_byte(
                            self.registers.get_hl(),
                            (self.bus.read_byte(self.registers.get_hl()) << 1)
                                | (self.bus.read_byte(self.registers.get_hl()) & 0x01),
                        );
                    }
                }
                self.pc.wrapping_add(2)
            }

            /* Shift the contents of register to the right. That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy operation) are copied to bit 5. The same operation is repeated in sequence for the rest of the register. The contents of bit 0 are copied to the CY flag, and bit 7 of register is unchanged. */
            Instruction::SRA(target) => {
                match target {
                    RegisterTarget::A => {
                        self.registers.f.carry = self.registers.a & 0x01 == 1;
                        self.registers.a = (self.registers.a >> 1) | (self.registers.a & 0x80);
                    }
                    RegisterTarget::B => {
                        self.registers.f.carry = self.registers.b & 0x01 == 1;
                        self.registers.b = (self.registers.b >> 1) | (self.registers.b & 0x80);
                    }
                    RegisterTarget::C => {
                        self.registers.f.carry = self.registers.c & 0x01 == 1;
                        self.registers.c = (self.registers.c >> 1) | (self.registers.c & 0x80);
                    }
                    RegisterTarget::D => {
                        self.registers.f.carry = self.registers.d & 0x01 == 1;
                        self.registers.d = (self.registers.d >> 1) | (self.registers.d & 0x80);
                    }
                    RegisterTarget::E => {
                        self.registers.f.carry = self.registers.e & 0x01 == 1;
                        self.registers.e = (self.registers.e >> 1) | (self.registers.e & 0x80);
                    }
                    RegisterTarget::H => {
                        self.registers.f.carry = self.registers.h & 0x01 == 1;
                        self.registers.h = (self.registers.h >> 1) | (self.registers.h & 0x80);
                    }
                    RegisterTarget::L => {
                        self.registers.f.carry = self.registers.l & 0x01 == 1;
                        self.registers.l = (self.registers.l >> 1) | (self.registers.l & 0x80);
                    }
                    RegisterTarget::HLI => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        self.registers.f.carry = value & 0x01 == 1;
                        self.bus
                            .set_byte(self.registers.get_hl(), (value >> 1) | (value & 0x80));
                    }
                }
                self.pc.wrapping_add(2)
            }

            /* Shift the contents of the lower-order four bits (0-3) of register to the higher-order four bits (4-7) of the register, and shift the higher-order four bits to the lower-order four bits. */
            Instruction::SWAP(target) => {
                match target {
                    RegisterTarget::A => {
                        self.registers.a = (self.registers.a << 4) | (self.registers.a >> 4)
                    }
                    RegisterTarget::B => {
                        self.registers.b = (self.registers.b << 4) | (self.registers.b >> 4)
                    }
                    RegisterTarget::C => {
                        self.registers.c = (self.registers.c << 4) | (self.registers.c >> 4)
                    }
                    RegisterTarget::D => {
                        self.registers.d = (self.registers.d << 4) | (self.registers.d >> 4)
                    }
                    RegisterTarget::E => {
                        self.registers.e = (self.registers.e << 4) | (self.registers.e >> 4)
                    }
                    RegisterTarget::H => {
                        self.registers.h = (self.registers.h << 4) | (self.registers.h >> 4)
                    }
                    RegisterTarget::L => {
                        self.registers.l = (self.registers.l << 4) | (self.registers.l >> 4)
                    }
                    RegisterTarget::HLI => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        self.bus
                            .set_byte(self.registers.get_hl(), (value << 4) | (value >> 4));
                    }
                }
                self.pc.wrapping_add(2)
            }

            /* Shift the contents of register to the right. That is, the contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy operation) are copied to bit 5. The same operation is repeated in sequence for the rest of the register. The contents of bit 0 are copied to the CY flag, and bit 7 of register is reset to 0. */
            Instruction::SRL(target) => {
                match target {
                    RegisterTarget::A => {
                        self.registers.f.carry = self.registers.a & 0x01 == 1;
                        self.registers.a = (self.registers.a >> 1) & 0x7F;
                    }
                    RegisterTarget::B => {
                        self.registers.f.carry = self.registers.b & 0x01 == 1;
                        self.registers.b = (self.registers.b >> 1) & 0x7F;
                    }
                    RegisterTarget::C => {
                        self.registers.f.carry = self.registers.c & 0x01 == 1;
                        self.registers.c = (self.registers.c >> 1) & 0x7F;
                    }
                    RegisterTarget::D => {
                        self.registers.f.carry = self.registers.d & 0x01 == 1;
                        self.registers.d = (self.registers.d >> 1) & 0x7F;
                    }
                    RegisterTarget::E => {
                        self.registers.f.carry = self.registers.e & 0x01 == 1;
                        self.registers.e = (self.registers.e >> 1) & 0x7F;
                    }
                    RegisterTarget::H => {
                        self.registers.f.carry = self.registers.h & 0x01 == 1;
                        self.registers.h = (self.registers.h >> 1) & 0x7F;
                    }
                    RegisterTarget::L => {
                        self.registers.f.carry = self.registers.l & 0x01 == 1;
                        self.registers.l = (self.registers.l >> 1) & 0x7F;
                    }
                    RegisterTarget::HLI => {
                        self.registers.f.carry =
                            self.bus.read_byte(self.registers.get_hl()) & 0x01 == 1;
                        self.bus.set_byte(
                            self.registers.get_hl(),
                            (self.bus.read_byte(self.registers.get_hl())) >> 1 & 0x7F,
                        )
                    }
                }
                self.pc.wrapping_add(2)
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

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false; // CHECK THIS ONCE
        new_value
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
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
