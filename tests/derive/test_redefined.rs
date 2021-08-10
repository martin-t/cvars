#![allow(unused_imports)]

use strum_macros::{Display, EnumString};

use cvars::SetGet;

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

fn main() {
    let mut cvars = Cvars::default();
    let _: bool = cvars.get("g_bool").unwrap();
    cvars.get_string("g_bool").unwrap();
    cvars.set("g_bool", true);
    cvars.set_str("g_bool", "true");
}
