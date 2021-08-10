#![allow(unused_imports)]

use strum_macros::{Display, EnumString};

use cvars::cvars;

type Option = ();
type Result = ();
type Box = ();
enum BadOption {
    Some(i32),
    None,
}
use BadOption::*;
enum BadResult {
    Ok(i32),
    Err(i32),
}
use BadResult::*;

cvars! {
    g_bool: bool = true,
    g_int: i32 = 42,
    g_usize: usize = 987654,
    g_float: f32 = 5.0,
    g_double: f64 = 10.0,
    g_enum: Enum = Enum::Two,
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
