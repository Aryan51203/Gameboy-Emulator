use crate::instructions::{
    ByteAddress, Indirect, LoadByteSource, LoadByteTarget, LoadType, LoadWordSource, LoadWordTarget,
};
use crate::memory::MemoryBus;
use crate::registers::Registers;

pub fn load(
    registers: &mut Registers,
    bus: &mut MemoryBus,
    pc: u16,
    ld_type: LoadType,
    sp: &mut u16,
) -> u16 {
    {
        match ld_type {
            LoadType::Byte(LoadByteTarget, LoadByteSource) => {
                let (source_value, pc_increment) = match LoadByteSource {
                    LoadByteSource::A => (registers.a, 1),
                    LoadByteSource::B => (registers.b, 1),
                    LoadByteSource::C => (registers.c, 1),
                    LoadByteSource::D => (registers.d, 1),
                    LoadByteSource::E => (registers.e, 1),
                    LoadByteSource::H => (registers.h, 1),
                    LoadByteSource::L => (registers.l, 1),
                    LoadByteSource::HLI => (bus.read_byte(registers.get_hl()), 1),
                    LoadByteSource::D8 => (bus.read_byte(pc + 1), 2),
                };

                match LoadByteTarget {
                    LoadByteTarget::A => registers.a = source_value,
                    LoadByteTarget::B => registers.b = source_value,
                    LoadByteTarget::C => registers.c = source_value,
                    LoadByteTarget::D => registers.d = source_value,
                    LoadByteTarget::E => registers.e = source_value,
                    LoadByteTarget::H => registers.h = source_value,
                    LoadByteTarget::L => registers.l = source_value,
                    LoadByteTarget::HLI => bus.set_byte(registers.get_hl(), source_value),
                };
                pc.wrapping_add(pc_increment)
            }

            LoadType::Word(LoadWordTarget, LoadWordSource) => {
                let (source_value, pc_increment) = match LoadWordSource {
                    LoadWordSource::BC => (registers.get_bc(), 1),
                    LoadWordSource::DE => (registers.get_de(), 1),
                    LoadWordSource::HL => (registers.get_hl(), 1),
                    LoadWordSource::SP => (*sp, 1),
                    LoadWordSource::D16 => {
                        let lower_byte = bus.read_byte(pc + 1) as u16;
                        let upper_byte = bus.read_byte(pc + 2) as u16;
                        ((upper_byte << 8) | lower_byte, 3)
                    }
                };

                match LoadWordTarget {
                    LoadWordTarget::BC => registers.set_bc(source_value),
                    LoadWordTarget::DE => registers.set_de(source_value),
                    LoadWordTarget::HL => registers.set_hl(source_value),
                    LoadWordTarget::SP => *sp = source_value,
                };
                pc.wrapping_add(pc_increment)
            }

            LoadType::AFromIndirect(target) => {
                match target {
                    Indirect::BCI => registers.a = bus.read_byte(registers.get_bc()),
                    Indirect::DEI => registers.a = bus.read_byte(registers.get_de()),
                    Indirect::HLINC => {
                        registers.a = bus.read_byte(registers.get_hl());
                        registers.set_hl(registers.get_hl().wrapping_add(1));
                    }
                    Indirect::HLDEC => {
                        registers.a = bus.read_byte(registers.get_hl());
                        registers.set_hl(registers.get_hl().wrapping_sub(1));
                    }
                }
                pc.wrapping_add(1)
            }

            LoadType::IndirectFromA(target) => {
                match target {
                    Indirect::BCI => bus.set_byte(registers.get_bc(), registers.a),
                    Indirect::DEI => bus.set_byte(registers.get_de(), registers.a),
                    Indirect::HLINC => {
                        bus.set_byte(registers.get_hl(), registers.a);
                        registers.set_hl(registers.get_hl().wrapping_add(1));
                    }
                    Indirect::HLDEC => {
                        bus.set_byte(registers.get_hl(), registers.a);
                        registers.set_hl(registers.get_hl().wrapping_sub(1));
                    }
                }
                pc.wrapping_add(1)
            }

            LoadType::AFromByteAddress(target) => match target {
                ByteAddress::A8 => {
                    let address = 0xFF00 | bus.read_byte(pc + 1) as u16;
                    registers.a = bus.read_byte(address);
                    pc.wrapping_add(2)
                }
                ByteAddress::C => {
                    let address = 0xFF00 | registers.c as u16;
                    registers.a = bus.read_byte(address);
                    pc.wrapping_add(1)
                }
                ByteAddress::A16 => {
                    let lower_byte = bus.read_byte(pc + 1) as u16;
                    let upper_byte = bus.read_byte(pc + 2) as u16;
                    let address = upper_byte << 8 | lower_byte;
                    registers.a = bus.read_byte(address);
                    pc.wrapping_add(3)
                }
            },

            LoadType::ByteAddressFromA(target) => match target {
                ByteAddress::A8 => {
                    let address = 0xFF00 | bus.read_byte(pc + 1) as u16;
                    bus.set_byte(address, registers.a);
                    pc.wrapping_add(2)
                }
                ByteAddress::C => {
                    let address = 0xFF00 | registers.c as u16;
                    bus.set_byte(address, registers.a);
                    pc.wrapping_add(1)
                }
                ByteAddress::A16 => {
                    let lower_byte = bus.read_byte(pc + 1) as u16;
                    let upper_byte = bus.read_byte(pc + 2) as u16;
                    let address = upper_byte << 8 | lower_byte;
                    bus.set_byte(address, registers.a);
                    pc.wrapping_add(3)
                }
            },

            LoadType::SPToAddress => {
                let lower_byte = bus.read_byte(pc + 1) as u16;
                let upper_byte = bus.read_byte(pc + 2) as u16;
                let address = upper_byte << 8 | lower_byte;
                bus.set_byte(address, *sp as u8);
                bus.set_byte(address + 1, (*sp >> 8) as u8);
                pc.wrapping_add(3)
            }
        }
    }
}
