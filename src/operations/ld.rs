use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


/**
 * Load
 * 
 * Instruction example
 * 0010 DR  offset
 * 0010 001 000000011
 * 
 * An address is comuputed by sign_extending bits 8 to 0 to 16 bits and adding
 * this value to the incremented PC. The content of memory at this address is loaded
 * into DR. the condition codes are set based on whether the value loaded is 
 * negative, zero, or positive.
 */
pub fn op_ld(reg: &mut Register, instr: u16, memory: &Memory) {
    let offset = sign_ext(instr & 0b111111111, 9);               // sign extend and get offset
    let dr = (instr >> 9) & 0b111;                               // get destination register
    reg[dr] = memory[reg[Reg::R_PC].wrapping_add(offset)];       // load from memory
    update_flags(reg, dr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_ld(){
        let mut reg: Register = Default::default();
        reg[Reg::R_PC] = 0x3000;
        let instr: u16 = 0b0010_001_000000011;
        let mut memory = Memory::new(65535);
        memory[0x3003] = 10;

        op_ld(&mut reg, instr, &memory);
        assert_eq!(reg[1] , 10);
    }
}
