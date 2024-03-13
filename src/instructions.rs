pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDL(ArithmeticTargetLong),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    CMP(ArithmeticTarget),
    INC(IncDecTarget),
    DEC(IncDecTarget),

    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),

    RLCA,
    RRCA,
    RLA,
    RRA,
    RLC(RegisterTarget),
    RRC(RegisterTarget),
    RL(RegisterTarget),
    RR(RegisterTarget),

    SLA(RegisterTarget),
    SRA(RegisterTarget),
    SWAP(RegisterTarget),
    SRL(RegisterTarget),

    BIT(u8, RegisterTarget),
    RES(u8, RegisterTarget),
    SET(u8, RegisterTarget),

    JP(JumpType),
    JPL,
    JR(JumpType),
    CALL(JumpType),
    RET(JumpType),
    RETI,
    RST(u8),

    PUSH(StackRegisters),
    POP(StackRegisters),

    LD(LoadType), // Target is before source

    //Others
    /* To be read about */
    CCF,
    SCF,
    DAA,
    CPL,
    HALT,
    STOP,
    DI,
    EI,
    NOP,
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    D8,
}

pub enum ArithmeticTargetLong {
    BC,
    DE,
    HL,
    SP,
    S8,
}

pub enum IncDecTarget {
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    A,
    BC,
    DE,
    HL,
    SP,
}

pub enum RegisterTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum JumpType {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum StackRegisters {
    AF,
    BC,
    DE,
    HL,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
    AFromIndirect(Indirect),
    IndirectFromA(Indirect),
    AFromByteAddress(ByteAddress),
    ByteAddressFromA(ByteAddress),
    SPToAddress,
}
pub enum LoadByteTarget {
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    A,
}

pub enum LoadByteSource {
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    A,
    D8,
}

pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
}

pub enum LoadWordSource {
    BC,
    DE,
    HL,
    SP,
    D16,
}

pub enum Indirect {
    BCI,
    DEI,
    HLINC,
    HLDEC,
}

pub enum ByteAddress {
    A8,
    C,
    A16,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // Rotate Left Carry Instructions
            0x00 => Some(Instruction::RLC(RegisterTarget::B)),
            0x01 => Some(Instruction::RLC(RegisterTarget::C)),
            0x02 => Some(Instruction::RLC(RegisterTarget::D)),
            0x03 => Some(Instruction::RLC(RegisterTarget::E)),
            0x04 => Some(Instruction::RLC(RegisterTarget::H)),
            0x05 => Some(Instruction::RLC(RegisterTarget::L)),
            0x06 => Some(Instruction::RLC(RegisterTarget::HLI)),
            0x07 => Some(Instruction::RLC(RegisterTarget::A)),

            // Rotate Right Carry Instructions
            0x08 => Some(Instruction::RRC(RegisterTarget::B)),
            0x09 => Some(Instruction::RRC(RegisterTarget::C)),
            0x0A => Some(Instruction::RRC(RegisterTarget::D)),
            0x0B => Some(Instruction::RRC(RegisterTarget::E)),
            0x0C => Some(Instruction::RRC(RegisterTarget::H)),
            0x0D => Some(Instruction::RRC(RegisterTarget::L)),
            0x0E => Some(Instruction::RRC(RegisterTarget::HLI)),
            0x0F => Some(Instruction::RRC(RegisterTarget::A)),

            //Rotate Left Instructions
            0x10 => Some(Instruction::RL(RegisterTarget::B)),
            0x11 => Some(Instruction::RL(RegisterTarget::C)),
            0x12 => Some(Instruction::RL(RegisterTarget::D)),
            0x13 => Some(Instruction::RL(RegisterTarget::E)),
            0x14 => Some(Instruction::RL(RegisterTarget::H)),
            0x15 => Some(Instruction::RL(RegisterTarget::L)),
            0x16 => Some(Instruction::RL(RegisterTarget::HLI)),
            0x17 => Some(Instruction::RL(RegisterTarget::A)),

