use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


///
/// Add operation
/// 
/// instruction example
/// 0001 dr SR1 0 00 SR2 --> get the second value from SR2 register.
/// 0001 dr SR1 1 imm5   --> immediate mode, do DR1 + imm5 after sign extending.
///
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


#[cfg(test)]
mod tests {
    use super::*;

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
}
