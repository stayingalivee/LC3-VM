
#[allow(non_camel_case_types)]
pub enum Cond_flags{
    FL_POS = 1,      // Positive -> 001
    FL_ZRO = 1 << 1, // Zero     -> 010
    FL_NEG = 1 << 2, // Negative -> 100
}