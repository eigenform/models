
use crate::*;
use crate::storage::*;

/// Set of different arithmetic/logical operations.
pub enum ALUOp {
    Add, 
    Sub, 
    And, 
    Or, 
    Xor, 
    Ror, 
    Rol,
    Lsl,
    Lsr,
    Asr,
    Lt,
    Ltu,
}

/// An arithmetic/logical instruction.
pub struct ALUInst {
    op: ALUOp,
    x: Operand,
    y: Operand,
}

/// Perform an ALU instruction.
///
/// Using a reference to some [Storage], try to resolve the concrete values of 
/// the operands and compute the resulting value.
pub fn exec_alu_inst(inst: ALUInst, s: &Storage) -> Option<u32> {
    use std::ops::{BitOr, BitAnd, BitXor};
    let (x, y) = ( s.resolve_operand(inst.x), s.resolve_operand(inst.y) );
    if x.is_some() && y.is_some() {
        let x = x.unwrap();
        let y = y.unwrap();
        let res = match inst.op {
            ALUOp::Add => x.wrapping_add(y),
            ALUOp::Sub => x.wrapping_sub(y),
            ALUOp::And => x.bitand(y),
            ALUOp::Or  => x.bitor(y),
            ALUOp::Xor => x.bitxor(y),
            ALUOp::Ror => x.rotate_right(y),
            ALUOp::Rol => x.rotate_left(y),
            ALUOp::Lsl => x.wrapping_shl(y),
            ALUOp::Lsr => x.wrapping_shr(y),
            ALUOp::Asr => (x as i32).wrapping_shr(y) as u32,
            ALUOp::Lt  => ((x as i32) < (y as i32)) as u32,
            ALUOp::Ltu => ((x < y)) as u32
        };
        Some(res)
    } else {
        None
    }
}


