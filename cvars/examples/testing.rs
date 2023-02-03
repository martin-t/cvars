// This is not a real example but a test file for debugging
// the macros using `cargo expand --example testing`.
// I used an example, not a bin (main.rs),
// so that it can use dev-dependencies
// to avoid having to put strum in normal dependencies.

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
    #[warn(clippy::pedantic)] // Testing that the field can have other attributes
    #[cvars(skip)]
    #[allow(clippy::pedantic)]
    pub g_skipped: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Enum {
    #[default]
    One,
    Two,
}

fn main() {}
