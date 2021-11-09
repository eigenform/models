
use crate::{ mem::*, rv32::* };
use object::{Object, ObjectSection};
use object::elf::SHF_ALLOC;
use std::fs;
use std::convert::TryInto;

pub struct RvRegs { data: [u32; 32] }
impl RvRegs {
    pub fn new() -> Self {
        let mut res = Self { data: [0; 32] };
        res.data[1] = 0xdead_0000;
        res.data[2] = 0x000f_0000;
        res
    }
    pub fn read(&self, idx: RvReg) -> u32 {
        if idx.0 == 0 { 0 } else { self.data[idx.0] }
    }
    pub fn write(&mut self, idx: RvReg, val: u32) {
        assert!(idx.0 != 0);
        self.data[idx.0] = val;
    }
}

pub enum StepResult {
    /// Increment the program counter
    Next,
    /// Write the program counter
    Goto(u32),
    /// Terminate the machine
    Terminate,
}

/// Simple interpreting-style evaluator/virtual machine for RV32I programs.
///
///
pub struct Interpreter {
    /// Program counter
    pc:  u32,
    /// Register file
    reg: RvRegs,
    /// Simple emulated memory device
    ram: Memory,
}
impl Interpreter {

    pub fn new() -> Self {
        Self { 
            pc:  0,
            reg: RvRegs::new(),
            ram: Memory::new(0x0010_0000),
        }
    }

    pub fn load_elf(&mut self, filename: &'static str) {
        let elf_data = fs::read(filename).unwrap();
        let elf = object::File::parse(&*elf_data).unwrap();
        // NOTE: This is fine, for now
        for section in elf.sections() {
            if let object::SectionFlags::Elf { sh_flags } = section.flags() {
                if ((sh_flags as u32) & SHF_ALLOC == SHF_ALLOC) {
                    let data = section.data().unwrap();
                    let addr: usize = section.address().try_into().unwrap();
                    self.ram.write(addr, data);
                }
            }
        }
        self.pc = elf.entry() as u32;
    }

    /// Evaluate the result of some ALU operation
    fn eval_alu_op(x: u32, y: u32, op: RvALUOp) -> u32 {
        use std::ops::{BitOr, BitAnd, BitXor};
        match op {
            RvALUOp::Add  => x.wrapping_add(y),
            RvALUOp::Sub  => x.wrapping_sub(y),
            RvALUOp::And  => x.bitand(y),
            RvALUOp::Or   => x.bitor(y),
            RvALUOp::Xor  => x.bitxor(y),
            RvALUOp::Sll  => x.checked_shl(y).unwrap_or(0),
            RvALUOp::Srl  => x.checked_shr(y).unwrap_or(0),
            RvALUOp::Slt  => if (x as i32) < (y as i32) { 1 } else { 0 },
            RvALUOp::Sltu => if x < y { 1 } else { 0 },
            _ => unimplemented!("{:?}", op),
        }
    }

    /// Evaluate some condition
    fn eval_branch_op(x: u32, y: u32, op: RvBranchOp) -> bool {
        match op {
            RvBranchOp::Eq  => x == y,
            RvBranchOp::Ne  => x != y,
            RvBranchOp::Lt  => (x as i32) < (y as i32),
            RvBranchOp::Ge  => (x as i32) >= (y as i32),
            RvBranchOp::Ltu => x < y,
            RvBranchOp::Geu => x >= y,
        }
    }


    /// Fetch and execute the instruction at the address specified by the 
    /// program counter, returning a [StepResult].
    pub fn step(&mut self) -> StepResult {
        let inst = RvEncoding(self.ram.read32(self.pc as usize)).decode();
        println!("{:08x}: {:x?}", self.pc, inst);

        match inst {
            RvInstr::Op(rd, rs1, rs2, op) => {
                self.reg.write(rd, 
                    Self::eval_alu_op(
                        self.reg.read(rs1), 
                        self.reg.read(rs2), op)
                );
                StepResult::Next
            },
            RvInstr::OpImm(rd, rs1, imm, op) => {
                self.reg.write(rd, 
                    Self::eval_alu_op( self.reg.read(rs1), imm as u32, op)
                );
                StepResult::Next
            }
            RvInstr::Store(rs1, rs2, imm, width) => {
                let val  = self.reg.read(rs2);
                let addr = self.reg.read(rs1).wrapping_add(imm as u32) as usize;
                match width {
                    RvWidth::Byte => self.ram.store8(addr, val as u8),
                    RvWidth::Half => self.ram.store16(addr, val as u16),
                    RvWidth::Word => self.ram.store32(addr, val),
                }
                StepResult::Next
            },
            RvInstr::Load(rd, rs1, imm, width) => {
                let addr = self.reg.read(rs1).wrapping_add(imm as u32) as usize;
                let res  = match width {
                    RvWidth::Byte => self.ram.load8(addr)  as u32,
                    RvWidth::Half => self.ram.load16(addr) as u32,
                    RvWidth::Word => self.ram.load32(addr) as u32,
                };
                self.reg.write(rd, res);
                StepResult::Next
            },
            RvInstr::Jal(rd, imm) => {
                if rd.0 != 0 {
                    self.reg.write(rd, self.pc.wrapping_add(4));
                }
                StepResult::Goto(self.pc.wrapping_add(imm as u32))
            },
            RvInstr::Branch(rs1, rs2, imm, op) => {
                let res = Self::eval_branch_op(
                    self.reg.read(rs1), self.reg.read(rs2), op);
                if res {
                    StepResult::Goto(self.pc.wrapping_add(imm as u32))
                } else {
                    StepResult::Next
                }
            }
            RvInstr::Lui(rd, imm) => {
                self.reg.write(rd, imm);
                StepResult::Next
            }
            RvInstr::Jalr(rd, rs1, imm) => {
                if rd.0 != 0 {
                    self.reg.write(rd, self.pc.wrapping_add(4));
                }
                StepResult::Goto(
                    self.reg.read(rs1).wrapping_add(imm as u32) & 0xffff_fffe
                )
            }
            _ => unimplemented!("{:x?}", inst),
        }
    }

    /// Run the machine indefinitely until it halts.
    pub fn run(&mut self) {
        loop {
            if self.pc == 0xdead_0000 { break; }
            let res = self.step();
            match res {
                StepResult::Next      => self.pc = self.pc.wrapping_add(4),
                StepResult::Goto(pc)  => self.pc = pc,
                StepResult::Terminate => break,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::interp::*;
    #[test]
    fn test() {
        let mut vm = Interpreter::new();
        vm.load_elf("./rv32/test.elf");
        vm.run();
    }
}
