mod defs;
mod operations;

use defs::memory::Memory;
use defs::register::*;
use operations::executor::*;


///
/// V irutal machine implementing LC3 (Little Computer - 3)
///
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

    // From now on the process is fairly simple
    // 1- load the instruction from the RAM (PC)
    // 2- increment PC
    // 3- inspect the opcode to determine the operation then perform it
    // 4- goto 1
    let mut running: bool = true;
    while running {
        let instr: u16 = memory[reg[Reg::R_PC]];                // fetch instruction
        reg[Reg::R_PC] += 1;                                    // increment program counter
        execute(instr, &mut reg, &mut memory, &mut running);        // execute instruction
    }
}
