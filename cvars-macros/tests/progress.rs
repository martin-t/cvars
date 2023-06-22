use std::fs;

#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/dummy.rs");

    // Run all test_* files for both derive and fnlike macros.
    // The files need to be copied to different directories because of how trybuild works.
    fs::create_dir_all("tests/derive").unwrap();
    fs::copy("tests/shared_derive.rs", "tests/derive/shared.rs").unwrap();
    for entry in fs::read_dir("tests/shared").unwrap() {
        let path_src = entry.unwrap().path();
        let path_dest = path_src.to_str().unwrap().replace("shared", "derive");
        fs::copy(&path_src, &path_dest).unwrap();
        t.pass(&path_dest);
    }
    fs::create_dir_all("tests/fnlike").unwrap();
    fs::copy("tests/shared_fnlike.rs", "tests/fnlike/shared.rs").unwrap();
    for entry in fs::read_dir("tests/shared").unwrap() {
        let path_src = entry.unwrap().path();
        let path_dest = path_src.to_str().unwrap().replace("shared", "fnlike");
        fs::copy(&path_src, &path_dest).unwrap();
        t.pass(&path_dest);
    }

    drop(t);

    // The tests run on drop so we only clean up after that.
    // The cleanup is only to declutter the workspace,
    // the test should work even if the files are left there from a previous run.
    fs::remove_dir_all("tests/derive").unwrap();
    fs::remove_dir_all("tests/fnlike").unwrap();
}
