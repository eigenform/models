
pub mod storage;
pub mod effects;
pub mod mem;
pub mod rv32;

#[cfg(test)]
mod tests {
    use crate::{ mem::*, rv32::* };
    use object::{Object, ObjectSection};
    use object::elf::{SHF_ALLOC};
    use std::fs;
    use std::convert::TryInto;

    fn load_elf(filename: &'static str, mem: &mut Memory) -> usize {
        let elf_data = fs::read(filename).unwrap();
        let elf = object::File::parse(&*elf_data).unwrap();
        let text_s = elf.section_by_name(".text").unwrap();
        let data_s = elf.section_by_name(".data").unwrap();

        for section in elf.sections() {
            if let object::SectionFlags::Elf { sh_flags } = section.flags() {
                if ((sh_flags as u32) & SHF_ALLOC == SHF_ALLOC) {
                    let data = section.data().unwrap();
                    let addr: usize = section.address().try_into().unwrap();
                    mem.write(addr, data);
                }
            }
        }
        elf.entry() as usize
    }

    struct Regs {
        data: [u32; 32],
    }
    impl Regs {
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

    #[test]
    fn interpreter() {
        let mut ram = Memory::new(0x0010_0000);
        let entrypt = load_elf("./rv32/test.elf", &mut ram);
        let mut pc  = entrypt as u32;
        let mut reg = Regs::new();

        loop {
            if pc == 0xdead_0000 {
                println!("Halted at pc=0xdead0000");
                break;
            }
            let inst = RvEncoding(ram.read32(pc as usize));
            let inst = inst.decode();
            println!("{:08x}: {:x?}", pc, inst);
            match inst {
                RvInstr::Op(rd, rs1, rs2, op) => {
                    let rs1 = reg.read(rs1);
                    let rs2 = reg.read(rs2);
                    let res = match op {
                        RvALUOp::Add => rs1.wrapping_add(rs2),
                        RvALUOp::Sll => rs1.checked_shl(rs2).unwrap_or(0),
                        _ => unimplemented!("{:?}", op),
                    };
                    reg.write(rd, res);
                },
                RvInstr::OpImm(rd, rs1, imm, op) => {
                    let rs1 = reg.read(rs1);
                    let imm = imm as u32;
                    let res = match op {
                        RvALUOp::Add => rs1.wrapping_add(imm),
                        RvALUOp::Sll => rs1.checked_shl(imm).unwrap_or(0),
                        _ => unimplemented!("{:?}", op),
                    };
                    reg.write(rd, res);
                }
                RvInstr::Store(rs1, rs2, imm, width) => {
                    let val  = reg.read(rs2);
                    let addr = reg.read(rs1).wrapping_add(imm as u32) as usize;
                    match width {
                        RvWidth::Byte => ram.store8(addr, val as u8),
                        RvWidth::Half => ram.store16(addr, val as u16),
                        RvWidth::Word => ram.store32(addr, val),
                    }
                },
                RvInstr::Load(rd, rs1, imm, width) => {
                    let addr = reg.read(rs1).wrapping_add(imm as u32) as usize;
                    let res  = match width {
                        RvWidth::Byte => ram.load8(addr)  as u32,
                        RvWidth::Half => ram.load16(addr) as u32,
                        RvWidth::Word => ram.load32(addr) as u32,
                    };
                    reg.write(rd, res);
                },
                RvInstr::Jal(rd, imm) => {
                    let next_pc = pc.wrapping_add(imm as u32);
                    if rd.0 != 0 {
                        reg.write(rd, pc.wrapping_add(4));
                    }
                    pc = next_pc;
                    continue;
                },
                RvInstr::Branch(rs1, rs2, imm, op) => {
                    let rs1 = reg.read(rs1);
                    let rs2 = reg.read(rs2);
                    let res = match op {
                        RvBranchOp::Eq  => rs1 == rs2,
                        RvBranchOp::Ne  => rs1 != rs2,
                        RvBranchOp::Lt  => (rs1 as i32) < (rs2 as i32),
                        RvBranchOp::Ge  => (rs1 as i32) >= (rs2 as i32),
                        RvBranchOp::Ltu => rs1 < rs2,
                        RvBranchOp::Geu => rs1 >= rs2
                    };
                    if res {
                        pc = pc.wrapping_add(imm as u32);
                        continue;
                    }
                }
                RvInstr::Lui(rd, imm) => {
                    reg.write(rd, imm);
                }
                RvInstr::Jalr(rd, rs1, imm) => {
                    let next_pc = reg.read(rs1)
                        .wrapping_add(imm as u32) & 0xffff_fffe;
                    if rd.0 != 0 {
                        reg.write(rd, pc.wrapping_add(4));
                    }
                    pc = next_pc;
                    continue;
                }
                _ => unimplemented!("{:x?}", inst),
            }
            pc += 4;
        }
    }
}


