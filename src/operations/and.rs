use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


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


#[cfg(test)]
mod tests {
    use super::*;

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
}
