mod register;
mod opcode;
mod cond_flags;
use register::Register;
use register::Reg;
use opcode::Opcode;

// Driver code.
fn main() {
    println!("Starting VM........");

    // RAM
    let max: usize = 65535;
    let mut memory: vec![0; max];
    
    //define registers and set PC to the default starting position
    let reg_count = 10;
    let PC_START: i16 = 0x3000;
    let mut register  = Register {
        reg: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    };
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
        let instruction: i16 = memory[register[Reg::R_PC]];
        
        /* Instruction example
         * 0001 000 000  1     00011
         * ADD  R0  R0   Mode  3
        */
        
        // get the operation bits
        let operation = instruction >> 12;
        let op1 = (instruction >> 6) << 7;
        let op2 = (instruction >> 9) << 4;
        
        
        if operation == i16(Opcode::OP_ADD) {
            
        }

    }

}

// convert enum to i16
fn i16(opcode: Opcode) -> i16 {
     return opcode as i16;
}