            //Rotate Right Instructions
            0x18 => Some(Instruction::RR(RegisterTarget::B)),
            0x19 => Some(Instruction::RR(RegisterTarget::C)),
            0x1A => Some(Instruction::RR(RegisterTarget::D)),
            0x1B => Some(Instruction::RR(RegisterTarget::E)),
            0x1C => Some(Instruction::RR(RegisterTarget::H)),
            0x1D => Some(Instruction::RR(RegisterTarget::L)),
            0x1E => Some(Instruction::RR(RegisterTarget::HLI)),
            0x1F => Some(Instruction::RR(RegisterTarget::A)),

            // Shift Left Arithmetic Instructions
            0x20 => Some(Instruction::SLA(RegisterTarget::B)),
            0x21 => Some(Instruction::SLA(RegisterTarget::C)),
            0x22 => Some(Instruction::SLA(RegisterTarget::D)),
            0x23 => Some(Instruction::SLA(RegisterTarget::E)),
            0x24 => Some(Instruction::SLA(RegisterTarget::H)),
            0x25 => Some(Instruction::SLA(RegisterTarget::L)),
            0x26 => Some(Instruction::SLA(RegisterTarget::HLI)),
            0x27 => Some(Instruction::SLA(RegisterTarget::A)),

            // Shift Right Arithmetic Instructions
            0x28 => Some(Instruction::SRA(RegisterTarget::B)),
            0x29 => Some(Instruction::SRA(RegisterTarget::C)),
            0x2A => Some(Instruction::SRA(RegisterTarget::D)),
            0x2B => Some(Instruction::SRA(RegisterTarget::E)),
            0x2C => Some(Instruction::SRA(RegisterTarget::H)),
            0x2D => Some(Instruction::SRA(RegisterTarget::L)),
            0x2E => Some(Instruction::SRA(RegisterTarget::HLI)),
            0x2F => Some(Instruction::SRA(RegisterTarget::A)),

            // Swap Instructions
            0x30 => Some(Instruction::SWAP(RegisterTarget::B)),
            0x31 => Some(Instruction::SWAP(RegisterTarget::C)),
            0x32 => Some(Instruction::SWAP(RegisterTarget::D)),
            0x33 => Some(Instruction::SWAP(RegisterTarget::E)),
            0x34 => Some(Instruction::SWAP(RegisterTarget::H)),
            0x35 => Some(Instruction::SWAP(RegisterTarget::L)),
            0x36 => Some(Instruction::SWAP(RegisterTarget::HLI)),
            0x37 => Some(Instruction::SWAP(RegisterTarget::A)),
            // Shift Right Logical Instructions
            0x38 => Some(Instruction::SRL(RegisterTarget::B)),
            0x39 => Some(Instruction::SRL(RegisterTarget::C)),
            0x3A => Some(Instruction::SRL(RegisterTarget::D)),
            0x3B => Some(Instruction::SRL(RegisterTarget::E)),
            0x3C => Some(Instruction::SRL(RegisterTarget::H)),
            0x3D => Some(Instruction::SRL(RegisterTarget::L)),
            0x3E => Some(Instruction::SRL(RegisterTarget::HLI)),
            0x3F => Some(Instruction::SRL(RegisterTarget::A)),

