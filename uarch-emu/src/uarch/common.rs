
use std::ops::{Index, IndexMut};

use std::sync::{Arc, RwLock};
use std::cell::{RefCell, RefMut, Ref};
use std::rc::Rc;

/// Type alias for shared references to resources.
pub type PipelineResource<T> = Rc<RefCell<T>>;

pub trait SimplePipelineStage {
    type In;
    type Out;

    /// Transform inputs to outputs.
    fn execute(&mut self, i: Self::In) -> Self::Out;
    fn stall(&mut self);
    fn unstall(&mut self);
}


/// Architectural register file.
#[derive(Debug)]
pub struct RegisterFile {
    data: Vec<u32>,
}
impl RegisterFile {
    pub fn new(size: usize, init: Option<&[u32]>) -> Self {
        let mut res = Self { data: vec![0u32; size] };
        if let Some(init) = init {
            res.data.copy_from_slice(init);
        }
        res
    }
}
impl Index<usize> for RegisterFile {
    type Output = u32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}
impl IndexMut<usize> for RegisterFile {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

/// Implemented on some union type that represents an ALU operation 
/// (as defined in some target instruction set).
pub trait FrontendALUOp {
    fn to_alu_op(&self) -> ALUOp;
}

/// Implemented on some union type that represents the width of a memory
/// access (as defined in some target instruction set).
pub trait FrontendAccessWidth {
    fn to_width(&self) -> Width;
}


/// An element in the instruction stream.
#[derive(Debug)]
pub struct Instruction {
    pub id: usize,
    pub op: Op,
}

/// Distinct types of instructions supported by the machine.
#[derive(Debug)]
pub enum Op {
    /// ALU operation (rD, x_value, y_value, operation)
    Alu(usize, u32, u32, ALUOp),
    /// Load operation (rD, addr, offset, width)
    Load(usize, u32, i32, Width),
    /// Store operation (value, addr, offset, width)
    Store(u32, u32, i32, Width),
}

/// Token for an effect on the state of the machine.
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

/// Arithmetic/logical operations supported by the emulated machine.
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

/// Widths of memory access operations supported by the machine.
#[derive(Debug)]
pub enum Width {
    Byte,
    Half,
    Word,
}


