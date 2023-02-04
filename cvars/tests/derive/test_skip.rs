mod shared;

use shared::Cvars;

fn main() {
    let mut cvars = Cvars::default();

    let old_value = cvars.g_skipped;
    assert_eq!(
        cvars.get::<i32>("g_skipped"),
        Err("Cvar named g_skipped with type i32 not found".to_owned())
    );
    assert_eq!(
        cvars.get_string("g_skipped"),
        Err("Cvar named g_skipped not found".to_owned())
    );
    assert_eq!(
        cvars.set("g_skipped", 1),
        Err("Cvar named g_skipped with type i32 not found".to_owned())
    );
    assert_eq!(
        cvars.set_str("g_skipped", "1"),
        Err("Cvar named g_skipped not found".to_owned())
    );
    aassert_eq!(cvars.g_skipped, old_value);
}
