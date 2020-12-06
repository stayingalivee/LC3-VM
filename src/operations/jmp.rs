use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


///
/// Jump
///
/// Instruction example
/// 1100 000 sr  000000
/// 1100 000 001 000000
///
/// The program unconditionally jumps to the location specified by the contents of
/// the base register. Bits [8:6] identify the base register.
///
/// The RET instruction is a special case of the JMP instruction.
/// The PC is loaded with the contents of R7, which contains the linkage
/// back to the instructionfollowing the subroutine call instruction.
pub fn op_jmp(reg: &mut Register, instr: u16) {
    let sr = (instr >> 6) & 0b111;
    reg[Reg::R_PC] = reg[sr];
}


#[cfg(test)]
mod tests {
    use super::*;

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
