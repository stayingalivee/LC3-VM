use crate::defs::cond_flags::Cond_flags;
use crate::defs::register::*;


pub fn sign_ext(mut val: u16, bit_count: i16) -> u16 {
    if (val >> (bit_count - 1)) & 1 == 1 {
        val = val | 0xffff << bit_count;
    }
    return val;
}

pub fn update_flags(reg: &mut Register, dr: u16) {
    if reg[dr] == 0 {
        reg[Reg::R_COND] = Cond_flags::FL_ZRO as u16;
    } else if (reg[dr] >> 15) == 1 {
        reg[Reg::R_COND] = Cond_flags::FL_NEG as u16;
    } else {
        reg[Reg::R_COND] = Cond_flags::FL_POS as u16;
    }
}