mod shared;

use shared::{Cvars, Enum};

fn main() {
    let mut cvars = Cvars::default();

    cvars.set("g_bool", true).unwrap();
    cvars.set("g_int", 42).unwrap();
    cvars.set("g_usize", 987654_usize).unwrap();
    cvars.set("g_float", 5.0_f32).unwrap();
    cvars.set("g_double", 10.0).unwrap();
    cvars.set("g_enum", Enum::Two).unwrap();

    assert_eq!(cvars.g_bool, true);
    assert_eq!(cvars.g_int, 42);
    assert_eq!(cvars.g_usize, 987654);
    assert_eq!(cvars.g_float, 5.0);
    assert_eq!(cvars.g_double, 10.0);
    assert_eq!(cvars.g_enum, Enum::Two);

    assert_eq!(cvars.set("bla", 666), Err("Cvar named bla with type i32 not found".to_owned()));
}
