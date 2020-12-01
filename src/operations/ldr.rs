use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


/**
 * Load immedate offset, loads a value from an address in memory
 * into a register, Opcode is 0110
 *
 * Instruction example
 * 0110 dr  base offset
 * 0110 001 011  000001
 *
 * n address is computed by sign-extending bits [5:0] to 16 bits and
 * adding thisvalue to the contents of the register specified by bits [8:6]
 * The contents of memoryat this address are loaded into DR.
 * The condition codes are set, based on whetherthe value loaded is
 * negative, zero, or positive
 */
pub fn op_ldr(reg: &mut Register, instr: u16, memory: &Memory) {
    let dr = (instr >> 9) & 0b111; // get destination register
    let base_r = (instr >> 6) & 0b111; // get base_r from instr
    let offset = sign_ext(instr & 0b111111, 6); // sign extend offset
    reg[dr] = memory[reg[base_r].wrapping_add(offset)]; // load immediate offset
    update_flags(reg, dr);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_op_ldr() {
        let mut register = Register {
            reg: [0, 0x3001, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        let instr: u16 = 0b0110_000_001_000011; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3004] = 10;
        op_ldr(&mut register, instr, &memory);
        assert_eq!(register[0], 10, "testing register value");
        assert_eq!(register[Reg::R_COND], 0b001, "testing positive flag");
    }

    #[test]
    fn test_op_ldr_neg_offset() {
        let mut register = Register {
            reg: [0, 0x3001, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        let instr: u16 = 0b0110_000_001_111111; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3000] = 10;
        op_ldr(&mut register, instr, &memory);
        assert_eq!(register[0], 10, "testing register value");
        assert_eq!(register[Reg::R_COND], 0b001, "testing positive flag");
    }

    #[test]
    fn test_op_ldr_flags() {
        let mut register = Register {
            reg: [0, 0x3001, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        let instr: u16 = 0b0110_000_001_111111; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3000] = 0b1111111111111011;
        op_ldr(&mut register, instr, &memory);
        assert_eq!(register[Reg::R_COND], 0b100, "testing positive flag");
    }

}
