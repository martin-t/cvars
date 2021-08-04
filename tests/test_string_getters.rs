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
    };

    assert_eq!(cvars.get_string("g_bool"), "true");
    assert_eq!(cvars.get_string("g_int"), "42");
    assert_eq!(cvars.get_string("g_usize"), "987654");
    assert_eq!(cvars.get_string("g_float"), "5.0");
    assert_eq!(cvars.get_string("g_double"), "10.0");
    assert_eq!(cvars.get_string("g_enum"), "Two");
}
