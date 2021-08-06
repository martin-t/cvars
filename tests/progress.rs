#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/fnlike/test_parse.rs");
    t.pass("tests/derive/test_parse.rs");
    t.pass("tests/derive/test_getters.rs");
    t.pass("tests/derive/test_setters.rs");
    t.pass("tests/derive/test_string_getters.rs");
    t.pass("tests/derive/test_string_setters.rs");
}
