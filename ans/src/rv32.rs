
#[repr(usize)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum RvOpcode {
    LOAD       = 0b00000, // [lb, lh, lw, lbu, lhu]
    LOAD_FP    = 0b00001,
    CUSTOM_0   = 0b00010,
    MISC_MEM   = 0b00011, // [fence, fence.i]
    OP_IMM     = 0b00100, // [addi, slti, sltiu, xori, ori, andi]
    AUIPC      = 0b00101, 
    OP_IMM_32  = 0b00110,
    STORE      = 0b01000, // [sb, sh, sw]
    STORE_FP   = 0b01001,
    CUSTOM_1   = 0b01010,
    AMO        = 0b01011,
    OP         = 0b01100, // [add, sub, sll, slt, sltu, xor, srl, sra, or, and]
    LUI        = 0b01101,
    OP_32      = 0b01110,
    MADD       = 0b10000,
    MSUB       = 0b10001,
    NMSUB      = 0b10010,
    NMADD      = 0b10011,
    OP_FP      = 0b10100,
    RES_0      = 0b10101,
    CUSTOM_2   = 0b10110,
    BRANCH     = 0b11000, // [beq, bne, blt, bge, bltu, bgeu]
    JALR       = 0b11001,
    RES_1      = 0b11010,
    JAL        = 0b11011,
    SYSTEM     = 0b11100,
    RES_2      = 0b11101,
    CUSTOM_3   = 0b11110,
}
impl From<u32> for RvOpcode {
    fn from(x: u32) -> Self {
        match x {
         0b00000 => Self::LOAD,
         0b00001 => Self::LOAD_FP,
         0b00010 => Self::CUSTOM_0,
         0b00011 => Self::MISC_MEM,
         0b00100 => Self::OP_IMM,
         0b00101 => Self::AUIPC,
         0b00110 => Self::OP_IMM_32,
         0b01000 => Self::STORE,
         0b01001 => Self::STORE_FP,
         0b01010 => Self::CUSTOM_1,
         0b01011 => Self::AMO,
         0b01100 => Self::OP,
         0b01101 => Self::LUI,
         0b01110 => Self::OP_32,
         0b10000 => Self::MADD,
         0b10001 => Self::MSUB,
         0b10010 => Self::NMSUB,
         0b10011 => Self::NMADD,
         0b10100 => Self::OP_FP,
         0b10101 => Self::RES_0,
         0b10110 => Self::CUSTOM_2,
         0b11000 => Self::BRANCH,
         0b11001 => Self::JALR,
         0b11010 => Self::RES_1,
         0b11011 => Self::JAL,
         0b11100 => Self::SYSTEM,
         0b11101 => Self::RES_2,
         0b11110 => Self::CUSTOM_3,
         _ => unimplemented!(),
        }
    }
}

pub enum RvEncodingFormat { R, I, S, U, B, J }

#[repr(transparent)]
pub struct RvEncoding(pub u32);
impl RvEncoding {
    pub fn fmt(&self) -> RvEncodingFormat {
        match self.opcode() {
            RvOpcode::OP     => RvEncodingFormat::R,
            RvOpcode::JAL    => RvEncodingFormat::J,
            RvOpcode::STORE  => RvEncodingFormat::S,
            RvOpcode::BRANCH => RvEncodingFormat::B,

            RvOpcode::AUIPC 
            | RvOpcode::LUI  => RvEncodingFormat::U,

            RvOpcode::MISC_MEM
            | RvOpcode::OP_IMM
            | RvOpcode::JALR
            | RvOpcode::SYSTEM
            | RvOpcode::LOAD => RvEncodingFormat::I,
            _ => unimplemented!(),
        }
    }
    pub fn opcode(&self) -> RvOpcode {
        RvOpcode::from(
            (self.0 & 0b0000_000_00000_00000_000_00000_11111_00) >> 2
        )
    }
    pub fn rd(&self) -> RvReg {
        RvReg(
            ((self.0 & 0b0000_000_00000_00000_000_11111_00000_00) >> 7)
            as usize
        )
    }
    pub fn f3(&self) -> u32 {
        (self.0 & 0b0000_000_00000_00000_111_00000_00000_00) >> 12
    }
    pub fn rs1(&self) -> RvReg {
        RvReg(
            ((self.0 & 0b0000_000_00000_11111_000_00000_00000_00) >> 15)
            as usize
        )
    }
    pub fn rs2(&self) -> RvReg {
        RvReg(
            ((self.0 & 0b0000_000_11111_00000_000_00000_00000_00) >> 20)
            as usize
        )
    }
    pub fn f7(&self) -> u32 {
        (self.0 & 0b1111_111_00000_00000_000_00000_00000_00) >> 25
    }
    pub fn uimm(&self) -> u32 {
        (self.0 & 0b1111_1111_1111_1111_1111_0000_0000_0000) >> 12
    }

