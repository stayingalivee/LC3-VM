use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


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

#[cfg(test)]
mod tests {
    use super::*;
    
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
}
