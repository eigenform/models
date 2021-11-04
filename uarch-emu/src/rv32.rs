
use crate::uarch::common;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

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
