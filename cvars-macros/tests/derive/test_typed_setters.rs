mod shared;

use shared::{Cvars, Enum};

fn main() {
    let mut cvars = Cvars::default();

    cvars.set("g_bool", false).unwrap();
    cvars.set("g_int", 43).unwrap();
    cvars.set("g_usize", 987655_usize).unwrap();
    cvars.set("g_float", 6.0_f32).unwrap();
    cvars.set("g_double", 11.0).unwrap();
    cvars.set("g_enum", Enum::One).unwrap();

    assert_eq!(cvars.g_bool, false);
    assert_eq!(cvars.g_int, 43);
    assert_eq!(cvars.g_usize, 987655);
    assert_eq!(cvars.g_float, 6.0);
    assert_eq!(cvars.g_double, 11.0);
    assert_eq!(cvars.g_enum, Enum::One);

    assert_eq!(
        cvars.set("bla", 666),
        Err("Cvar named bla with type i32 not found".to_owned())
    );
}
