
#[allow(non_camel_case_types)]
pub enum Cond_flags{
    FL_POS = 1,      // Positive
    FL_ZRO = 1 << 1, // Zero
    FL_NEG = 1 << 2, // Negative
}