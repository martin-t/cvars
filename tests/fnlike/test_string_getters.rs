mod shared;

use shared::Cvars;

fn main() {
    let cvars = Cvars::new();

    assert_eq!(cvars.get_string("g_bool"), "true");
    assert_eq!(cvars.get_string("g_int"), "42");
    assert_eq!(cvars.get_string("g_usize"), "987654");
    assert_eq!(cvars.get_string("g_float"), "5");
    assert_eq!(cvars.get_string("g_double"), "10");
    assert_eq!(cvars.get_string("g_enum"), "Two");
}
