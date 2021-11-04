
pub mod rv32;
pub mod core;
pub mod uarch;
pub mod mem;

use crate::rv32::*;
use crate::core::*;
use crate::mem::*;

use crate::uarch::simple::*;
use crate::uarch::common::*;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut pc = 0x0000_0000u32;

    let rf = Rc::new(RefCell::new(RegisterFile::new()));
    let dmem = Rc::new(RefCell::new(DataMemory::new()));

    let mut if_stage = FetchStage::new();
    let mut id_stage = DecodeStage::new(rf.clone());
    let mut ex_stage = ExecutionStage::new();
    let mut me_stage = MemoryStage::new(dmem.clone());
    let mut wb_stage = WritebackStage::new(rf.clone());

    for cycle in 0..32 {
        wb_stage.execute(
            me_stage.execute(
                ex_stage.execute(
                    id_stage.execute(
                        if_stage.execute(pc)
                    )
                )
            )
        );

        println!("{:08x}: {:08x?}", pc, rf);
        pc += 4;
    }
}




