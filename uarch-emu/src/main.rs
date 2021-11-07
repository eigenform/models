
pub mod rv32;
pub mod core;
pub mod uarch;
pub mod mem;


#[cfg(test)]
mod uarch_simple {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::mem::*;
    use crate::uarch::common::*;
    use crate::uarch::simple::SingleCycleMachine;

    const INITIAL_REGS: [u32; 8] = [ 0, 1, 2, 3, 4, 5, 6, 7 ];

    #[test]
    fn test() {
        let dmem = Rc::new(RefCell::new(DataMemory::new()));
        let rf = Rc::new(RefCell::new(
                RegisterFile::new(8, Some(&INITIAL_REGS))
        ));
        let mut p = SingleCycleMachine::new(
            rf.clone(), dmem.clone()
        );

        for _cycle in 0..8 {
            println!("{:08x?}", rf.borrow());
            p.step();
            println!("");
        }
    }
}



fn main() {
}


