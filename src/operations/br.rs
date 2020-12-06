use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


/// Conditional Branch.
///
/// Instruction example
/// 0000 n z p   offset
/// 0000 1 1 1 000000011
///
/// The condition codes specified by the state of bits [11:9] are tested. If bit [11] is set,
/// N is tested; if bit [11] is clear, N is not tested. If bit [10] is set, Z is tested, etc.
/// If any of the condition codes tested is set, the program branches to the location
/// specified by adding the sign-extended PCoffset9 field to the incremented PC.
pub fn op_br(reg: &mut Register, instr: u16) {
    let offset = sign_ext(instr & 0b111111111, 9);             // get offset to be incremented
    let flags = (instr >> 9) & 0b111;                          // get the nzp flag

    if flags & reg[Reg::R_COND] != 0 {                         // if bitwise add doesn't produce
        reg[Reg::R_PC] = reg[Reg::R_PC].wrapping_add(offset);  // an on bit then all flags are 0
    }                                                          // in which case condition isn't met
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
