
pub enum UnaryRelation { Id, NotId }
pub enum BinaryRelation { Eq, Ne, Lt, Gt }
pub enum ArithOp { Add, Sub, Mul, Div }
pub enum LogicOp { And, Or, Xor, Ror, Rol, Lsl, Lsr, Asr }
pub enum Operation {
    Cmp1  { x: VirtReg, op: UnaryRelation },
    Move  { x: VirtReg, y: VirtReg },
    Cmp2  { x: VirtReg, y: VirtReg, op: BinaryRelation },
    Arith { x: VirtReg, y: VirtReg, op: ArithOp },
    Logic { x: VirtReg, y: VirtReg, op: LogicOp },
}

/// An effect on the [visible] state of a machine.
pub enum DataEffect {
    /// Set some variable to a particular value.
    Id { dst: VirtReg, val: u32 },
    /// Set some variable to the result of an operation.
    Op { dst: VirtReg, op: Operation },
}

/// The name of a particular [DataEffect] within some [BasicBlock].
pub struct VirtReg(usize);
impl VirtReg {
    fn get(&self) -> usize { self.0 }
}


/// The name of a particular set of data effects (a [BasicBlock]).
pub struct BasicBlockId(pub usize);

/// An atomic set of data effects on a machine.
pub struct BasicBlock {
    eff: Vec<DataEffect>,
    ctrl: ControlEffect,
}
impl Default for BasicBlock {
    fn default() -> Self {
        Self { eff: Vec::new(), ctrl: ControlEffect::Halt }
    }
}

trait InstructionStream {
    /// Obtain the next [DataEffect] in the instruction stream.
    fn next(&self) -> Option<DataEffect>;
}


/// A control effect on the machine; a change in the set of the next possible
/// data effects on the machine. Control effects *control* the order (in the
/// time domain) in which data effects occur on the machine.
///
/// Control effects determine *whether or not* some relation holds (or some 
/// condition is satisfied), and then cause the machine to undergo some 
/// set of data effects.
///
pub enum ControlEffect {
    /// Halt execution of the machine.
    Halt,

    /// Perform a particular set of data effects (a [BasicBlock]).
    BranchUncond { eff: BasicBlockId },

    /// Perform one of two sets of data effects, depending on whether or not
    /// some unary relation holds.
    Branch1Cond { 
        eff1: BasicBlockId, eff2: BasicBlockId, 
        data: VirtReg, 
        op: UnaryRelation 
    },

    /// Perform one of two sets of data effects, depending on whether or not
    /// some binary relation holds.
    Branch2Cond { 
        eff1: BasicBlockId, eff2: BasicBlockId, 
        data1: VirtReg, data2: VirtReg, 
        op: BinaryRelation
    },
}


//#[cfg(test)]
//mod test {
//    use crate::effects::*;
//    #[test]
//    fn test() {
//        let mut bb = BasicBlock::default();
//    }
//}



