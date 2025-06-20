#[test]
fn build_migration() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/compile.rs");
}

