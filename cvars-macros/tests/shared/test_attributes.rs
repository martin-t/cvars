mod shared;

use shared::Cvars;

fn main() {
    let cvars = Cvars::default();

    let cvars2 = cvars.clone();

    let expected = r#"Cvars {
    g_bool: true,
    g_int: 42,
    g_usize: 987654,
    g_float: 5.0,
    g_double: 10.0,
    g_enum: Two,
    g_string: "String",
    g_skipped: 666,
}"#;
    assert_eq!(format!("{:#?}", cvars2), expected);
}
