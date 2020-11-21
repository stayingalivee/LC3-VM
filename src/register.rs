use std::ops::{Index,IndexMut};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Reg{
    R_R0 = 0,
    R_R1 = 1,
    R_R2 = 2,
    R_R3 = 3,
    R_R4 = 4,
    R_R5 = 5,
    R_R6 = 6,
    R_R7 = 7,
    R_PC = 8,
    R_COND = 9,
}

pub struct Register {
    pub reg: [u16; 10],
}


impl Default for Register {
    fn default() -> Register {
        Register {
            reg: [0; 10]
        }
    }
}

/// override indexing with enum Reg
impl IndexMut<Reg> for Register {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        &mut self.reg[index as usize]
    }
}
impl Index<Reg> for Register {
    type Output = u16;
    fn index(&self, index: Reg) -> &Self::Output {
        &self.reg[index as usize]
    }
}

/// override indexing with u16
impl IndexMut<u16> for Register {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
       &mut self.reg[index as usize]
    }
}
impl Index<u16> for Register {
    type Output = u16;
    fn index(&self, index: u16) -> &Self::Output {
        &self.reg[index as usize]
    }
}