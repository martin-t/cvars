#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    // For simplifity it would be nice to keep most tests exactly the same.
    // Verify with this command:
    // diff -r --exclude test_redefined.rs --exclude shared.rs cvars/tests/{derive,fnlike}

    t.pass("tests/derive/test_parse.rs");
    t.pass("tests/derive/test_extra_impl.rs");
    t.pass("tests/derive/test_getters.rs");
    t.pass("tests/derive/test_setters.rs");
    t.pass("tests/derive/test_string_getters.rs");
    t.pass("tests/derive/test_string_setters.rs");
    t.pass("tests/derive/test_dyn.rs");
    t.pass("tests/derive/test_redefined.rs");
    t.pass("tests/derive/test_skip.rs");

    t.pass("tests/fnlike/test_parse.rs");
    t.pass("tests/fnlike/test_extra_impl.rs");
    t.pass("tests/fnlike/test_getters.rs");
    t.pass("tests/fnlike/test_setters.rs");
    t.pass("tests/fnlike/test_string_getters.rs");
    t.pass("tests/fnlike/test_string_setters.rs");
    t.pass("tests/fnlike/test_dyn.rs");
    t.pass("tests/fnlike/test_redefined.rs");

    t.pass("tests/dummy.rs");
}
