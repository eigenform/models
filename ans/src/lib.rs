
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

    #[test]
    fn test() {
        let elf_data = fs::read("./rv32/test.elf").unwrap();
        let elf = object::File::parse(&*elf_data).unwrap();

        let text_s = elf.section_by_name(".text").unwrap();
        let data_s = elf.section_by_name(".data").unwrap();

        // Allocate 1MiB for physical memory
        let mut ram = Memory::new(0x0010_0000);

        let entry = elf.entry() as usize;
        for section in elf.sections() {
            if let object::SectionFlags::Elf { sh_flags } = section.flags() {
                if ((sh_flags as u32) & SHF_ALLOC == SHF_ALLOC) {
                    let data = section.data().unwrap();
                    let addr: usize = section.address().try_into().unwrap();
                    ram.write(addr, data);
                }
            }
        }

        let mut pc = entry;
        for i in 0..24 {
            let inst = RvEncoding(ram.read32(pc));
            inst.decode();
            pc += 4;
        }

    }
}


