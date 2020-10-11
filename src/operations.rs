use crate::register::Reg;
use crate::register::Register;

pub fn op_st(){

}

pub fn op_br(){

}

pub fn op_ld(){

}

/**
 * instruction example
 * TODO: test this 
 * 0001 DR SR1 0 00 SR2 --> SR2 has the actual data.
 * 0001 DR SR1 1 imm5   --> imm5 is a pointer to the memory that has our data. 
 */
pub fn op_add(reg: &mut Register, instr_wo_op: u16){

    // destination register
    let mut DR = instr_wo_op >> 9;
    // first operand
    let SR1 = (instr_wo_op >> 6) << 3;
    // is it in an immediate mode
    let mode = (instr_wo_op >> 5) << 6;

    // if immediate mode then fetch the data from the instruction
    if mode == 1 {
        let imm5: u16 = sign_ext(instr_wo_op << 7);
        reg[DR] = reg[SR1] + imm5;

    } // otherwise get the data from the register 
    else {
        let SR2 = instr_wo_op << 13;
        reg[DR] = reg[SR1] + reg[SR2];
    }

    update_flags(DR);
}

pub fn op_and(){

}

pub fn op_jmp(){

}

pub fn op_jsr(){

}

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

fn sign_ext(imm5: u16) -> u16{
    //TODO: fill in sign extending logic.
    return 0;
}

fn update_flags(DR: u16) {
    //TODO: fill in flag update logic.
}