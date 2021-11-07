
pub trait VirtualStorage {
    fn resolve(&self, idx: usize) -> Option<u32>;
    fn allocate(&mut self) -> Option<usize>;
    fn write(&mut self, idx: usize, data: u32);
    fn free(&mut self, idx: usize);
}

pub trait ArchitecturalStorage {
    type Backing: VirtualStorage;
    fn resolve(&self, idx: usize, s: &Self::Backing) -> Option<u32>;
    fn rename(&mut self, idx: usize, name: usize);
    fn write(&mut self, idx: usize, data: u32);
}



#[cfg(test)]
mod test {
    use crate::storage::*;

    #[derive(Copy, Clone)]
    pub enum ArchRegValue { Valid(u32), Name(usize) }
    pub struct ArchitecturalRegisters {
        data: [ArchRegValue; 32],
    }
    impl ArchitecturalRegisters {
        pub fn new() -> Self { 
            Self { data: [ArchRegValue::Valid(0); 32] } 
        }
    }
    impl ArchitecturalStorage for ArchitecturalRegisters {
        type Backing = SSAVirtualRegisters;
        fn resolve(&self, idx: usize, s: &SSAVirtualRegisters) -> Option<u32> {
            match self.data[idx] {
                ArchRegValue::Valid(x) => Some(x),
                ArchRegValue::Name(n) => s.resolve(n),
            }
        }
        fn rename(&mut self, idx: usize, name: usize) {
            self.data[idx] = ArchRegValue::Name(name);
        }
        fn write(&mut self, idx: usize, data: u32) {
            self.data[idx] = ArchRegValue::Valid(data);
        }
    }

    pub struct SSAVirtualRegisters {
        data: Vec<Option<u32>>,
    }
    impl SSAVirtualRegisters {
        pub fn new() -> Self { 
            Self { data: Vec::new() } 
        }
    }
    impl VirtualStorage for SSAVirtualRegisters {
        fn resolve(&self, idx: usize) -> Option<u32> {
            self.data[idx]
        }
        fn allocate(&mut self) -> Option<usize> {
            self.data.push(None);
            Some(self.data.len())
        }
        fn free(&mut self, idx: usize) {
            self.data[idx] = None;
        }
        fn write(&mut self, idx: usize, data: u32) {
            assert!(self.data[idx].is_none());
            self.data[idx] = Some(data);
        }
    }

    #[derive(Clone, Copy)]
    pub enum ROBEntry { InFlight, Valid(u32), Free }
    pub struct ReorderBuffer {
        data: Vec<ROBEntry>,
        size: usize,
        head: usize,
        tail: usize,
    }
    impl ReorderBuffer {
        pub fn new(size: usize) -> Self { 
            assert!(size % 2 == 0);
            Self { 
                size, 
                head: 0,
                tail: 0,
                data: vec![ROBEntry::Free; size] 
            }
        }
    }
    impl VirtualStorage for ReorderBuffer {
        fn resolve(&self, idx: usize) -> Option<u32> {
            match self.data[idx] {
                ROBEntry::Valid(data) => Some(data),
                _ => None,
            }
        }
        fn allocate(&mut self) -> Option<usize> {
            let head = self.head;
            match self.data[head] {
                ROBEntry::Free => {
                    self.data[head] = ROBEntry::InFlight;
                    self.head = (self.head + 1) % self.size;
                    Some(head)
                },
                _ => None,
            }
        }
        fn free(&mut self, idx: usize) {
            self.data[idx] = ROBEntry::Free;
        }
        fn write(&mut self, idx: usize, data: u32) {
            match self.data[idx] {
                ROBEntry::InFlight => self.data[idx] = ROBEntry::Valid(data),
                _ => panic!(""),
            }
        }
    }


    #[test]
    fn rob_full() {
        let mut rob = ReorderBuffer::new(32);
        for i in 0..32 {
            assert!(rob.allocate().is_some());
        }
        assert!(rob.allocate().is_none());
    }

}