            // Bit instructions
            0x40 => Some(Instruction::BIT(0x00, RegisterTarget::B)),
            0x41 => Some(Instruction::BIT(0x00, RegisterTarget::C)),
            0x42 => Some(Instruction::BIT(0x00, RegisterTarget::D)),
            0x43 => Some(Instruction::BIT(0x00, RegisterTarget::E)),
            0x44 => Some(Instruction::BIT(0x00, RegisterTarget::H)),
            0x45 => Some(Instruction::BIT(0x00, RegisterTarget::L)),
            0x46 => Some(Instruction::BIT(0x00, RegisterTarget::HLI)),
            0x47 => Some(Instruction::BIT(0x00, RegisterTarget::A)),
            0x48 => Some(Instruction::BIT(0x01, RegisterTarget::B)),
            0x49 => Some(Instruction::BIT(0x01, RegisterTarget::C)),
            0x4A => Some(Instruction::BIT(0x01, RegisterTarget::D)),
            0x4B => Some(Instruction::BIT(0x01, RegisterTarget::E)),
            0x4C => Some(Instruction::BIT(0x01, RegisterTarget::H)),
            0x4D => Some(Instruction::BIT(0x01, RegisterTarget::L)),
            0x4E => Some(Instruction::BIT(0x01, RegisterTarget::HLI)),
            0x4F => Some(Instruction::BIT(0x01, RegisterTarget::A)),
            0x50 => Some(Instruction::BIT(0x02, RegisterTarget::B)),
            0x51 => Some(Instruction::BIT(0x02, RegisterTarget::C)),
            0x52 => Some(Instruction::BIT(0x02, RegisterTarget::D)),
            0x53 => Some(Instruction::BIT(0x02, RegisterTarget::E)),
            0x54 => Some(Instruction::BIT(0x02, RegisterTarget::H)),
            0x55 => Some(Instruction::BIT(0x02, RegisterTarget::L)),
            0x56 => Some(Instruction::BIT(0x02, RegisterTarget::HLI)),
            0x57 => Some(Instruction::BIT(0x02, RegisterTarget::A)),
            0x58 => Some(Instruction::BIT(0x03, RegisterTarget::B)),
            0x59 => Some(Instruction::BIT(0x03, RegisterTarget::C)),
            0x5A => Some(Instruction::BIT(0x03, RegisterTarget::D)),
            0x5B => Some(Instruction::BIT(0x03, RegisterTarget::E)),
            0x5C => Some(Instruction::BIT(0x03, RegisterTarget::H)),
            0x5D => Some(Instruction::BIT(0x03, RegisterTarget::L)),
            0x5E => Some(Instruction::BIT(0x03, RegisterTarget::HLI)),
            0x5F => Some(Instruction::BIT(0x03, RegisterTarget::A)),
            0x60 => Some(Instruction::BIT(0x04, RegisterTarget::B)),
            0x61 => Some(Instruction::BIT(0x04, RegisterTarget::C)),
            0x62 => Some(Instruction::BIT(0x04, RegisterTarget::D)),
            0x63 => Some(Instruction::BIT(0x04, RegisterTarget::E)),
            0x64 => Some(Instruction::BIT(0x04, RegisterTarget::H)),
            0x65 => Some(Instruction::BIT(0x04, RegisterTarget::L)),
            0x66 => Some(Instruction::BIT(0x04, RegisterTarget::HLI)),
            0x67 => Some(Instruction::BIT(0x04, RegisterTarget::A)),
            0x68 => Some(Instruction::BIT(0x05, RegisterTarget::B)),
            0x69 => Some(Instruction::BIT(0x05, RegisterTarget::C)),
            0x6A => Some(Instruction::BIT(0x05, RegisterTarget::D)),
            0x6B => Some(Instruction::BIT(0x05, RegisterTarget::E)),
            0x6C => Some(Instruction::BIT(0x05, RegisterTarget::H)),
            0x6D => Some(Instruction::BIT(0x05, RegisterTarget::L)),
            0x6E => Some(Instruction::BIT(0x05, RegisterTarget::HLI)),
            0x6F => Some(Instruction::BIT(0x05, RegisterTarget::A)),
            0x70 => Some(Instruction::BIT(0x06, RegisterTarget::B)),
            0x71 => Some(Instruction::BIT(0x06, RegisterTarget::C)),
            0x72 => Some(Instruction::BIT(0x06, RegisterTarget::D)),
            0x73 => Some(Instruction::BIT(0x06, RegisterTarget::E)),
            0x74 => Some(Instruction::BIT(0x06, RegisterTarget::H)),
            0x75 => Some(Instruction::BIT(0x06, RegisterTarget::L)),
            0x76 => Some(Instruction::BIT(0x06, RegisterTarget::HLI)),
            0x77 => Some(Instruction::BIT(0x06, RegisterTarget::A)),
            0x78 => Some(Instruction::BIT(0x07, RegisterTarget::B)),
            0x79 => Some(Instruction::BIT(0x07, RegisterTarget::C)),
            0x7A => Some(Instruction::BIT(0x07, RegisterTarget::D)),
            0x7B => Some(Instruction::BIT(0x07, RegisterTarget::E)),
            0x7C => Some(Instruction::BIT(0x07, RegisterTarget::H)),
            0x7D => Some(Instruction::BIT(0x07, RegisterTarget::L)),
            0x7E => Some(Instruction::BIT(0x07, RegisterTarget::HLI)),
            0x7F => Some(Instruction::BIT(0x07, RegisterTarget::A)),

