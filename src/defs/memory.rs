use std::ops::{Index, IndexMut};


/// Random Access Memory RAM struct.
/// implements Index and IndexMut trait to facilitate indexing with u16
/// and Reg enum (to use PC)
pub struct Memory {
    pub size: usize,
    pub memory: Vec<u16>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            memory: vec![0; size]
        }
    }
}

impl Index<u16> for Memory {
    type Output = u16;
    fn index(&self, index: u16) -> &Self::Output {
        &self.memory[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.memory[index as usize]        
    }
}
