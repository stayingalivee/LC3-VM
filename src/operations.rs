use crate::register::Reg;
use crate::register::Register;
use crate::cond_flags::Cond_flags;
use crate::memory::Memory;


pub fn op_st(){

}

pub fn op_br(){

}

pub fn op_ld(){

}

/**
 * Add operation
 * instruction example
 * 0001 dr SR1 0 00 SR2 --> get the second value from SR2 register.
 * 0001 dr SR1 1 imm5   --> immediate mode, do DR1 + imm5 after sign extending.
 */
pub fn op_add(reg: &mut Register, instr: u16){
    let dr = (instr >> 9) & 0b111;                      // destination register
    let sr1 = (instr >> 6) & 0b111;                     // first operand
    let mode = (instr >> 5) & 0b1;                      // is it in an immediate mode ?

    if mode == 1 {                                      // if immediate mode then fetch the data from the instruction itself.
        let imm5: u16 = sign_ext(instr & 0b11111, 5);
        reg[dr] = reg[sr1].wrapping_add(imm5);          // allow overflow to handle negative input
    } else {                                            // otherwise get the data from the register 
        let sr2 = instr & 0b111;
        reg[dr] = reg[sr1].wrapping_add(reg[sr2]);
    }
    update_flags(reg, dr);                              // update pos/neg/zero flags`
}

pub fn op_and(){

}

pub fn op_jmp(){

}

pub fn op_jsr(){

}

/**
 * Load Indirect operation, loads a value from an address
 * in memory into a register. Opcode is 1010
 * 
 * Instruction example
 * 1010 dr PC_offset9
 * 1010 001 100010011
 * 
 * An address is computed by sign-extending bits [8:0] to 16 bits and adding 
 * this value to the incremented PC. What is stored in memory at this address 
 * is theaddress of the data to be loaded into DR. The condition codes are set, 
 * based onwhether the value loaded is negative, zero, or positive.
 */
pub fn op_ldi(reg: &mut Register, instr: u16, memory: &Memory){
    let dr = (instr >> 9) & 0b111;                   // get destination register
    let offset = sign_ext(instr & 0b111111111, 9);   // sign extend pc offset
    reg[dr] = memory[memory[reg[Reg::R_PC].wrapping_add(offset)]];           // load indirect.
    update_flags(reg, dr);
}

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
pub fn op_ldr(reg: &mut Register, instr: u16, memory: &Memory){
    let dr = (instr >> 9) & 0b111;                         // get destination register
    let base_r = (instr >> 6) & 0b111;                     // get base_r from instr
    let offset = sign_ext(instr & 0b111111, 6);            // sign extend offset
    reg[dr] = memory[reg[base_r].wrapping_add(offset)];    // load immediate offset
    update_flags(reg, dr);
}

/**
 * Load Effective Address
 * 
 * Instruction example
 * 1110  dr  offset
 * 1110  001 000000011
 * 
 * An address is computed by sign-extending bits [8:0] to 16 bits and adding this
 * value to the incremented PC. This address is loaded into DR. The condition
 * codes are set, based on whether the value loaded is negative, zero, or positive.
 * 
 * LEA R0, Target
 */
pub fn op_lea(reg: &mut Register, instr: u16){
    let dr = (instr >> 9) & 0b111;                       // get destination reginster
    let offset = sign_ext(instr & 0b111111111, 9);       // sign extend and get offset
    reg[dr] = reg[Reg::R_PC].wrapping_add(offset);       // do lea operation
    update_flags(reg, dr);
}

pub fn op_not(){

}

pub fn op_res(){

}

pub fn op_rti(){

}

pub fn op_sti(){

}

pub fn op_str(){

}

pub fn op_trap(){

}


/// ================================================== ///
/// =============== helper functions ================= ///
/// ================================================== ///
pub fn sign_ext(mut val: u16, bit_count: i16) -> u16{
    if (val >> (bit_count - 1)) & 1 == 1 {
        val = val | 0xffff << bit_count;
    }
    return val;
}