            // Reset instructions
            0x80 => Some(Instruction::RES(0x00, RegisterTarget::B)),
            0x81 => Some(Instruction::RES(0x00, RegisterTarget::C)),
            0x82 => Some(Instruction::RES(0x00, RegisterTarget::D)),
            0x83 => Some(Instruction::RES(0x00, RegisterTarget::E)),
            0x84 => Some(Instruction::RES(0x00, RegisterTarget::H)),
            0x85 => Some(Instruction::RES(0x00, RegisterTarget::L)),
            0x86 => Some(Instruction::RES(0x00, RegisterTarget::HLI)),
            0x87 => Some(Instruction::RES(0x00, RegisterTarget::A)),
            0x88 => Some(Instruction::RES(0x01, RegisterTarget::B)),
            0x89 => Some(Instruction::RES(0x01, RegisterTarget::C)),
            0x8A => Some(Instruction::RES(0x01, RegisterTarget::D)),
            0x8B => Some(Instruction::RES(0x01, RegisterTarget::E)),
            0x8C => Some(Instruction::RES(0x01, RegisterTarget::H)),
            0x8D => Some(Instruction::RES(0x01, RegisterTarget::L)),
            0x8E => Some(Instruction::RES(0x01, RegisterTarget::HLI)),
            0x8F => Some(Instruction::RES(0x01, RegisterTarget::A)),
            0x90 => Some(Instruction::RES(0x02, RegisterTarget::B)),
            0x91 => Some(Instruction::RES(0x02, RegisterTarget::C)),
            0x92 => Some(Instruction::RES(0x02, RegisterTarget::D)),
            0x93 => Some(Instruction::RES(0x02, RegisterTarget::E)),
            0x94 => Some(Instruction::RES(0x02, RegisterTarget::H)),
            0x95 => Some(Instruction::RES(0x02, RegisterTarget::L)),
            0x96 => Some(Instruction::RES(0x02, RegisterTarget::HLI)),
            0x97 => Some(Instruction::RES(0x02, RegisterTarget::A)),
            0x98 => Some(Instruction::RES(0x03, RegisterTarget::B)),
            0x99 => Some(Instruction::RES(0x03, RegisterTarget::C)),
            0x9A => Some(Instruction::RES(0x03, RegisterTarget::D)),
            0x9B => Some(Instruction::RES(0x03, RegisterTarget::E)),
            0x9C => Some(Instruction::RES(0x03, RegisterTarget::H)),
            0x9D => Some(Instruction::RES(0x03, RegisterTarget::L)),
            0x9E => Some(Instruction::RES(0x03, RegisterTarget::HLI)),
            0x9F => Some(Instruction::RES(0x03, RegisterTarget::A)),
            0xA0 => Some(Instruction::RES(0x04, RegisterTarget::B)),
            0xA1 => Some(Instruction::RES(0x04, RegisterTarget::C)),
            0xA2 => Some(Instruction::RES(0x04, RegisterTarget::D)),
            0xA3 => Some(Instruction::RES(0x04, RegisterTarget::E)),
            0xA4 => Some(Instruction::RES(0x04, RegisterTarget::H)),
            0xA5 => Some(Instruction::RES(0x04, RegisterTarget::L)),
            0xA6 => Some(Instruction::RES(0x04, RegisterTarget::HLI)),
            0xA7 => Some(Instruction::RES(0x04, RegisterTarget::A)),
            0xA8 => Some(Instruction::RES(0x05, RegisterTarget::B)),
            0xA9 => Some(Instruction::RES(0x05, RegisterTarget::C)),
            0xAA => Some(Instruction::RES(0x05, RegisterTarget::D)),
            0xAB => Some(Instruction::RES(0x05, RegisterTarget::E)),
            0xAC => Some(Instruction::RES(0x05, RegisterTarget::H)),
            0xAD => Some(Instruction::RES(0x05, RegisterTarget::L)),
            0xAE => Some(Instruction::RES(0x05, RegisterTarget::HLI)),
            0xAF => Some(Instruction::RES(0x05, RegisterTarget::A)),
            0xB0 => Some(Instruction::RES(0x06, RegisterTarget::B)),
            0xB1 => Some(Instruction::RES(0x06, RegisterTarget::C)),
            0xB2 => Some(Instruction::RES(0x06, RegisterTarget::D)),
            0xB3 => Some(Instruction::RES(0x06, RegisterTarget::E)),
            0xB4 => Some(Instruction::RES(0x06, RegisterTarget::H)),
            0xB5 => Some(Instruction::RES(0x06, RegisterTarget::L)),
            0xB6 => Some(Instruction::RES(0x06, RegisterTarget::HLI)),
            0xB7 => Some(Instruction::RES(0x06, RegisterTarget::A)),
            0xB8 => Some(Instruction::RES(0x07, RegisterTarget::B)),
            0xB9 => Some(Instruction::RES(0x07, RegisterTarget::C)),
            0xBA => Some(Instruction::RES(0x07, RegisterTarget::D)),
            0xBB => Some(Instruction::RES(0x07, RegisterTarget::E)),
            0xBC => Some(Instruction::RES(0x07, RegisterTarget::H)),
            0xBD => Some(Instruction::RES(0x07, RegisterTarget::L)),
            0xBE => Some(Instruction::RES(0x07, RegisterTarget::HLI)),
            0xBF => Some(Instruction::RES(0x07, RegisterTarget::A)),

