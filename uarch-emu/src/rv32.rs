
use crate::uarch::common;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[repr(usize)]
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


#[repr(transparent)]
struct RvEncoding(pub u32);
impl RvEncoding {
    fn opcode(&self) -> RvOpcode {
        RvOpcode::from(
            (self.0 & 0b0000_000_00000_00000_000_00000_11111_00) >> 2
        )
    }
    fn rd(&self) -> RvReg {
        RvReg(
            ((self.0 & 0b0000_000_00000_00000_000_11111_00000_00) >> 7)
            as usize
        )
    }
    fn f3(&self) -> usize {
        ((self.0 & 0b0000_000_00000_00000_111_00000_00000_00) >> 12)
            as usize
    }
    fn rs1(&self) -> RvReg {
        RvReg(
            ((self.0 & 0b0000_000_00000_11111_000_00000_00000_00) >> 15)
            as usize
        )
    }
    fn rs2(&self) -> RvReg {
        RvReg(
            ((self.0 & 0b0000_000_11111_00000_000_00000_00000_00) >> 20)
            as usize
        )
    }
    fn f7(&self) -> usize {
        ((self.0 & 0b1111_111_00000_00000_000_00000_00000_00) >> 25)
            as usize
    }
}



#[derive(Debug, Clone, Copy)]
pub enum RvWidth {
    Byte,
    Half,
    Word,
}
impl common::FrontendAccessWidth for RvWidth {
    fn to_width(&self) -> common::Width {
        match self {
            Self::Byte => common::Width::Byte,
            Self::Half => common::Width::Half,
            Self::Word => common::Width::Word,
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
impl common::FrontendALUOp for RvALUOp {
    fn to_alu_op(&self) -> common::ALUOp {
        match self {
            Self::Add => common::ALUOp::Add,
            Self::Sub => common::ALUOp::Sub,
            Self::Xor => common::ALUOp::Xor,
            Self::Or => common::ALUOp::Or,
            Self::And => common::ALUOp::And,
            Self::Slt => common::ALUOp::LtSigned,
            Self::Sltu => common::ALUOp::LtUnsigned,
            Self::Sll => common::ALUOp::Sll,
            Self::Srl => common::ALUOp::Srl,
            Self::Sra => common::ALUOp::Sra,
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
    /// Load upper immediate
    Lui(RvReg, u32),
    /// Memory load
    Load(RvReg, RvReg, i32, RvWidth),
    /// Memory store
    Store(RvReg, RvReg, i32, RvWidth),
}

impl Distribution<RvWidth> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RvWidth {
        match rng.gen_range(0..=2) {
            0 => RvWidth::Byte,
            1 => RvWidth::Half,
            _ => RvWidth::Word,
        }
    }
}

impl Distribution<RvALUOp> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RvALUOp {
        match rng.gen_range(0..=9) {
            0 => RvALUOp::Add,
            1 => RvALUOp::Sub,
            2 => RvALUOp::Sll,
            3 => RvALUOp::Slt,
            4 => RvALUOp::Sltu,
            5 => RvALUOp::Xor,
            6 => RvALUOp::Srl,
            7 => RvALUOp::Sra,
            8 => RvALUOp::Or,
            _ => RvALUOp::And,
        }
    }
}

impl Distribution<RvInstr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RvInstr {
        let rd  = RvReg(rng.gen_range(1..=7));
        let rs1 = RvReg(rng.gen_range(0..=7));
        let rs2 = RvReg(rng.gen_range(0..=7));

        let mut op: RvALUOp = rng.gen();

        match rng.gen_range(0..4) {
            0 => {
                let imm = match op {
                    RvALUOp::Sll 
                    | RvALUOp::Srl 
                    | RvALUOp::Sra => rng.gen_range(0..32),
                    _ => rng.gen_range(-0xfff..=0xfff),
                };
                if matches!(op, RvALUOp::Sub) {
                    op = RvALUOp::Add;
                }
                RvInstr::OpImm(rd, rs1, imm, op)
            }
            1 => RvInstr::Op(rd, rs1, rs2, op),
            2 => {
                let imm = rng.gen_range(-0xfff..=0xfff);
                let w = rng.gen();
                RvInstr::Load(rd, rs1, imm, w)
            }
            _ => RvInstr::Lui(rd, rng.gen_range(0x0000_0000..=0x000f_ffff)),
        }
    }
}