fn update_flags(reg: &mut Register, dr: u16) {
    if reg[dr] == 0 {
        reg[Reg::R_COND] = Cond_flags::FL_ZRO as u16;
    }else if (reg[dr] >> 15) == 1 {
        reg[Reg::R_COND] = Cond_flags::FL_NEG as u16;
    }else{
        reg[Reg::R_COND] = Cond_flags::FL_POS as u16;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_sign_ext() {
        let val: u16 = 0b0000000000011111;
        let bit_count: i16 = 5;
        let extended = sign_ext(val, bit_count);
        println!("{}", extended);
    }
    
    #[test]
    fn test_op_add_imm(){
        let mut register  = Register {reg: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]};
        let instr: u16 = 0b0001_001_010_1_00011;
        op_add(&mut register, instr);
        assert_eq!(register[1], 3);
    }
    
    #[test]
    fn test_op_add(){
        let mut register  = Register {reg: [0, 0, 2, 3, 0, 0, 0, 0, 0, 0]};
        let instr: u16 = 0b0001_001_010_0_00011;
        op_add(&mut register, instr);
        assert_eq!(register[1], 5);
    }
    
    #[test]
    fn test_op_add_neg(){
        let mut register  = Register {reg: [0, 0, 0, 3, 0, 0, 0, 0, 0, 0]};
        let instr: u16 = 0b0001_001_010_0_00011;
        op_add(&mut register, instr);
        assert_eq!(register[1], 3);
    }
    
    #[test]
    fn test_op_add_sign_ext_imm5(){
        let mut register  = Register {reg: [0, 0, 2, 0, 0, 0, 0, 0, 0, 0]};
        let instr: u16 = 0b_0001_001_010_1_11111;
        op_add(&mut register, instr);
        assert_eq!(register[1], 0b0000000000000001);
    }
    
    #[test]
    fn test_pos_flag(){
        let mut register  = Register {reg: [0, 0, 4, 3, 0, 0, 0, 0, 0, 0]};
        let instr: u16 = 0b0001_001_010_0_00011;
        op_add(&mut register, instr);
        assert_eq!(register[Reg::R_COND], 0b001);
    }
    
    #[test]
    fn test_neg_flag(){
        let mut register  = Register {reg: [0, 0, 1, 0, 0, 0, 0, 0, 0, 0]};
        let instr: u16 = 0b0001_001_010_1_10011;
        op_add(&mut register, instr);
        assert_eq!(register[Reg::R_COND], 0b100);
    }
    
    #[test]
    fn test_zero_flag(){
        let mut register  = Register {reg: [0, 0, 1, 0, 0, 0, 0, 0, 0, 0]};
        let instr: u16 = 0b0001_001_010_1_11111;
        op_add(&mut register, instr);
        assert_eq!(register[Reg::R_COND], 0b010);
    }

    #[test]
    fn test_ldi(){
        let pc_start: u16 = 0x3000;                     // set program start
        let mut reg: Register = Default::default();     // init regs
        reg[Reg::R_PC] = pc_start;                      // init regs
        let instr: u16 = 0b1010_001_000000001;          // define instruction
        let mut memory = Memory::new(65535);            // declare memory
        memory[0x3001] = 0x3002;                        // indirect pointer
        memory[0x3002] = 10;                            // actual data to be loaded
        op_ldi(&mut reg, instr, &mut memory);
        assert_eq!(reg[1], 10, "testing register value");
        assert_eq!(reg[Reg::R_COND], 0b001, "testing positive flags");
    }

    #[test]
    fn test_ldi_flags(){
        let pc_start: u16 = 0x3000;                     // set program start
        let mut reg: Register = Default::default();     // init regs
        reg[Reg::R_PC] = pc_start;                      // init regs
        let instr: u16 = 0b1010_001_000000001;          // define instruction
        let mut memory = Memory::new(65535);            // declare memory
        memory[0x3001] = 0x3002;                        // indirect pointer
        memory[0x3002] = 0b1111111111111101;            // actual data to be loaded
        op_ldi(&mut reg, instr, &mut memory);
        assert_eq!(reg[Reg::R_COND], 0b100, "testing negative flags");

        memory[0x3002] = 0;
        op_ldi(&mut reg, instr, &mut memory);
        assert_eq!(reg[Reg::R_COND], 0b010, "testing zero flags");
    }
    
    #[test]
    fn test_ldr(){
        let mut register  = Register {reg: [0, 0x3001, 0, 0, 0, 0, 0, 0, 0, 0]};

        let instr: u16 = 0b0110_000_001_000011;          // define instruction
        let mut memory = Memory::new(65535);            // declare memory
        memory[0x3004] = 10;
        op_ldr(&mut register, instr, &memory);
        assert_eq!(register[0], 10, "testing register value");
        assert_eq!(register[Reg::R_COND], 0b001, "testing positive flag");
    }

    #[test]
    fn test_ldr_neg_offset(){
        let mut register  = Register {reg: [0, 0x3001, 0, 0, 0, 0, 0, 0, 0, 0]};

        let instr: u16 = 0b0110_000_001_111111;          // define instruction
        let mut memory = Memory::new(65535);            // declare memory
        memory[0x3000] = 10;
        op_ldr(&mut register, instr, &memory);
        assert_eq!(register[0], 10, "testing register value");
        assert_eq!(register[Reg::R_COND], 0b001, "testing positive flag");
    }

    #[test]
    fn test_ldr_flags(){
        let mut register  = Register {reg: [0, 0x3001, 0, 0, 0, 0, 0, 0, 0, 0]};

        let instr: u16 = 0b0110_000_001_111111;          // define instruction
        let mut memory = Memory::new(65535);            // declare memory
        memory[0x3000] = 0b1111111111111011;
        op_ldr(&mut register, instr, &memory);
        assert_eq!(register[Reg::R_COND], 0b100, "testing positive flag");
    }

    #[test]
    fn test_lea(){
        let mut register: Register = Default::default();
        let instr:u16 = 0b1110_001_000000011;
        register[Reg::R_PC] = 0x3000;
        op_lea(&mut register, instr);
        assert_eq!(register[1], 0x3003);
    }

    #[test]
    fn test_lea_neg_offset(){
        let mut register: Register = Default::default();
        let instr:u16 = 0b1110_001_111111111;
        register[Reg::R_PC] = 0x3001;
        op_lea(&mut register, instr);
        assert_eq!(register[1], 0x3000);
    }
}