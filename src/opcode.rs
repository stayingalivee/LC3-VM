
#[allow(non_camel_case_types)]
pub enum Opcode {
    OP_BR  = 0,    // branch
    OP_ADD = 1,    // add 
    OP_LD  = 2,    // load
    OP_ST  = 3,    // store
    OP_JSR = 4,    // jump register
    OP_AND = 5,    // bitwise and
    OP_LDR = 6,    // load register
    OP_STR = 7,    // store register
    OP_RTI = 8,    // unused
    OP_NOT = 9,    // bitwise not
    OP_LDI = 10,   // load indirect
    OP_STI = 11,   // store indirect
    OP_JMP = 12,   // jump
    OP_RES = 13,   // reserved (unused)
    OP_LEA = 14,   // load effective address
    OP_TRAP = 15    // execute trap
}


impl Opcode {
    pub fn from_i16(value: i16) -> Self {
        match value {
            0   => Self::OP_BR,
            1   => Self::OP_ADD,
            2   => Self::OP_LD,
            3   => Self::OP_ST,
            4   => Self::OP_JSR,
            5   => Self::OP_AND,
            6   => Self::OP_LDR,
            7   => Self::OP_STR,
            8   => Self::OP_RTI,
            9   => Self::OP_NOT,
            10  => Self::OP_LDI,
            11  => Self::OP_STI,
            12  => Self::OP_JMP,
            13  => Self::OP_RES,
            14  => Self::OP_LEA,
            15  => Self::OP_TRAP,
        }
    }
}