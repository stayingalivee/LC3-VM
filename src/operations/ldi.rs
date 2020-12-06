use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


/// 
/// Load Indirect operation, loads a value from an address
/// in memory into a register. Opcode is 1010
///
/// Instruction example
/// 1010 dr PC_offset9
/// 1010 001 100010011
///
///  An address is computed by sign-extending bits [8:0] to 16 bits and adding
/// this value to the incremented PC. What is stored in memory at this address
/// is theaddress of the data to be loaded into DR. The condition codes are set,
/// based onwhether the value loaded is negative, zero, or positive.
pub fn op_ldi(reg: &mut Register, instr: u16, memory: &Memory) {
    let dr = (instr >> 9) & 0b111; // get destination register
    let offset = sign_ext(instr & 0b111111111, 9); // sign extend pc offset
    reg[dr] = memory[memory[reg[Reg::R_PC].wrapping_add(offset)]]; // load indirect.
    update_flags(reg, dr);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_ldi() {
        let pc_start: u16 = 0x3000; // set program start
        let mut reg: Register = Default::default(); // init regs
        reg[Reg::R_PC] = pc_start; // init regs
        let instr: u16 = 0b1010_001_000000001; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3001] = 0x3002; // indirect pointer
        memory[0x3002] = 10; // actual data to be loaded
        op_ldi(&mut reg, instr, &mut memory);
        assert_eq!(reg[1], 10, "testing register value");
        assert_eq!(reg[Reg::R_COND], 0b001, "testing positive flags");
    }
    
    #[test]
    fn test_op_ldi_flags() {
        let pc_start: u16 = 0x3000; // set program start
        let mut reg: Register = Default::default(); // init regs
        reg[Reg::R_PC] = pc_start; // init regs
        let instr: u16 = 0b1010_001_000000001; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3001] = 0x3002; // indirect pointer
        memory[0x3002] = 0b1111111111111101; // actual data to be loaded
        op_ldi(&mut reg, instr, &mut memory);
        assert_eq!(reg[Reg::R_COND], 0b100, "testing negative flags");

        memory[0x3002] = 0;
        op_ldi(&mut reg, instr, &mut memory);
        assert_eq!(reg[Reg::R_COND], 0b010, "testing zero flags");
    }
}
