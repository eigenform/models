use crate::*;

pub struct Memory {
    pub data: Vec<u8>,
}
impl Memory {
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

pub struct ArchitecturalStorage {
    pub data: Vec<ArchValue>,
}
impl ArchitecturalStorage {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![ArchValue::Valid(0); size]
        }
    }
    /// Try to resolve the value of an architectural register.
    fn resolve(&self, idx: usize, p: &PhysicalStorage) -> Option<u32> {
        match self.data[idx] {
            // Return the value in the architectural register.
            ArchValue::Valid(data) => Some(data),
            // Try to resolve the value of a physical register.
            ArchValue::Name(name) => p.resolve(name),
        }
    }
}

pub struct PhysicalStorage {
    pub data: Vec<PhysValue>,
}
impl PhysicalStorage {
    pub fn new() -> Self {
        Self {
            data: Vec::new()
        }
    }
    /// Try to resolve the value of a physical register.
    fn resolve(&self, idx: usize) -> Option<u32> {
        match self.data[idx] {
            // Return the value in the physical register.
            PhysValue::Valid(data) => Some(data),
            // This value has not been computed, or is otherwise invalid.
            PhysValue::Invalid => None,
        }
    }
}


/// Some set of registers for storing values in the machine.
pub struct Storage {
    /// A set of "architectural" registers.
    pub arch: Vec<ArchValue>,
    /// A set of "physical" registers.
    pub phys: Vec<PhysValue>,
}
impl Storage {
    pub fn new(rf_size: usize) -> Self {
        Self {
            arch: vec![ArchValue::Valid(0); rf_size],
            phys: Vec::new(),
        }
    }
}

impl Storage {
    /// Try to resolve the value of a physical register.
    fn resolve_phys_value(&self, idx: usize) -> Option<u32> {
        match self.phys[idx] {
            // Return the value in the physical register.
            PhysValue::Valid(data) => Some(data),
            // This value has not been computed, or is otherwise invalid.
            PhysValue::Invalid => None,
        }
    }
    /// Try to resolve the value of an architectural register.
    fn resolve_arch_value(&self, idx: usize) -> Option<u32> {
        match self.arch[idx] {
            // Return the value in the architectural register.
            ArchValue::Valid(data) => Some(data),
            // Try to resolve the value of a physical register.
            ArchValue::Name(name) => self.resolve_phys_value(name),
        }
    }
}

impl Storage {
    pub fn resolve_operand(&self, o: Operand) -> Option<u32> {
        match o {
            // This is an immediate operand, we already have the data
            Operand::Imm(data) => Some(data),
            // Try to resolve the value in this register
            Operand::ArchReg(reg) => self.resolve_arch_value(reg),
        }
    }

    pub fn rename_alloc(&mut self, idx: usize) {
        self.phys.push(PhysValue::Invalid);
        let name = self.phys.len();
        self.arch[idx] = ArchValue::Name(name);
    }

    pub fn writeback_phys(&mut self, idx: usize, data: u32) {
        self.phys[idx] = PhysValue::Valid(data);
    }
}




