//! Simple single-cycle machine.

use std::ops::{BitAnd, BitOr, BitXor};

use crate::rv32::*;
use crate::uarch::common::*;
use crate::mem::*;

pub struct SingleCycleMachine {
    pub cyc: usize,
    pub pc: u32,
    if_stage: FetchStage,
    id_stage: DecodeStage,
    ex_stage: ExecutionStage,
    me_stage: MemoryStage,
    wb_stage: WritebackStage,
}
impl SingleCycleMachine {
    pub fn new(rf: PipelineResource<RegisterFile>,
               dmem: PipelineResource<DataMemory>) -> Self {
        Self {
            cyc: 0,
            pc: 0,
            if_stage: FetchStage::new(),
            id_stage: DecodeStage::new(rf.clone()),
            ex_stage: ExecutionStage::new(),
            me_stage: MemoryStage::new(dmem.clone()),
            wb_stage: WritebackStage::new(rf.clone()),
        }
    }
    pub fn step(&mut self) {

        println!("[IF] {:08x?}", self.pc);
        let f = self.if_stage.execute(self.pc);

        println!("[ID] {:x?}", &f);
        let d = self.id_stage.execute(f);

        println!("[EX] {:x?}", &d);
        let x = self.ex_stage.execute(d);

        println!("[ME] {:x?}", &x);
        let m = self.me_stage.execute(x);

        println!("[WB] {:x?}", &m);
        let w = self.wb_stage.execute(m);

        self.pc += 4;
        self.cyc += 1;
    }
}



pub struct FetchStage {
    stall: bool,
}
impl FetchStage {
    pub fn new() -> Self { Self { stall: false, } }
}
impl SimplePipelineStage for FetchStage {
    type In  = u32;
    type Out = RvInstr;
    fn execute(&mut self, i: Self::In) -> Self::Out {
        rand::random()
    }
    fn stall(&mut self) { self.stall = true }
    fn unstall(&mut self) { self.stall = false }
}

pub struct DecodeStage {
    stall: bool,
    next_id: usize,
    rf: PipelineResource<RegisterFile>,
}
impl DecodeStage {
    pub fn new(rf: PipelineResource<RegisterFile>) -> Self { 
        Self { stall: false, next_id: 0, rf } 
    }
}
impl SimplePipelineStage for DecodeStage {
    type In  = RvInstr;
    type Out = Instruction;
    fn execute(&mut self, i: Self::In) -> Self::Out {
        let rf = self.rf.borrow();
        let op = match i {
            RvInstr::Op(rd, rs1, rs2, op) => 
                Op::Alu(rd.0, rf[rs1.0], rf[rs2.0], op.to_alu_op()),
            RvInstr::OpImm(rd, rs1, imm, op) => 
                Op::Alu(rd.0, rf[rs1.0], imm as u32, op.to_alu_op()),
            RvInstr::Lui(rd, imm) => 
                Op::Alu(rd.0, 0, imm << 12, ALUOp::Add),
            RvInstr::Load(rd, rs1, imm, w) => 
                Op::Load(rd.0, rf[rs1.0], imm, w.to_width()),
            RvInstr::Store(rs1, rs2, imm, w) => 
                Op::Store(rf[rs2.0], rf[rs1.0], imm, w.to_width()),
            _ => unimplemented!(),
        };

        let id = self.next_id;
        self.next_id += 1;
        Instruction { op, id }
    }
    fn stall(&mut self) { self.stall = true }
    fn unstall(&mut self) { self.stall = false }

}

pub struct ExecutionStage {
    stall: bool,
}
impl ExecutionStage {
    pub fn new() -> Self { Self { stall: false } }
}
impl SimplePipelineStage for ExecutionStage {
    type In  = Instruction;
    type Out = Effect;
    fn execute(&mut self, i: Self::In) -> Self::Out {
        match i.op {
            Op::Alu(rd, x, y, aluop) => {
                let res = match aluop {
                    ALUOp::Add => x.wrapping_add(y),
                    ALUOp::Sub => x.wrapping_sub(y),
                    ALUOp::LtSigned => {
                        if (x as i32) < (y as i32) { 1 } else { 0 }
                    }
                    ALUOp::LtUnsigned => {
                        if x < y { 1 } else { 0 }
                    }
                    ALUOp::Sll => x.checked_shl(y).unwrap_or(0),
                    ALUOp::Srl => x.checked_shr(y).unwrap_or(0),
                    ALUOp::Sra => x.checked_shr(y).unwrap_or(0),
                    ALUOp::Xor => x.bitxor(y),
                    ALUOp::Or => x.bitor(y),
                    ALUOp::And => x.bitand(y),
                };
                Effect::RegWrite(rd, res)
            }
            Op::Load(rd, base, off, w) => {
                let addr = base.wrapping_add(off as u32);
                Effect::MemLoad(rd, addr, w)
            }
            Op::Store(val, base, off, w) => {
                let addr = base.wrapping_add(off as u32);
                Effect::MemStore(val, addr, w)
            }
            _ => unimplemented!(),
        }
    }
    fn stall(&mut self) { self.stall = true }
    fn unstall(&mut self) { self.stall = false }

}

pub struct MemoryStage {
    stall: bool,
    dmem: PipelineResource<DataMemory>,
}
impl MemoryStage {
    pub fn new(dmem: PipelineResource<DataMemory>) -> Self { 
        Self { stall: false, dmem } 
    }
}
impl SimplePipelineStage for MemoryStage {
    type In = Effect;
    type Out = Effect;
    fn execute(&mut self, i: Self::In) -> Self::Out {
        let mut dmem = self.dmem.borrow_mut();
        match i {
            Effect::MemLoad(rd, addr, w) => {
                let addr = (addr & 0x0000_fffc) as usize;
                let res = match w {
                    Width::Byte => dmem.load8(addr as usize) as u32,
                    Width::Half => dmem.load16(addr as usize) as u32,
                    Width::Word => dmem.load32(addr as usize) as u32,
                };
                Effect::RegWrite(rd, res)
            }
            Effect::MemStore(val, addr, w) => {
                let addr = (addr & 0x0000_fffc) as usize;
                let res = match w {
                    Width::Byte => 
                        dmem.store8(addr as usize, val as u8),
                    Width::Half => 
                        dmem.store16(addr as usize, val as u16),
                    Width::Word => 
                        dmem.store32(addr as usize, val),
                };
                Effect::None
            }
            _ => i,
        }
    }
    fn stall(&mut self) { self.stall = true }
    fn unstall(&mut self) { self.stall = false }

}


pub struct WritebackStage {
    stall: bool,
    rf: PipelineResource<RegisterFile>,
}
impl WritebackStage {
    pub fn new(rf: PipelineResource<RegisterFile>) -> Self { 
        Self { stall: false, rf } 
    }
}
impl SimplePipelineStage for WritebackStage {
    type In = Effect;
    type Out = ();
    fn execute(&mut self, i: Self::In) -> Self::Out {
        let mut rf = self.rf.borrow_mut();
        match i {
            Effect::RegWrite(rd, val) => {
                rf[rd] = val;
            }
            Effect::None => {}
            _ => unimplemented!("{:?}", i),
        }
    }
    fn stall(&mut self) { self.stall = true }
    fn unstall(&mut self) { self.stall = false }
}


