mod shared;

use shared::Cvars;

fn main() {
    let cvars = Cvars::new();

    // Creating a second struct so that type inferrence works.
    // Just `assert_eq!(cvars.get("g_int"), cvars.g_int);`
    // would require specifying types, same for using `==`.
    let other = Cvars {
        g_bool: cvars.get("g_bool").unwrap(),
        g_int: cvars.get("g_int").unwrap(),
        g_usize: cvars.get("g_usize").unwrap(),
        g_float: cvars.get("g_float").unwrap(),
        g_double: cvars.get("g_double").unwrap(),
        g_enum: cvars.get("g_enum").unwrap(),
        g_skipped: 0,
    };
    assert_eq!(other.g_bool, cvars.g_bool);
    assert_eq!(other.g_int, cvars.g_int);
    assert_eq!(other.g_usize, cvars.g_usize);
    assert_eq!(other.g_float, cvars.g_float);
    assert_eq!(other.g_double, cvars.g_double);
    assert_eq!(other.g_enum, cvars.g_enum);

    assert_eq!(cvars.get::<i32>("bla"), Err("Cvar named bla with type i32 not found".to_owned()));
}
