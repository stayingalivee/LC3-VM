
mod register;
mod opcode;
mod cond_flags;
mod operations;
mod memory;
use register::Register;
use register::Reg;
use opcode::Opcode;
use operations::*;
use memory::Memory;

/**
 * Virutal machine implementing LC3 (Little Computer - 3)
 */
fn main() {
    println!("Starting VM........");

    // define the RAM
    let MAX: usize = 65535;
    let mut memory = Memory::new(MAX);

    //define registers and set PC to the default starting position
    let reg_count = 10;
    let PC_START: u16 = 0x3000;
    let mut register: Register = Default::default(); 
    register[Reg::R_PC] = PC_START;
    
    /*
     * From now on the process is fairly simple 
     * 1- load the instruction from the RAM (PC)
     * 2- increment PC
     * 3- inspect the opcode to determine the operation then perform it
     * 4- goto 1
    */
    let running: bool = true;
    while running {

        // get the instruction
        let instruction: u16 = memory[register[Reg::R_PC]];
    
        // get the operation bits
        let operation: u16 = instruction >> 12;


        match Opcode::from_u16(operation) {
            Opcode::OP_ST  =>  {
                op_st()
            },
            Opcode::OP_BR  =>  {
                op_br()
            },
            Opcode::OP_LD  =>  {
                op_ld()
            },
            Opcode::OP_ADD =>  {
                op_add(&mut register, instruction);
            },
            Opcode::OP_AND =>  {
                op_and()
            },
            Opcode::OP_JMP =>  {
                op_jmp()
            },
            Opcode::OP_JSR =>  {
                op_jsr()
            },
            Opcode::OP_LDI =>  {
                op_ldi(&mut register, instruction, &memory);
            },
            Opcode::OP_LDR =>  {
                op_ldr(&mut register, instruction, &memory)
            },
            Opcode::OP_LEA =>  {
                op_lea()
            },
            Opcode::OP_NOT =>  {
                op_not()
            },
            Opcode::OP_RES =>  {
                op_res()
            },
            Opcode::OP_RTI =>  {
                op_rti()
            },
            Opcode::OP_STI =>  {
                op_sti()
            },
            Opcode::OP_STR =>  {
                op_str()
            },
            Opcode::OP_TRAP => {
                op_trap()
            },
        }
        
       
   }
}
