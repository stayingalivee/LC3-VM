use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


/// Store
/// 
/// Instruction Example
/// 0011 SR  offset
/// 0011 001 000000011
/// 
/// The contents of the register specified by SR are stored in the memory location
/// whose address is computer by sign extending bits 8-0 to 16 bits and adding this
/// vlaue to the incremented PC.
pub fn op_st(reg: & Register, instr: u16, memory: &mut Memory) {
    let offset = sign_ext(instr & 0b111111111, 9);                // get offset
    let sr = (instr >> 9) & 0b111;                                // get source reg
    memory[reg[Reg::R_PC].wrapping_add(offset)] = reg[sr];        // store to memory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_st(){
        let mut reg: Register = Default::default();
        reg[Reg::R_PC] = 0x3000;
        reg[1] = 10;
        let instr: u16 = 0b0011_001_000000011;
        let mut memory = Memory::new(65535);
        op_st(&reg, instr, &mut memory);
        assert_eq!(memory[0x3003], 10);
    }
}
