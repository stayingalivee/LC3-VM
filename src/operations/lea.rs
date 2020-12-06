use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


/// Load Effective Address
///
/// Instruction example
/// 1110  dr  offset
/// 1110  001 000000011
///
/// An address is computed by sign-extending bits [8:0] to 16 bits and adding this
/// value to the incremented PC. This address is loaded into DR. The condition
/// codes are set, based on whether the value loaded is negative, zero, or positive.
///
/// LEA R0, Target
pub fn op_lea(reg: &mut Register, instr: u16) {
    let dr = (instr >> 9) & 0b111; // get destination register
    let offset = sign_ext(instr & 0b111111111, 9); // sign extend and get offset
    reg[dr] = reg[Reg::R_PC].wrapping_add(offset); // do lea operation
    update_flags(reg, dr);
}


#[cfg(test)]
mod tests {
    use super::*;

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
}
