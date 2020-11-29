
#[allow(non_camel_case_types)]
pub enum Traps{
    TRAP_GETC  = 32,    /* get char from keybaord */
    TRAP_OUT   = 33,    /* output a char */
    TRAP_PUTS  = 34,    /* output a word string */
    TRAP_IN    = 35,    /* get char from keyboard */
    TRAP_PUTSP = 36,    /* output a byte string */
    TRAP_HALT  = 37,    /* halt the program */
}

impl Traps {
    pub fn from_u16(value: u16) -> Self {
        match value {
            32 => Self::TRAP_GETC,
            33 => Self::TRAP_OUT,
            34 => Self::TRAP_PUTS,
            35 => Self::TRAP_IN,
            36 => Self::TRAP_PUTSP,
            _ => Self::TRAP_HALT,
        }
    }
}
