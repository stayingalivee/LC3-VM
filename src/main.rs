mod defs;
mod operations;

use defs::memory::Memory;
use defs::register::*;
use operations::executor::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Error, ErrorKind, Read};

///
/// Virutal machine implementing LC3 (Little Computer - 3)
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

pub fn read_image_file(memory: &mut Memory, image_path: String) -> Result<Vec<u16>, Error> {
    let mut buffer = Vec::new();
    File::open(image_path)?.read_to_end(&mut buffer)?;
    get_instr_from_buffer(&buffer)
}

pub fn get_instr_from_buffer(data: &[u8]) -> Result<Vec<u16>, Error> {
    if data.len() % 2 != 0 {
        return Err(Error::new(ErrorKind::InvalidData,
            "input must be a multiple of 2"));
    }
    Ok(data
        .chunks(2)
        .map(|x| x[1] as u16 | (x[0] as u16) << 8)
        .collect())
}

pub fn print_instr(x: u16) {
    let mut number = x.clone();
    let mut i = 16;
    while i > 0 {
        let bit = (number & 0b1000000000000000) >> 15;
        print!("{}", bit);
        number = number << 1;
        i -= 1;
    }
    println!("");
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loading_image_file(){
        let max: usize = 65535;
        let mut memory = Memory::new(max);
        if let Ok(instructions) = read_image_file(&mut memory, String::from("./halt.obj")) {
            for instr in instructions.clone(){
                print_instr(instr);
            }
        }
         
    }
}
