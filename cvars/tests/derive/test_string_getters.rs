mod shared;

use shared::{Cvars, Enum};

fn main() {
    let cvars = Cvars {
        g_bool: true,
        g_int: 42,
        g_usize: 987654,
        g_float: 5.0,
        g_double: 10.0,
        g_enum: Enum::Two,
        g_skipped: 0,
    };

    assert_eq!(cvars.get_string("g_bool"), Ok("true".to_owned()));
    assert_eq!(cvars.get_string("g_int"), Ok("42".to_owned()));
    assert_eq!(cvars.get_string("g_usize"), Ok("987654".to_owned()));
    assert_eq!(cvars.get_string("g_float"), Ok("5".to_owned()));
    assert_eq!(cvars.get_string("g_double"), Ok("10".to_owned()));
    assert_eq!(cvars.get_string("g_enum"), Ok("Two".to_owned()));

    assert_eq!(cvars.get_string("bla"), Err("Cvar named bla not found".to_owned()));
}
