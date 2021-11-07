
pub mod storage;
pub mod effects;

#[cfg(test)]
mod tests {
    use object::{Object, ObjectSection};
    use object::elf::{SHF_ALLOC};
    use std::fs;

    #[test]
    fn test() {
        let elf_data = fs::read("./rv32/test.elf").unwrap();
        let elf = object::File::parse(&*elf_data).unwrap();

        let text_s = elf.section_by_name(".text").unwrap();
        let data_s = elf.section_by_name(".data").unwrap();

        // Allocate 1MiB for physical memory
        let mut ram = vec![0u8; 0x0010_0000];

        let entry = elf.entry() as usize;
        for section in elf.sections() {
            if let object::SectionFlags::Elf { sh_flags } = section.flags() {
                if ((sh_flags as u32) & SHF_ALLOC == SHF_ALLOC) {
                    let base_off = section.address() as usize;
                    let data     = section.data().unwrap();
                    let tail_off = base_off + data.len();
                    ram[base_off..tail_off].copy_from_slice(data);
                    println!("{:x?}", section);
                }
            }
        }

    }
}


