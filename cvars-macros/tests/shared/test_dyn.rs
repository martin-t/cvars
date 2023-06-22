mod shared;

use cvars::SetGet;

use shared::Cvars;

fn main() {
    let mut cvars = Cvars::default();
    take_dyn(&mut cvars);
}

fn take_dyn(cvars: &mut dyn SetGet) {
    cvars.set_str("g_bool", "false").unwrap();
    cvars.set_str("g_int", "43").unwrap();
    cvars.set_str("g_usize", "987655").unwrap();
    cvars.set_str("g_float", "6.0").unwrap();
    cvars.set_str("g_double", "11.0").unwrap();
    cvars.set_str("g_enum", "one").unwrap();

    assert_eq!(
        cvars.set_str("bla", "666"),
        Err("Cvar named bla not found".to_owned())
    );
    assert_eq!(
        cvars.set_str("g_int", "not a num"),
        Err("failed to parse not a num as type i32: invalid digit found in string".to_owned())
    );

    assert_eq!(cvars.get_string("g_bool"), Ok("false".to_owned()));
    assert_eq!(cvars.get_string("g_int"), Ok("43".to_owned()));
    assert_eq!(cvars.get_string("g_usize"), Ok("987655".to_owned()));
    assert_eq!(cvars.get_string("g_float"), Ok("6".to_owned()));
    assert_eq!(cvars.get_string("g_double"), Ok("11".to_owned()));
    assert_eq!(cvars.get_string("g_enum"), Ok("One".to_owned()));

    assert_eq!(
        cvars.get_string("bla"),
        Err("Cvar named bla not found".to_owned())
    );
}
