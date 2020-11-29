use crate::cond_flags::Cond_flags;
use crate::memory::Memory;
use crate::register::*;
use crate::traps::Traps;
use std::io::Read;

/**
 * Store
 * 
 * Instruction Example
 * 0011 SR  offset
 * 0011 001 000000011
 * 
 * The contents of the register specified by SR are stored in the memory location
 * whose address is computer by sign extending bits 8-0 to 16 bits and adding this
 * vlaue to the incremented PC.
 */
pub fn op_st(reg: & Register, instr: u16, memory: &mut Memory) {
    let offset = sign_ext(instr & 0b111111111, 9);                // get offset
    let sr = (instr >> 9) & 0b111;                                // get source reg
    memory[reg[Reg::R_PC].wrapping_add(offset)] = reg[sr];        // store to memory
}

/**
 * Store indirect
 * 
 * Instruction example
 * 1011 sr  offset
 * 1011 001 000000011
 * 
 * The contents of the register specified by SR are stored in the memory location
 * whose address is obtained as follows: Bits [8:0] are sign-extended to 16 bits and
 * added to the incremented PC. What is in memory at this address is the address of
 * the location to which the data in SR is stored.
 */
pub fn op_sti(reg: & Register, instr: u16, memory: &mut Memory) {
    let offset = sign_ext(instr & 0b111111111, 9);                 // get offset
    let sr = (instr >> 9) & 0b111;                                 // get source reg
    let address = memory[reg[Reg::R_PC].wrapping_add(offset)];
    memory[address] = reg[sr];                                     // store indirectly
}

/**
 * Store base + offset
 * 
 * Instruction example
 * 0111 sr  base 000001
 * 0111 001 002  000001
 * 
 * The contents of the register specified by SR are stored in the memory location
 * whose address is computed by sign-extending bits [5:0] to 16 bits and adding this
 * value to the contents of the register specified by bits [8:6]
 */
pub fn op_str(reg: & Register, instr: u16, memory: &mut Memory) {
    let offset = sign_ext(instr & 0b111111, 6);
    let base = (instr >> 6) & 0b111;
    let sr = (instr >> 9) & 0b111;
    memory[reg[base].wrapping_add(offset)] = reg[sr];
}

/**
 * Conditional Branch.
 *
 * Instruction example
 * 0000 n z p   offset
 * 0000 1 1 1 000000011
 *
 * The condition codes specified by the state of bits [11:9] are tested. If bit [11] is set,
 * N is tested; if bit [11] is clear, N is not tested. If bit [10] is set, Z is tested, etc.
 * If any of the condition codes tested is set, the program branches to the location
 * specified by adding the sign-extended PCoffset9 field to the incremented PC.
 */
pub fn op_br(reg: &mut Register, instr: u16) {
    let offset = sign_ext(instr & 0b111111111, 9);             // get offset to be incremented
    let flags = (instr >> 9) & 0b111;                          // get the nzp flag

    if flags & reg[Reg::R_COND] != 0 {                         // if bitwise add doesn't produce
        reg[Reg::R_PC] = reg[Reg::R_PC].wrapping_add(offset);  // an on bit then all flags are 0
    }                                                          // in which case condition isn't met
}

/**
 * Load
 * 
 * Instruction example
 * 0010 DR  offset
 * 0010 001 000000011
 * 
 * An address is comuputed by sign_extending bits 8 to 0 to 16 bits and adding
 * this value to the incremented PC. The content of memory at this address is loaded
 * into DR. the condition codes are set based on whether the value loaded is 
 * negative, zero, or positive.
 */
pub fn op_ld(reg: &mut Register, instr: u16, memory: &Memory) {
    let offset = sign_ext(instr & 0b111111111, 9);               // sign extend and get offset
    let dr = (instr >> 9) & 0b111;                               // get destination register
    reg[dr] = memory[reg[Reg::R_PC].wrapping_add(offset)];       // load from memory
    update_flags(reg, dr);
}

/**
 * Add operation
 * instruction example
 * 0001 dr SR1 0 00 SR2 --> get the second value from SR2 register.
 * 0001 dr SR1 1 imm5   --> immediate mode, do DR1 + imm5 after sign extending.
 */
