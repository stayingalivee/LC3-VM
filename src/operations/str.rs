use crate::defs::register::*;
use crate::defs::memory::*;
use crate::operations::helper::*;


/**
 * Store base + offset
 * 
 * Instruction example
 * 0111 sr  base 000001
 * 0111 001 002  000001
 * 
 * The contents of the register specified by SR are stored in the memory location
 * whose address is computed by sign-extending bits [5:0] to 16 bits and adding this
 * value to the contents of the register specified by bits [8:6]
 */
pub fn op_str(reg: & Register, instr: u16, memory: &mut Memory) {
    let offset = sign_ext(instr & 0b111111, 6);
    let base = (instr >> 6) & 0b111;
    let sr = (instr >> 9) & 0b111;
    memory[reg[base].wrapping_add(offset)] = reg[sr];
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_op_str(){
        let mut reg: Register = Default::default();
        let instr: u16 = 0b0111_001_010_000011;
        reg[1] = 10;
        reg[2] = 0x3000;
        let mut memory = Memory::new(65535);
        op_str(&reg, instr, &mut memory);
        assert_eq!(memory[0x3003], 10);
    }
}
