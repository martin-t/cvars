// This is useful for debugging the macros.
// Put some code here, then run `cargo expand --example testing`.
// This is an example, not a bin, so that it can use dev-dependencies.

use strum_macros::{Display, EnumString};

use cvars::SetGet;

#[derive(Debug, Clone, Default, SetGet)]
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

fn main() {}
