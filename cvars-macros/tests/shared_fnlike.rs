use strum_macros::{Display, EnumString};

use cvars::cvars;

cvars! {
    //! Inner doc comment - passed to macros as an attribute
    #![derive(Debug, Clone)]
    g_bool: bool = true,
    g_int: i32 = 42,
    g_usize: usize = 987654,
    // Comment - should not affect macros
    g_float: f32 = 5.0,
    /// Doc comment - passed to macros as an attribute
    g_double: f64 = 10.0,
    g_enum: Enum = Enum::Two,
    #[warn(clippy::pedantic)] // Testing that the field can have other attributes
    #[cvars(skip)]
    #[allow(clippy::pedantic)]
    g_skipped: i32 = 666,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Enum {
    #[default]
    One,
    Two,
}
