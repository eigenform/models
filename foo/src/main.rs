
pub mod alu;
pub mod lsu;
pub mod storage;
pub mod traits;

/// Representing the state of some architectural register.
#[derive(Clone, Copy)]
pub enum ArchValue {
    /// The value in this register is valid.
    Valid(u32),
    /// This register is mapped to the name of a physical register.
    Name(usize),
}

/// Representing the state of some physical register.
#[derive(Clone, Copy)]
pub enum PhysValue {
    /// The value in this physical register is valid.
    Valid(u32),
    /// This physical register is empty.
    Invalid,
}

/// Different kinds of input data to an [Inst].
pub enum Operand {
    /// The name of an architectural register.
    ArchReg(usize),
    /// An immediate value encoded directly within an instruction.
    Imm(u32),
}



fn main() {
}



