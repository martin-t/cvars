mod shared;

use shared::{Cvars, Enum};

fn main() {
    let mut cvars = Cvars::default();

    cvars.set_str("g_bool", "false").unwrap();
    cvars.set_str("g_int", "43").unwrap();
    cvars.set_str("g_usize", "987655").unwrap();
    cvars.set_str("g_float", "6.0").unwrap();
    cvars.set_str("g_double", "11.0").unwrap();
    cvars.set_str("g_enum", "one").unwrap();

    assert_eq!(cvars.g_bool, false);
    assert_eq!(cvars.g_int, 43);
    assert_eq!(cvars.g_usize, 987655);
    assert_eq!(cvars.g_float, 6.0);
    assert_eq!(cvars.g_double, 11.0);
    assert_eq!(cvars.g_enum, Enum::One);

    assert_eq!(
        cvars.set_str("bla", "666"),
        Err("Cvar named bla not found".to_owned())
    );
    assert_eq!(
        cvars.set_str("g_int", "not a num"),
        Err("failed to parse not a num as type i32: invalid digit found in string".to_owned())
    );
}
