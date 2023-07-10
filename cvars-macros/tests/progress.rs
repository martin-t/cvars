use std::fs;

use trybuild::TestCases;

#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/test_dummy.rs");
    t.pass("tests/test_sorted_good.rs");
    t.compile_fail("tests/test_sorted_bad.rs");

    // Run all test_* files for both derive and fnlike macros.
    // The files need to be copied to different directories because of how trybuild works.
    run_shared_tests("derive", &t);
    run_shared_tests("fnlike", &t);

    drop(t);

    // The tests run on drop so we only clean up after that.
    // The cleanup is only to declutter the workspace,
    // the test should work even if the files are left there from a previous run.
    // Doesn't run if tests fail (TestCases::drop panics).
    fs::remove_dir_all("tests/derive").unwrap();
    fs::remove_dir_all("tests/fnlike").unwrap();
}

fn run_shared_tests(kind: &str, t: &TestCases) {
    fs::create_dir_all(format!("tests/{kind}")).unwrap();
    fs::copy(
        format!("tests/shared_{kind}.rs"),
        format!("tests/{kind}/shared.rs"),
    )
    .unwrap();
    for entry in fs::read_dir("tests/shared").unwrap() {
        let path_src = entry.unwrap().path();
        let path_dest = path_src.to_str().unwrap().replace("shared", kind);
        fs::copy(&path_src, &path_dest).unwrap();
        t.pass(&path_dest);
    }
}
