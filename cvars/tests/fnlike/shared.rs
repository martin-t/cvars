use strum_macros::{Display, EnumString};

use cvars::cvars;

cvars! {
    g_bool: bool = true,
    g_int: i32 = 42,
    g_usize: usize = 987654,
    g_float: f32 = 5.0,
    g_double: f64 = 10.0,
    g_enum: Enum = Enum::Two,
    g_skipped: i32 = 0,
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
