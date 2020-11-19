use crate::register::Reg;
use crate::register::Register;
use crate::cond_flags::Cond_flags;
use std::num::Wrapping;


pub fn op_st(){

}

pub fn op_br(){

}

pub fn op_ld(){

}

/**
 * Add operation
 * 
 * instruction example
 * TODO: test this 
 * 0001 dr SR1 0 00 SR2 --> get the second value from SR2 register.
 * 0001 dr SR1 1 imm5   --> immediate mode, do DR1 + imm5 after sign extending.
 *               10001
 * 1111111111111101 -> -3
 * 0000000000000111 ->
 * 
 * a = b + c
 */
pub fn op_add(reg: &mut Register, instr: u16){
    let dr = (instr >> 9) & 0b111;                      // destination register
    let sr1 = (instr >> 6) & 0b111;                     // first operand
    let mode = (instr >> 5) & 0b1;                      // is it in an immediate mode ?

    if mode == 1 {                                      // if immediate mode then fetch the data from the instruction itself.
        let imm5: u16 = sign_ext(instr & 0b11111, 5);
        reg[dr] = reg[sr1].wrapping_add(imm5);
    } else {                                            // otherwise get the data from the register 
        let sr2 = instr & 0b111;
        reg[dr] = reg[sr1].wrapping_add(reg[sr2]);
    }
    update_flags(reg, dr);
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
 * 
 * 1010 dr PC_offset9
 */
pub fn op_ldi(){

}

pub fn op_ldr(){

}

pub fn op_lea(){

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

/// ================================================== ///
/// ================ test functions ================== ///
/// ================================================== ///
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