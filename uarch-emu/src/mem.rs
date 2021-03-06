
/// Simple emulated memory device.
pub struct DataMemory {
    pub data: [u8; 0x0001_0000],
}
impl DataMemory {
    pub fn new() -> Self {
        let mut res = Self {
            data: [0; 0x0001_0000],
        };
        for i in 0..0x0001_0000 {
            res.data[i] = rand::random();
        }
        res
    }
    pub fn load8(&self, addr: usize) -> u8 {
        self.data[addr]
    }
    pub fn load16(&self, addr: usize) -> u16 {
        use std::convert::TryInto;
        let slice: [u8; 2] = self.data[addr..addr + 2].try_into().unwrap();
        let res = u16::from_ne_bytes(slice);
        res
    }
    pub fn load32(&self, addr: usize) -> u32 {
        use std::convert::TryInto;
        let slice: [u8; 4] = self.data[addr..addr + 4].try_into().unwrap();
        let res = u32::from_ne_bytes(slice);
        res
    }

    pub fn store8(&mut self, addr: usize, val: u8) {
        self.data[addr] = val;
    }
    pub fn store16(&mut self, addr: usize, val: u16) {
        self.data[addr..addr + 2].copy_from_slice(&val.to_le_bytes());
    }
    pub fn store32(&mut self, addr: usize, val: u32) {
        self.data[addr..addr + 4].copy_from_slice(&val.to_le_bytes());
    }
}


