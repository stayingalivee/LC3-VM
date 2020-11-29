mod cond_flags;
mod memory;
mod opcode;
mod operations;
mod register;
mod traps;
use memory::Memory;
use opcode::Opcode;
use operations::*;
use register::Reg;
use register::Register;

/**
 * Virutal machine implementing LC3 (Little Computer - 3)
 */
fn main() {
    println!("Starting VM........");

    // define the RAM
    let max: usize = 65535;
    let mut memory = Memory::new(max);

    // declare registers and set PC to the default starting position
    let _reg_count = 10;
    let pc_start: u16 = 0x3000;
    let mut reg: Register = Default::default();
    reg[Reg::R_PC] = pc_start;

    /*
     * From now on the process is fairly simple
     * 1- load the instruction from the RAM (PC)
     * 2- increment PC
     * 3- inspect the opcode to determine the operation then perform it
     * 4- goto 1
     */
    let mut running: bool = true;
    while running {
        let instr: u16 = memory[reg[Reg::R_PC]];
        reg[Reg::R_PC] += 1;
        let operation: u16 = instr >> 12;

        match Opcode::from_u16(operation) {
            Opcode::OP_ST    =>  op_st(&reg, instr, &mut memory),
            Opcode::OP_STI   =>  op_sti(&reg, instr, &mut memory),
            Opcode::OP_STR   =>  op_str(&reg, instr, &mut memory),
            Opcode::OP_BR    =>  op_br(&mut reg, instr),
            Opcode::OP_LD    =>  op_ld(&mut reg, instr, &memory),
            Opcode::OP_ADD   =>  op_add(&mut reg, instr),
            Opcode::OP_AND   =>  op_and(&mut reg, instr),
            Opcode::OP_JMP   =>  op_jmp(&mut reg, instr),
            Opcode::OP_JSR   =>  op_jsr(&mut reg, instr),
            Opcode::OP_LDI   =>  op_ldi(&mut reg, instr, &memory),
            Opcode::OP_LDR   =>  op_ldr(&mut reg, instr, &memory),
            Opcode::OP_LEA   =>  op_lea(&mut reg, instr),
            Opcode::OP_NOT   =>  op_not(&mut reg, instr),
            Opcode::OP_RES   =>  op_res(),
            Opcode::OP_RTI   =>  op_rti(),
            Opcode::OP_TRAP  =>  running = op_trap(&mut reg, instr, &memory),
        }
    }
}