pub fn op_add(reg: &mut Register, instr: u16) {
    let dr = (instr >> 9) & 0b111;                  // destination register
    let sr1 = (instr >> 6) & 0b111;                 // first operand
    let mode = (instr >> 5) & 0b1;                  // is it in an immediate mode ?

    if mode == 1 {
        // if immediate mode then fetch the data from the instruction itself.
        let imm5: u16 = sign_ext(instr & 0b11111, 5);
        reg[dr] = reg[sr1].wrapping_add(imm5); // allow overflow to handle negative input
    } else {
        // otherwise get the data from the register
        let sr2 = instr & 0b111;
        reg[dr] = reg[sr1].wrapping_add(reg[sr2]);
    }
    update_flags(reg, dr); // update pos/neg/zero flags`
}

/**
 * Bitwise logical AND
 * 
 * Instruction example
 * 0101 DR  SR1 0 00 SR2
 * 0101 DR  SR2 1 imm5
 * 
 * f bit [5] is 0, the second source operand is obtained from SR2. If bit [5] is 1,
 * the second source operand is obtained by sign-extending the imm5 field to 16
 * bits. In either case, the second source operand and the contents of SR1 are 
 * bit-wise ANDed, and the result stored in DR. The condition codes are set, based on
 * whether the binary value produced, taken as a 2’s complement integer,
 * is negative,zero, or positive.
 */
pub fn op_and(reg: &mut Register, instr: u16) {
    let dr = (instr >> 9) & 0b111;                   // get destination register
    let sr1 = (instr >> 6) & 0b111;                  // get source register #1
    let mode = (instr >> 5) & 0b1;
    if mode == 1 {                                   // immediate mode
        let imm5 = sign_ext(instr & 0b11111, 5);
        reg[dr] = reg[sr1] & imm5;
    }else{                  
        let sr2 = instr & 0b111;                         // get second operand from sr2
        reg[dr] = reg[sr1] & reg[sr2];
    }
    update_flags(reg, dr);
}

/**
 * Jump
 *
 * Instruction example
 * 1100 000 sr  000000
 * 1100 000 001 000000
 *
 * The program unconditionally jumps to the location specified by the contents of
 * the base register. Bits [8:6] identify the base register.
 *
 * The RET instruction is a special case of the JMP instruction.
 * The PC is loaded with the contents of R7, which contains the linkage
 * back to the instructionfollowing the subroutine call instruction.
 */
pub fn op_jmp(reg: &mut Register, instr: u16) {
    let sr = (instr >> 6) & 0b111;
    reg[Reg::R_PC] = reg[sr];
}

/**
 * Jump to subroutine
 * 
 * Instruction example
 * 0100 1 offset
 * 0100 0 00 SR 000000 
 * 
 * First, the incremented PC is saved to R7. This is the linkage back to the calling
 * routine. Then the PC is loaded with the address of the first instruction of the 
 * subroutine, causing an unconditional jump to that address. The address of the
 * subroutine is obtained from the base register(if bit 11 is 0), or the address is 
 * computed by sign-extending bits 10 to 0 abd adding this value to the incremented
 * PC(if bit 11 is 1)
 * 
 */
