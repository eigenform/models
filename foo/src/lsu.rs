
use crate::*;
use crate::storage::*;

pub enum LSUOp {
    Store(Operand),
    Load,
}

pub enum LSUOpWidth {
    Byte,
    Half,
    Word,
}

pub struct LSUInst {
    op: LSUOp,
    addr: Operand,
    width: LSUOpWidth,
}

pub fn exec_lsu_inst(inst: LSUInst, s: &Storage, m: &mut Memory) -> Option<u32>
{
    if let Some(addr) = s.resolve_operand(inst.addr) {
        let addr = addr as usize;
        match inst.op {
            LSUOp::Load => {
                match inst.width {
                    LSUOpWidth::Byte => Some(m.load8(addr) as u32),
                    LSUOpWidth::Half => Some(m.load16(addr) as u32),
                    LSUOpWidth::Word => Some(m.load32(addr) as u32),
                }
            },
            LSUOp::Store(data) => {
                if let Some(data) = s.resolve_operand(data) {
                    match inst.width {
                        LSUOpWidth::Byte => m.store8(addr, data as u8),
                        LSUOpWidth::Half => m.store16(addr, data as u16),
                        LSUOpWidth::Word => m.store32(addr, data as u32),
                    }
                    Some(0)
                } else {
                    None
                }
            }
        }
    } else {
        None
    }

}


