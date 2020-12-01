use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;

/**
 * Store indirect
 * 
 * Instruction example
 * 1011 sr  offset
 * 1011 001 000000011
 * 
 * The contents of the register specified by SR are stored in the memory location
 * whose address is obtained as follows: Bits [8:0] are sign-extended to 16 bits and
 * added to the incremented PC. What is in memory at this address is the address of
 * the location to which the data in SR is stored.
 */

pub fn op_sti(reg: & Register, instr: u16, memory: &mut Memory) {
    let offset = sign_ext(instr & 0b111111111, 9);                 // get offset
    let sr = (instr >> 9) & 0b111;                                 // get source reg
    let address = memory[reg[Reg::R_PC].wrapping_add(offset)];
    memory[address] = reg[sr];                                     // store indirectly
}
