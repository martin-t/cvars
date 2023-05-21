// This is not a real example but a test file for debugging
// the macros using `cargo expand --package cvars --example testing-fnlike`.
// I used an example, not a bin (main.rs),
// so that it can use dev-dependencies
// to avoid having to put strum in normal dependencies.

use strum_macros::{Display, EnumString};

use cvars::cvars;

cvars! {
    g_bool: bool = false,
    g_int: i32 = 0,
    g_usize: usize = 0,
    g_float: f32 = 0.0,
    g_double: f64 = 0.0,
    g_enum: Enum = Enum::One,
    #[warn(clippy::pedantic)] // Testing that the field can have other attributes
    #[cvars(skip)]
    #[allow(clippy::pedantic)]
    g_skipped: i32 = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Enum {
    #[default]
    One,
    Two,
}

fn main() {}
