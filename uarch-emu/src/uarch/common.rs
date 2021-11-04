
use std::ops::{Index, IndexMut};

use std::sync::{Arc, RwLock};
use std::cell::{RefCell, RefMut, Ref};
use std::rc::Rc;

pub type PipelineResource<T> = Rc<RefCell<T>>;


/// Architectural register file.
#[derive(Debug)]
pub struct RegisterFile {
    data: [u32; 8],
}
impl RegisterFile {
    pub fn new() -> Self {
        let mut res = Self {
            data: [0; 8],
        };
        for i in 0..8 {
            res.data[i] = i as u32;
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