    pub fn simm(&self) -> i32 {
        fn sext32(x: u32, bits: u32) -> i32 {
            ((x << (32 - bits)) as i32) >> (32 - bits)
        }
        match self.fmt() {
            RvEncodingFormat::I => {
                let imm = (self.0 & 0xfff0_0000) >> 20;
                let imm = sext32(imm, 12);
                imm
            },
            RvEncodingFormat::S => {
                let hi  = (self.0 & 0xfe00_0000) >> 20;
                let lo  = (self.0 & 0x0000_0f80) >> 7;
                let imm = sext32((hi | lo), 12);
                imm
            },
            RvEncodingFormat::J => { 
                let imm20    = ((self.0 & 0x8000_0000) >> 31) << 20;
                let imm10_1  = ((self.0 & 0x7fe0_0000) >> 21) << 1;
                let imm11    = ((self.0 & 0x0010_0000) >> 20) << 11;
                let imm19_12 = ((self.0 & 0x000f_f000) >> 12) << 12;
                let tmp = imm20 | imm19_12 | imm11 | imm10_1;
                println!("{:08x}", tmp);
                let imm = sext32(tmp, 21);
                println!("{:08x} ({})", imm, imm);
                imm
            },
            RvEncodingFormat::B => { 
                let imm12   = ((self.0 & 0x8000_0000) >> 31) << 12;
                let imm10_5 = ((self.0 & 0x7e00_0000) >> 25) << 5;
                let imm4_1  = ((self.0 & 0x0000_0f00) >> 8) << 1;
                let imm11   = ((self.0 & 0x0000_0080) >> 7) << 11;
                let tmp = imm12 | imm11 | imm10_5 | imm4_1;
                let imm = sext32(tmp, 12);
                imm
            },
            _ => unimplemented!(),
        }
    }
}

impl RvEncoding {
    pub fn decode(&self) -> RvInstr {
        match self.opcode() {
            RvOpcode::OP => {
                let alu_op = RvALUOp::from((self.f3(), self.f7()));
                RvInstr::Op(self.rd(), self.rs1(), self.rs2(), alu_op)
            },
            RvOpcode::OP_IMM => {
                let alu_op = RvALUOp::from((self.f3(), 0b0000000));
                RvInstr::OpImm(self.rd(), self.rs1(), self.simm(), alu_op)
            },
            RvOpcode::LOAD => {
                let w   = RvWidth::from(self.f3());
                println!("{:08x}", self.0);
                RvInstr::Load(self.rd(), self.rs1(), self.simm(), w)
            },
            RvOpcode::STORE => {
                let w   = RvWidth::from(self.f3());
                RvInstr::Store(self.rs1(), self.rs2(), self.simm(), w)
            },
            RvOpcode::JAL => {
                RvInstr::Jal(self.rd(), self.simm())
            },
            RvOpcode::LUI => {
                RvInstr::Lui(self.rd(), self.uimm())
            }
            RvOpcode::BRANCH => {
                let br_op = RvBranchOp::from(self.f3());
                RvInstr::Branch(self.rs1(), self.rs2(), self.simm(), br_op)
            }
            RvOpcode::JALR => {
                assert!(self.f3() == 0b000);
                RvInstr::Jalr(self.rd(), self.rs1(), self.simm())
            },
            _ => unimplemented!("Unimplemented opcode {:?}", self.opcode()),
        }
    }
}



#[derive(Debug, Clone, Copy)]
pub enum RvWidth {
    Byte,
    Half,
    Word,
}
impl From<u32> for RvWidth {
    fn from(x: u32) -> Self {
        match x {
            0b000 => Self::Byte,
            0b001 => Self::Half,
            0b010 => Self::Word,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RvALUOp {
    Add,
    Sub,

    Sll,
    Slt,
    Sltu,
    Xor,

    Srl,
    Sra,

    Or,
    And,
}
impl From<(u32, u32)> for RvALUOp {
    fn from(x: (u32, u32)) -> Self {
        match x {
            (0b000, 0b0000000) => Self::Add,
            (0b000, 0b0100000) => Self::Sub,

            (0b001, 0b0000000) => Self::Sll,
            (0b010, 0b0000000) => Self::Slt,
            (0b011, 0b0000000) => Self::Sltu,
            (0b100, 0b0000000) => Self::Xor,

            (0b101, 0b0000000) => Self::Srl,
            (0b101, 0b0100000) => Self::Sra,

            (0b110, 0b0000000) => Self::Or,
            (0b111, 0b0000000) => Self::And,
            _ => unimplemented!("ALU op f3={:03b} f7={:07b}", x.0, x.1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RvBranchOp {
    Eq,
    Ne,
    Lt,
    Ge,
    Ltu,
    Geu,
}
impl From<u32> for RvBranchOp {
    fn from(x: u32) -> Self {
        match x {
            0b000 => Self::Eq,
            0b001 => Self::Ne,
            0b010 => unimplemented!(),
            0b011 => unimplemented!(),
            0b100 => Self::Lt,
            0b101 => Self::Ge,
            0b110 => Self::Ltu,
            0b111 => Self::Geu,
            _ => unimplemented!(),
        }
    }
}




#[derive(Clone, Copy, Debug)]
pub struct RvReg(pub usize);


#[derive(Debug, Clone, Copy)]
pub enum RvInstr {
    /// ALU operation
    Op(RvReg, RvReg, RvReg, RvALUOp),

    /// ALU operation with immediate
    OpImm(RvReg, RvReg, i32, RvALUOp),
    /// Memory load
    Load(RvReg, RvReg, i32, RvWidth),
    /// Jump-and-link register
    Jalr(RvReg, RvReg, i32),

    /// Load upper immediate
    Lui(RvReg, u32),

    /// Memory store
    Store(RvReg, RvReg, i32, RvWidth),

    /// Jump-and-link
    Jal(RvReg, i32),

    /// Conditional branch
    Branch(RvReg, RvReg, i32, RvBranchOp),
}


