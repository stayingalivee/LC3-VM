use crate::defs::memory::*;
use crate::defs::register::*;
use crate::operations::helper::*;


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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_not() {
        let mut register: Register = Default::default();
        register[2] = 0b0101111000101011;
        let instr: u16 = 0b1100_001_010_111111;
        op_not(&mut register, instr);
        assert_eq!(register[1] ^ register[2], 0xffff);
    }

}
