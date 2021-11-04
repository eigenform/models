

/// An arithmetic or logical operation.
#[derive(Debug)]
pub enum ALUOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Sll,
    Srl,
    Sra,
    LtUnsigned,
    LtSigned,
}

pub trait FrontendALUOp {
    fn to_alu_op(&self) -> ALUOp;
}
pub trait FrontendAccessWidth {
    fn to_width(&self) -> Width;
}

/// Supported widths of memory access operations in the machine.
#[derive(Debug)]
pub enum Width {
    Byte,
    Half,
    Word,
}

/// Supported operations in the machine.
#[derive(Debug)]
pub enum Op {
    /// ALU operation (rD, x_value, y_value, operation)
    Alu(usize, u32, u32, ALUOp),
    /// Load operation (rD, addr, offset, width)
    Load(usize, u32, i32, Width),
    /// Store operation (value, addr, offset, width)
    Store(u32, u32, i32, Width),
}

/// Describing an effect on the state of the machine.
#[derive(Debug)]
pub enum Effect {
    /// Empty effect (no-op)
    None,
    /// Write an architectural register (rD, value).
    RegWrite(usize, u32),
    /// Load from memory to an architectural register (rD, addr, width).
    MemLoad(usize, u32, Width),
    /// Store a value to memory (value, addr, width).
    MemStore(u32, u32, Width),
}


