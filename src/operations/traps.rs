use crate::defs::traps::Traps;
use crate::defs::register::*;
use crate::defs::memory::*;
use std::io::Read;

/// trap routines
/// 
/// The implementation of the traps is provided in normal rust functions 
/// instead of redirecting the instruction flow to a pre-determined address 
/// on the memory(like normal machines do). 
pub fn op_trap(reg: &mut Register, instr: u16, memory: &Memory) -> bool {
    let mut running: bool = true;
    match Traps::from_u16(instr & 0xFF){
        Traps::TRAP_GETC  =>  trap_getc(reg),
        Traps::TRAP_HALT  =>  trap_halt(&mut running),
        Traps::TRAP_IN    =>  trap_in(reg),
        Traps::TRAP_OUT   =>  trap_out(reg),
        Traps::TRAP_PUTS  =>  trap_puts(reg, &memory),
        Traps::TRAP_PUTSP =>  trap_putsp(reg, &memory),
    }
    return running;
}

/// GETC trap code used to get one chracter from the standard input
/// the character is saved to R0.
fn trap_getc(reg: &mut Register){
    let input: u16 = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u16).unwrap();
    reg[Reg::R_R0] = input;
}

/// HALT Trap code to halt the program.
fn trap_halt(running: &mut bool){
    println!("HALT PROGRAM");
    *running = false;
}

/// IN trap code to get one character from stdin and output it to stdout
/// after its saved to R0.
fn trap_in(reg: &mut Register){
    print!("Enter a character: ");
    let input: char = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char).unwrap();

    reg[Reg::R_R0] = input as u16;
}

/// OUT trap code used to output a character to standard output.
fn trap_out(reg: &Register){
   print!("{}", reg[Reg::R_R0]); 
}

/// PUTS trap code used to output a null terminated string.
/// The string displayed has its address in R0. In LC3 a character
/// is stored in a single momory location => each character is 16 bits
/// and not one byte
fn trap_puts(reg: &Register, memory: &Memory){
    let mut i = reg[Reg::R_R0]; 
    let mut c: char = 'a';
    while c != '\0'{
        c = (memory[i] as u8) as char;
        print!("{}", c);
        i += 1;
    }
}

/// PUTSP trap code used to output a null terminated string to stdout
/// the address of the string is fetched from R0. Characters are printed
/// in a big endian format.
fn trap_putsp(reg: &Register, memory: &Memory){
    let mut i: u16 = reg[Reg::R_R0];
    while memory[i] as u8 as char != '\0' {
        let c1: char = (memory[i] & 0xFF) as u8 as char;
        print!("{}", c1);
        let c2: u8 = (memory[i] >> 8) as u8;
        if c2 != 0 {
            print!("{}", c2 as char);
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trap_halt(){
        let mut running = true;
        trap_halt(&mut running);
        assert_eq!(running, false);
    }

    #[test]
    fn test_trap_puts(){
        let mut register = Register::default();
        let memory = Memory::new(100);
        trap_puts(&mut register, &memory);
    }
}
