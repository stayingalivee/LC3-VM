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
    pub reg: [i16; 10],
}

impl IndexMut<Reg> for Register {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        &mut self.reg[index as usize]
    }
}

impl Index<Reg> for Register {
    type Output = i16;
    fn index(&self, index: Reg) -> &Self::Output {
        &self.reg[index as usize]
    }
}
