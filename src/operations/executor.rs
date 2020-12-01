use crate::defs::register::*;
use crate::defs::memory::*;
use crate::defs::opcode::*;

pub fn execute(instr: u16, reg:&mut Register,memory: &mut Memory) -> bool {
    let mut running: bool = true;
    let operation: u16 = instr >> 12;

    match Opcode::from_u16(operation) {
        Opcode::OP_ST    => super::st::op_st(&reg, instr, memory),
        Opcode::OP_STI   => super::sti::op_sti(&reg, instr, memory),
        Opcode::OP_STR   => super::str::op_str(&reg, instr, memory),
        Opcode::OP_BR    => super::br::op_br(reg, instr),
        Opcode::OP_LD    => super::ld::op_ld(reg, instr, memory),
        Opcode::OP_ADD   => super::add::op_add(reg, instr),
        Opcode::OP_AND   => super::and::op_and(reg, instr),
        Opcode::OP_JMP   => super::jmp::op_jmp(reg, instr),
        Opcode::OP_JSR   => super::jsr::op_jsr(reg, instr),
        Opcode::OP_LDI   => super::ldi::op_ldi(reg, instr, memory),
        Opcode::OP_LDR   => super::ldr::op_ldr(reg, instr, memory),
        Opcode::OP_LEA   => super::lea::op_lea(reg, instr),
        Opcode::OP_NOT   => super::not::op_not(reg, instr),
       // Opcode::OP_RES   => super::op_res(),
      //  Opcode::OP_RTI   => super:: op_rti(),
        Opcode::OP_TRAP  =>  running = super::traps::op_trap(reg, instr, memory)
    }
    return running;
}
