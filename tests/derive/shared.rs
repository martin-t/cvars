use strum_macros::{Display, EnumString};

use cvars::Cvars;

#[derive(Debug, Clone, Default, Cvars)]
pub struct Cvars {
    pub g_bool: bool,
    pub g_int: i32,
    pub g_usize: usize,
    pub g_float: f32,
    pub g_double: f64,
    pub g_enum: Enum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Enum {
    One,
    Two,
}

impl Default for Enum {
    fn default() -> Self {
        Enum::One
    }
}
