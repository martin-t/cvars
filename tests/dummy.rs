use strum_macros::{Display, EnumString};

use cvars::SetGetDummy;

#[derive(Debug, Clone, Default, SetGetDummy)]
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

// Just test that it generates the getters and setters but don't run it.
// Trybuild doesn't offer a better way to just assert the code compiles without runnin it.
// I also tried catch_unwind but it still ended up producing ugly warnings.
fn unused() {
    let mut cvars = Cvars::default();
    let _ = cvars.set("g_bool", true);
    let _ = cvars.get::<bool>("g_bool");
    let _ = cvars.set_str("g_bool", "true");
    let _ = cvars.get_string("g_bool");
}

fn main() {}
