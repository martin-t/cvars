mod shared;

use shared::Cvars;

fn main() {
    let mut cvars = Cvars::default();

    cvars.set_str("g_bool", "false").unwrap();
    assert_eq!(cvars.g_bool, false);
    cvars.set_str("g_bool", "true").unwrap();
    assert_eq!(cvars.g_bool, true);

    cvars.set_str("g_bool", "f").unwrap();
    assert_eq!(cvars.g_bool, false);
    cvars.set_str("g_bool", "t").unwrap();
    assert_eq!(cvars.g_bool, true);

    cvars.set_str("g_bool", "0").unwrap();
    assert_eq!(cvars.g_bool, false);
    cvars.set_str("g_bool", "1").unwrap();
    assert_eq!(cvars.g_bool, true);
}
