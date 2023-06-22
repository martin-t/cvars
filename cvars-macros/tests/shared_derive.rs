use strum_macros::{Display, EnumString};

use cvars::SetGet;

#[derive(Debug, Clone, SetGet)]
pub struct Cvars {
    pub g_bool: bool,
    pub g_int: i32,
    pub g_usize: usize,
    // Comment - should not affect macros
    pub g_float: f32,
    /// Doc comment - passed to macros as an attribute
    pub g_double: f64,
    pub g_enum: Enum,
    #[warn(clippy::pedantic)] // Testing that the field can have other attributes
    #[cvars(skip)]
    #[allow(clippy::pedantic)]
    pub g_skipped: i32,
}

impl Default for Cvars {
    fn default() -> Self {
        Self {
            g_bool: true,
            g_int: 42,
            g_usize: 987654,
            g_float: 5.0,
            g_double: 10.0,
            g_enum: Enum::Two,
            g_skipped: 666,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Enum {
    #[default]
    One,
    Two,
}
