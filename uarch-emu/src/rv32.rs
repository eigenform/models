
use crate::core;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum RvWidth {
    Byte,
    Half,
    Word,
}
impl core::FrontendAccessWidth for RvWidth {
    fn to_width(&self) -> core::Width {
        match self {
            Self::Byte => core::Width::Byte,
            Self::Half => core::Width::Half,
            Self::Word => core::Width::Word,
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
impl core::FrontendALUOp for RvALUOp {
    fn to_alu_op(&self) -> core::ALUOp {
        match self {
            Self::Add => core::ALUOp::Add,
            Self::Sub => core::ALUOp::Sub,
            Self::Xor => core::ALUOp::Xor,
            Self::Or => core::ALUOp::Or,
            Self::And => core::ALUOp::And,
            Self::Slt => core::ALUOp::LtSigned,
            Self::Sltu => core::ALUOp::LtUnsigned,
            Self::Sll => core::ALUOp::Sll,
            Self::Srl => core::ALUOp::Srl,
            Self::Sra => core::ALUOp::Sra,
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