            // Set instructions
            0xC0 => Some(Instruction::SET(0x00, RegisterTarget::B)),
            0xC1 => Some(Instruction::SET(0x00, RegisterTarget::C)),
            0xC2 => Some(Instruction::SET(0x00, RegisterTarget::D)),
            0xC3 => Some(Instruction::SET(0x00, RegisterTarget::E)),
            0xC4 => Some(Instruction::SET(0x00, RegisterTarget::H)),
            0xC5 => Some(Instruction::SET(0x00, RegisterTarget::L)),
            0xC6 => Some(Instruction::SET(0x00, RegisterTarget::HLI)),
            0xC7 => Some(Instruction::SET(0x00, RegisterTarget::A)),
            0xC8 => Some(Instruction::SET(0x01, RegisterTarget::B)),
            0xC9 => Some(Instruction::SET(0x01, RegisterTarget::C)),
            0xCA => Some(Instruction::SET(0x01, RegisterTarget::D)),
            0xCB => Some(Instruction::SET(0x01, RegisterTarget::E)),
            0xCC => Some(Instruction::SET(0x01, RegisterTarget::H)),
            0xCD => Some(Instruction::SET(0x01, RegisterTarget::L)),
            0xCE => Some(Instruction::SET(0x01, RegisterTarget::HLI)),
            0xCF => Some(Instruction::SET(0x01, RegisterTarget::A)),
            0xD0 => Some(Instruction::SET(0x02, RegisterTarget::B)),
            0xD1 => Some(Instruction::SET(0x02, RegisterTarget::C)),
            0xD2 => Some(Instruction::SET(0x02, RegisterTarget::D)),
            0xD3 => Some(Instruction::SET(0x02, RegisterTarget::E)),
            0xD4 => Some(Instruction::SET(0x02, RegisterTarget::H)),
            0xD5 => Some(Instruction::SET(0x02, RegisterTarget::L)),
            0xD6 => Some(Instruction::SET(0x02, RegisterTarget::HLI)),
            0xD7 => Some(Instruction::SET(0x02, RegisterTarget::A)),
            0xD8 => Some(Instruction::SET(0x03, RegisterTarget::B)),
            0xD9 => Some(Instruction::SET(0x03, RegisterTarget::C)),
            0xDA => Some(Instruction::SET(0x03, RegisterTarget::D)),
            0xDB => Some(Instruction::SET(0x03, RegisterTarget::E)),
            0xDC => Some(Instruction::SET(0x03, RegisterTarget::H)),
            0xDD => Some(Instruction::SET(0x03, RegisterTarget::L)),
            0xDE => Some(Instruction::SET(0x03, RegisterTarget::HLI)),
            0xDF => Some(Instruction::SET(0x03, RegisterTarget::A)),
            0xE0 => Some(Instruction::SET(0x04, RegisterTarget::B)),
            0xE1 => Some(Instruction::SET(0x04, RegisterTarget::C)),
            0xE2 => Some(Instruction::SET(0x04, RegisterTarget::D)),
            0xE3 => Some(Instruction::SET(0x04, RegisterTarget::E)),
            0xE4 => Some(Instruction::SET(0x04, RegisterTarget::H)),
            0xE5 => Some(Instruction::SET(0x04, RegisterTarget::L)),
            0xE6 => Some(Instruction::SET(0x04, RegisterTarget::HLI)),
            0xE7 => Some(Instruction::SET(0x04, RegisterTarget::A)),
            0xE8 => Some(Instruction::SET(0x05, RegisterTarget::B)),
            0xE9 => Some(Instruction::SET(0x05, RegisterTarget::C)),
            0xEA => Some(Instruction::SET(0x05, RegisterTarget::D)),
            0xEB => Some(Instruction::SET(0x05, RegisterTarget::E)),
            0xEC => Some(Instruction::SET(0x05, RegisterTarget::H)),
            0xED => Some(Instruction::SET(0x05, RegisterTarget::L)),
            0xEE => Some(Instruction::SET(0x05, RegisterTarget::HLI)),
            0xEF => Some(Instruction::SET(0x05, RegisterTarget::A)),
            0xF0 => Some(Instruction::SET(0x06, RegisterTarget::B)),
            0xF1 => Some(Instruction::SET(0x06, RegisterTarget::C)),
            0xF2 => Some(Instruction::SET(0x06, RegisterTarget::D)),
            0xF3 => Some(Instruction::SET(0x06, RegisterTarget::E)),
            0xF4 => Some(Instruction::SET(0x06, RegisterTarget::H)),
            0xF5 => Some(Instruction::SET(0x06, RegisterTarget::L)),
            0xF6 => Some(Instruction::SET(0x06, RegisterTarget::HLI)),
            0xF7 => Some(Instruction::SET(0x06, RegisterTarget::A)),
            0xF8 => Some(Instruction::SET(0x07, RegisterTarget::B)),
            0xF9 => Some(Instruction::SET(0x07, RegisterTarget::C)),
            0xFA => Some(Instruction::SET(0x07, RegisterTarget::D)),
            0xFB => Some(Instruction::SET(0x07, RegisterTarget::E)),
            0xFC => Some(Instruction::SET(0x07, RegisterTarget::H)),
            0xFD => Some(Instruction::SET(0x07, RegisterTarget::L)),
            0xFE => Some(Instruction::SET(0x07, RegisterTarget::HLI)),
            0xFF => Some(Instruction::SET(0x07, RegisterTarget::A)),
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // Add instructions
            0x09 => Some(Instruction::ADDL(ArithmeticTargetLong::BC)),
            0x19 => Some(Instruction::ADDL(ArithmeticTargetLong::DE)),
            0x29 => Some(Instruction::ADDL(ArithmeticTargetLong::HL)),
            0x39 => Some(Instruction::ADDL(ArithmeticTargetLong::SP)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HLI)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0xC6 => Some(Instruction::ADD(ArithmeticTarget::D8)),
            0xE8 => Some(Instruction::ADDL(ArithmeticTargetLong::S8)), // S8 + SP
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HLI)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),
            0xCE => Some(Instruction::ADC(ArithmeticTarget::D8)),

            // Sub instructions
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HLI)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::D8)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HLI)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),
            0xDE => Some(Instruction::SBC(ArithmeticTarget::D8)),

            // Compare Instructions
            0xB8 => Some(Instruction::CMP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CMP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CMP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CMP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CMP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CMP(ArithmeticTarget::L)),
            0xBE => Some(Instruction::CMP(ArithmeticTarget::HLI)),
            0xBF => Some(Instruction::CMP(ArithmeticTarget::A)),
            0xFE => Some(Instruction::CMP(ArithmeticTarget::D8)),

            // Increment Instructions
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),

            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x34 => Some(Instruction::INC(IncDecTarget::HLI)),

            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            0x2C => Some(Instruction::INC(IncDecTarget::L)),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),

            // Decrement Instructions
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x35 => Some(Instruction::DEC(IncDecTarget::HLI)),

            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
            0x2B => Some(Instruction::DEC(IncDecTarget::HL)),
            0x3B => Some(Instruction::DEC(IncDecTarget::SP)),

            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x2D => Some(Instruction::DEC(IncDecTarget::L)),
            0x3D => Some(Instruction::DEC(IncDecTarget::A)),

            // Bitwise AND
            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HLI)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::D8)),

            // Bitwise OR
            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::HLI)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::D8)),

            // Bitwise XOR
            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HLI)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xEE => Some(Instruction::XOR(ArithmeticTarget::D8)),

            // Other instructions
            0x3F => Some(Instruction::CCF),
            0x37 => Some(Instruction::SCF),
            0x27 => Some(Instruction::DAA),
            0x2F => Some(Instruction::CPL),

            // Rotate Instructions
            0x07 => Some(Instruction::RLCA),
            0x0F => Some(Instruction::RRCA),
            0x17 => Some(Instruction::RLA),
            0x1F => Some(Instruction::RRA),

            // Jump Instructions
            0xC2 => Some(Instruction::JP(JumpType::NotZero)),
            0xC3 => Some(Instruction::JP(JumpType::Always)),
            0xCA => Some(Instruction::JP(JumpType::Zero)),
            0xD2 => Some(Instruction::JP(JumpType::NotCarry)),
            0xDA => Some(Instruction::JP(JumpType::Carry)),
            0xE9 => Some(Instruction::JPL),

            //Jump Relative Instructions
            0x18 => Some(Instruction::JR(JumpType::Always)),
            0x28 => Some(Instruction::JR(JumpType::Zero)),
            0x38 => Some(Instruction::JR(JumpType::Carry)),
            0x20 => Some(Instruction::JR(JumpType::NotZero)),
            0x30 => Some(Instruction::JR(JumpType::NotCarry)),

            // Call instructions
            0xC4 => Some(Instruction::CALL(JumpType::NotZero)),
            0xCC => Some(Instruction::CALL(JumpType::Zero)),
            0xCD => Some(Instruction::CALL(JumpType::Always)),
            0xD4 => Some(Instruction::CALL(JumpType::NotCarry)),
            0xDC => Some(Instruction::CALL(JumpType::Carry)),

            //Return Instructions
            0xC0 => Some(Instruction::RET(JumpType::NotZero)),
            0xC8 => Some(Instruction::RET(JumpType::Zero)),
            0xC9 => Some(Instruction::RET(JumpType::Always)),
            0xD0 => Some(Instruction::RET(JumpType::NotCarry)),
            0xD8 => Some(Instruction::RET(JumpType::Carry)),
            0xD9 => Some(Instruction::RETI),
            0xC7 => Some(Instruction::RST(0x00)),
            0xD7 => Some(Instruction::RST(0x02)),
            0xE7 => Some(Instruction::RST(0x04)),
            0xF7 => Some(Instruction::RST(0x06)),
            0xCF => Some(Instruction::RST(0x01)),
            0xDF => Some(Instruction::RST(0x03)),
            0xEF => Some(Instruction::RST(0x05)),
            0xFF => Some(Instruction::RST(0x07)),

            // Miscellanous instructions
            0x00 => Some(Instruction::NOP),
            0x10 => Some(Instruction::STOP),
            0x76 => Some(Instruction::HALT),
            0xF3 => Some(Instruction::DI),
            0xFB => Some(Instruction::EI),

            // Load instructions
            0x01 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::BC,
                LoadWordSource::D16,
            ))),
            0x02 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::BCI))),
            0x06 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D8,
            ))),
            0x08 => Some(Instruction::LD(LoadType::SPToAddress)),
            0x0A => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::BCI))),
            0x0E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D8,
            ))),
            0x11 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::DE,
                LoadWordSource::D16,
            ))),
            0x12 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::DEI))),
            0x16 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D8,
            ))),
            0x1A => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::DEI))),
            0x1E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D8,
            ))),
            0x21 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::HL,
                LoadWordSource::D16,
            ))),
            0x22 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::HLINC))),
            0x26 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D8,
            ))),
            0x2A => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::HLINC))),
            0x2E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D8,
            ))),
            0x31 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::SP,
                LoadWordSource::D16,
            ))),
            0x32 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::HLDEC))),
            0x36 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::D8,
            ))),
            0x3A => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::HLDEC))),
            0x3E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D8,
            ))),
            //
            0x40 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::B,
            ))),
            0x41 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::C,
            ))),
            0x42 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D,
            ))),
            0x43 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::E,
            ))),
            0x44 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::H,
            ))),
            0x45 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::L,
            ))),
            0x46 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::HLI,
            ))),
            0x47 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::A,
            ))),
            0x48 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::B,
            ))),
            0x49 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::C,
            ))),
            0x4A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D,
            ))),
            0x4B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::E,
            ))),
            0x4C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::H,
            ))),
            0x4D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::L,
            ))),
            0x4E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::HLI,
            ))),
            0x4F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::A,
            ))),
            //
            0x50 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::B,
            ))),
            0x51 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::C,
            ))),
            0x52 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D,
            ))),
            0x53 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::E,
            ))),
            0x54 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::H,
            ))),
            0x55 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::L,
            ))),
            0x56 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::HLI,
            ))),
            0x57 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::A,
            ))),
            0x58 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::B,
            ))),
            0x59 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::C,
            ))),
            0x5A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D,
            ))),
            0x5B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::E,
            ))),
            0x5C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::H,
            ))),
            0x5D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::L,
            ))),
            0x5E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::HLI,
            ))),
            0x5F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::A,
            ))),
            //
            0x60 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::B,
            ))),
            0x61 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::C,
            ))),
            0x62 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D,
            ))),
            0x63 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::E,
            ))),
            0x64 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::H,
            ))),
            0x65 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::L,
            ))),
            0x66 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::HLI,
            ))),
            0x67 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::A,
            ))),
            0x68 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::B,
            ))),
            0x69 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::C,
            ))),
            0x6A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D,
            ))),
            0x6B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::E,
            ))),
            0x6C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::H,
            ))),
            0x6D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::L,
            ))),
            0x6E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::HLI,
            ))),
            0x6F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::A,
            ))),
            //
            0x70 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::B,
            ))),
            0x71 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::C,
            ))),
            0x72 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::D,
            ))),
            0x73 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::E,
            ))),
            0x74 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::H,
            ))),
            0x75 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::L,
            ))),
            0x77 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::A,
            ))),
            0x78 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::B,
            ))),
            0x79 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::C,
            ))),
            0x7A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D,
            ))),
            0x7B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::E,
            ))),
            0x7C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::H,
            ))),
            0x7D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::L,
            ))),
            0x7E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::HLI,
            ))),
            0x7F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::A,
            ))),
            //
            0xE0 => Some(Instruction::LD(LoadType::ByteAddressFromA(ByteAddress::A8))),
            0xE2 => Some(Instruction::LD(LoadType::ByteAddressFromA(ByteAddress::C))),
            0xEA => Some(Instruction::LD(LoadType::ByteAddressFromA(
                ByteAddress::A16,
            ))),
            0xF0 => Some(Instruction::LD(LoadType::AFromByteAddress(ByteAddress::A8))),
            0xF2 => Some(Instruction::LD(LoadType::AFromByteAddress(ByteAddress::C))),
            0xF8 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::HL,
                LoadWordSource::SP,
            ))),
            0xF9 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::SP,
                LoadWordSource::HL,
            ))),
            0xFA => Some(Instruction::LD(LoadType::AFromByteAddress(
                ByteAddress::A16,
            ))),

            // Stack instructions
            0xC1 => Some(Instruction::POP(StackRegisters::BC)),
            0xC5 => Some(Instruction::PUSH(StackRegisters::BC)),
            0xD1 => Some(Instruction::POP(StackRegisters::DE)),
            0xD5 => Some(Instruction::PUSH(StackRegisters::DE)),
            0xE1 => Some(Instruction::POP(StackRegisters::HL)),
            0xE5 => Some(Instruction::PUSH(StackRegisters::HL)),
            0xF1 => Some(Instruction::POP(StackRegisters::AF)),
            0xF5 => Some(Instruction::PUSH(StackRegisters::AF)),

            // All other instruction sets are illegal
            _ => None,
        }
    }
}
