// This is not a real example but a test file for debugging
// the macros using `cargo expand --package cvars --example testing-fnlike`.
// I used an example, not a bin (main.rs),
// so that it can use dev-dependencies
// to avoid having to put strum in normal dependencies.

#![allow(unused_imports)]
use strum_macros::{Display, EnumString};

use cvars::cvars;

cvars! {
    g_whatever: f64 = 42.0,
}

fn main() {}