pub fn op_jsr(reg: &mut Register, instr: u16) {
    reg[Reg::R_R7] = reg[Reg::R_PC];                           // save PC value in R7
    if (instr >> 11) & 0b1 == 0 {                              // if jmp mode is immediate
        let sr = (instr >> 6) & 0b111;                         // get the register that cointains the address we want to jmp to
        reg[Reg::R_PC] = reg[sr];                              // jump
    } else {                                                   // if jump mode is not immediate
        let offset = sign_ext(instr & 0b11111111111, 11);      // sign_extend offset
        reg[Reg::R_PC] = reg[Reg::R_PC].wrapping_add(offset);  // add offset to current PC allowing overflow
    }
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
pub fn op_ldi(reg: &mut Register, instr: u16, memory: &Memory) {
    let dr = (instr >> 9) & 0b111; // get destination register
    let offset = sign_ext(instr & 0b111111111, 9); // sign extend pc offset
    reg[dr] = memory[memory[reg[Reg::R_PC].wrapping_add(offset)]]; // load indirect.
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
pub fn op_ldr(reg: &mut Register, instr: u16, memory: &Memory) {
    let dr = (instr >> 9) & 0b111; // get destination register
    let base_r = (instr >> 6) & 0b111; // get base_r from instr
    let offset = sign_ext(instr & 0b111111, 6); // sign extend offset
    reg[dr] = memory[reg[base_r].wrapping_add(offset)]; // load immediate offset
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
pub fn op_lea(reg: &mut Register, instr: u16) {
    let dr = (instr >> 9) & 0b111; // get destination register
    let offset = sign_ext(instr & 0b111111111, 9); // sign extend and get offset
    reg[dr] = reg[Reg::R_PC].wrapping_add(offset); // do lea operation
    update_flags(reg, dr);
}

/**
 * Bitwise inversion
 *
 * Instruction exmple
 * 1100 dr sr 111111
 *
 * The bit-wise complement of the contents of SR is stored in DR. The condition
 * codes are set, based on whether the binary value produced is negative, zero, or positive.
 */
pub fn op_not(reg: &mut Register, instr: u16) {
    let dr = (instr >> 9) & 0b111; // get destination register
    let sr = (instr >> 6) & 0b111; // get source register
    reg[dr] = !reg[sr]; // do bitwise not operation
    update_flags(reg, dr);
}

pub fn op_res() {/* not used */}
pub fn op_rti() {/* not used */}


//==================================================//
//====================== TRAPS =====================//
//==================================================//
/**
 * trap routines
 * 
 * The implementation of the traps is provided in normal rust functions 
 * instead of redirecting the instruction flow to a pre-determined address 
 * on the memory(like normal machines do). 
 */
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

/**
 * GETC trap code used to get one chracter from the standard input
 * the character is saved to R0.
 */
fn trap_getc(reg: &mut Register){
    let input: u16 = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u16).unwrap();
    reg[Reg::R_R0] = input;
}

/**
 * HALT Trap code to halt the program.
 */
fn trap_halt(running: &mut bool){
    println!("HALT PROGRAM");
    *running = false;
}

/**
 * IN trap code to get one character from stdin and output it to stdout
 * after its saved to R0.
 */
fn trap_in(reg: &mut Register){
    print!("Enter a character: ");
    let input: char = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char).unwrap();

    reg[Reg::R_R0] = input as u16;
}

/**
 * OUT trap code used to output a character to standard output.
 */
fn trap_out(reg: &Register){
   print!("{}", reg[Reg::R_R0]); 
}

/**
 * PUTS trap code used to output a null terminated string.
 * The string displayed has its address in R0. In LC3 a character
 * is stored in a single momory location => each character is 16 bits
 * and not one byte
 */
fn trap_puts(reg: &Register, memory: &Memory){
    let mut i = reg[Reg::R_R0]; 
    let mut c: char = 'a';
    while c != '\0'{
        c = (memory[i] as u8) as char;
        print!("{}", c);
        i += 1;
    }
}

/**
 * PUTSP trap code used to output a null terminated string to stdout
 * the address of the string is fetched from R0
 */
fn trap_putsp(reg: &Register, memory: &Memory){
    let mut i: u16 = reg[Reg::R_R0];
    let mut c: char = 'a';
    while c!= '\0' {
        let c1: char = (memory[i] & 0xFF) as u8 as char;
        print!("{}", c1);
        let c2: u8 = (memory[i] >> 8) as u8;
        if c2 != 0 {
            print!("{}", c2 as char);
        }

    }
}

//==================================================//
//===================== Helpers ====================//
//==================================================//
pub fn sign_ext(mut val: u16, bit_count: i16) -> u16 {
    if (val >> (bit_count - 1)) & 1 == 1 {
        val = val | 0xffff << bit_count;
    }
    return val;
}

fn update_flags(reg: &mut Register, dr: u16) {
    if reg[dr] == 0 {
        reg[Reg::R_COND] = Cond_flags::FL_ZRO as u16;
    } else if (reg[dr] >> 15) == 1 {
        reg[Reg::R_COND] = Cond_flags::FL_NEG as u16;
    } else {
        reg[Reg::R_COND] = Cond_flags::FL_POS as u16;
    }
}

//==================================================//
//====================== Tests =====================//
//==================================================//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_puts(){
        let mut register = Register::default();
        let memory = Memory::new(100);
        trap_puts(&mut register, &memory);
    }

    #[test]
    fn test_op_and(){
        let mut register = Register::default();
        register[2] = 0b0000000011111111;
        let instr: u16 = 0b0101_001_010_0_00_011;
        op_and(&mut register, instr);
        assert_eq!(register[1], 0);

        let instr: u16 = 0b0101_001_010_1_00011;
        op_and(&mut register, instr);
        assert_eq!(register[1], 3);
    }

    #[test]
    fn test_op_str(){
        let mut reg: Register = Default::default();
        let instr: u16 = 0b0111_001_010_000011;
        reg[1] = 10;
        reg[2] = 0x3000;
        let mut memory = Memory::new(65535);
        op_str(&reg, instr, &mut memory);
        assert_eq!(memory[0x3003], 10);
    }

    #[test]
    fn test_op_st(){
        let mut reg: Register = Default::default();
        reg[Reg::R_PC] = 0x3000;
        reg[1] = 10;
        let instr: u16 = 0b0011_001_000000011;
        let mut memory = Memory::new(65535);
        op_st(&reg, instr, &mut memory);
        assert_eq!(memory[0x3003], 10);
    }

    #[test]
    fn test_op_ld(){
        let mut reg: Register = Default::default();
        reg[Reg::R_PC] = 0x3000;
        let instr: u16 = 0b0010_001_000000011;
        let mut memory = Memory::new(65535);
        memory[0x3003] = 10;

        op_ld(&mut reg, instr, &memory);
        assert_eq!(reg[1] , 10);

    }

    #[test]
    fn test_op_jsr(){
        let mut register = Register::default();
        // 0100 1 offset
        let instr: u16 = 0b0100_1_00000000011;
        register[Reg::R_PC] = 0x3000;
        op_jsr(&mut register, instr);
        assert_eq!(register[Reg::R_PC], 0x3003);
        assert_eq!(register[Reg::R_R7], 0x3000);

        // 0100 0 00 SR 000000 
        let instr = 0b0100_0_00_010_000000;
        register[Reg::R_R2] = 0x3500;
        op_jsr(&mut register, instr);
        assert_eq!(register[Reg::R_PC], 0x3500);
        assert_eq!(register[Reg::R_R7], 0x3003);

        // 0100 1 offset -> negative offset
        let instr: u16 = 0b0100_1_11111111111;
        register[Reg::R_PC] = 0x3001;
        op_jsr(&mut register, instr);
        assert_eq!(register[Reg::R_PC], 0x3000);
        assert_eq!(register[Reg::R_R7], 0x3001);
    }

    #[test]
    fn test_sign_ext() {
        let val: u16 = 0b0000000000011111;
        let bit_count: i16 = 5;
        let extended = sign_ext(val, bit_count);
        println!("{}", extended);
    }
    #[test]
    fn test_op_add_imm() {
        let mut register = Register {
            reg: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let instr: u16 = 0b0001_001_010_1_00011;
        op_add(&mut register, instr);
        assert_eq!(register[1], 3);
    }
    #[test]
    fn test_op_add() {
        let mut register = Register {
            reg: [0, 0, 2, 3, 0, 0, 0, 0, 0, 0],
        };
        let instr: u16 = 0b0001_001_010_0_00011;
        op_add(&mut register, instr);
        assert_eq!(register[1], 5);
    }
    #[test]
    fn test_op_add_neg() {
        let mut register = Register {
            reg: [0, 0, 0, 3, 0, 0, 0, 0, 0, 0],
        };
        let instr: u16 = 0b0001_001_010_0_00011;
        op_add(&mut register, instr);
        assert_eq!(register[1], 3);
    }
    #[test]
    fn test_op_add_sign_ext_imm5() {
        let mut register = Register {
            reg: [0, 0, 2, 0, 0, 0, 0, 0, 0, 0],
        };
        let instr: u16 = 0b_0001_001_010_1_11111;
        op_add(&mut register, instr);
        assert_eq!(register[1], 0b0000000000000001);
    }
    #[test]
    fn test_pos_flag() {
        let mut register = Register {
            reg: [0, 0, 4, 3, 0, 0, 0, 0, 0, 0],
        };
        let instr: u16 = 0b0001_001_010_0_00011;
        op_add(&mut register, instr);
        assert_eq!(register[Reg::R_COND], 0b001);
    }
    #[test]
    fn test_neg_flag() {
        let mut register = Register {
            reg: [0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        };
        let instr: u16 = 0b0001_001_010_1_10011;
        op_add(&mut register, instr);
        assert_eq!(register[Reg::R_COND], 0b100);
    }
    #[test]
    fn test_zero_flag() {
        let mut register = Register {
            reg: [0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        };
        let instr: u16 = 0b0001_001_010_1_11111;
        op_add(&mut register, instr);
        assert_eq!(register[Reg::R_COND], 0b010);
    }

    #[test]
    fn test_op_ldi() {
        let pc_start: u16 = 0x3000; // set program start
        let mut reg: Register = Default::default(); // init regs
        reg[Reg::R_PC] = pc_start; // init regs
        let instr: u16 = 0b1010_001_000000001; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3001] = 0x3002; // indirect pointer
        memory[0x3002] = 10; // actual data to be loaded
        op_ldi(&mut reg, instr, &mut memory);
        assert_eq!(reg[1], 10, "testing register value");
        assert_eq!(reg[Reg::R_COND], 0b001, "testing positive flags");
    }

    #[test]
    fn test_op_ldi_flags() {
        let pc_start: u16 = 0x3000; // set program start
        let mut reg: Register = Default::default(); // init regs
        reg[Reg::R_PC] = pc_start; // init regs
        let instr: u16 = 0b1010_001_000000001; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3001] = 0x3002; // indirect pointer
        memory[0x3002] = 0b1111111111111101; // actual data to be loaded
        op_ldi(&mut reg, instr, &mut memory);
        assert_eq!(reg[Reg::R_COND], 0b100, "testing negative flags");

        memory[0x3002] = 0;
        op_ldi(&mut reg, instr, &mut memory);
        assert_eq!(reg[Reg::R_COND], 0b010, "testing zero flags");
    }
    #[test]
    fn test_op_ldr() {
        let mut register = Register {
            reg: [0, 0x3001, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        let instr: u16 = 0b0110_000_001_000011; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3004] = 10;
        op_ldr(&mut register, instr, &memory);
        assert_eq!(register[0], 10, "testing register value");
        assert_eq!(register[Reg::R_COND], 0b001, "testing positive flag");
    }

    #[test]
    fn test_op_ldr_neg_offset() {
        let mut register = Register {
            reg: [0, 0x3001, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        let instr: u16 = 0b0110_000_001_111111; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3000] = 10;
        op_ldr(&mut register, instr, &memory);
        assert_eq!(register[0], 10, "testing register value");
        assert_eq!(register[Reg::R_COND], 0b001, "testing positive flag");
    }

    #[test]
    fn test_op_ldr_flags() {
        let mut register = Register {
            reg: [0, 0x3001, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        let instr: u16 = 0b0110_000_001_111111; // define instruction
        let mut memory = Memory::new(65535); // declare memory
        memory[0x3000] = 0b1111111111111011;
        op_ldr(&mut register, instr, &memory);
        assert_eq!(register[Reg::R_COND], 0b100, "testing positive flag");
    }

    #[test]
    fn test_op_lea() {
        let mut register: Register = Default::default();
        let instr: u16 = 0b1110_001_000000011;
        register[Reg::R_PC] = 0x3000;
        op_lea(&mut register, instr);
        assert_eq!(register[1], 0x3003);
    }

    #[test]
    fn test_op_lea_neg_offset() {
        let mut register: Register = Default::default();
        let instr: u16 = 0b1110_001_111111111;
        register[Reg::R_PC] = 0x3001;
        op_lea(&mut register, instr);
        assert_eq!(register[1], 0x3000);
    }

    #[test]
    fn test_op_not() {
        let mut register: Register = Default::default();
        register[2] = 0b0101111000101011;
        let instr: u16 = 0b1100_001_010_111111;
        op_not(&mut register, instr);
        assert_eq!(register[1] ^ register[2], 0xffff);
    }

    #[test]
    fn test_op_br(){
        //0000 1 1 1 000000011
        let mut register = Register::default();
        register[Reg::R_PC] = 0x3000;                // current pos of PC
        register[Reg::R_COND] = 0b001;               // positive flag on
        let instr: u16 = 0b0000_1_1_1_000000011;     // offset of 3
        op_br(&mut register, instr);
        assert_eq!(register[Reg::R_PC], 0x3003, "positive offset");

        let instr: u16 = 0b0000_1_1_1_111111111;     // offset of -1
        op_br(&mut register, instr);
        assert_eq!(register[Reg::R_PC], 0x3002, "negative offset");

        let instr: u16 = 0b0000_0_0_0_111111111;     // turn off flags
        op_br(&mut register, instr);
        assert_eq!(register[Reg::R_PC], 0x3002, "flags turned off");
    }

    #[test]
    fn test_op_jmp(){
        let mut register = Register::default();
        let instr: u16 = 0b1100_000_001_000000;
        register[1] = 0x3500;
        register[Reg::R_PC] = 0x3001;
        op_jmp(&mut register, instr);

        assert_eq!(register[Reg::R_PC], 0x3500);
    }
}
