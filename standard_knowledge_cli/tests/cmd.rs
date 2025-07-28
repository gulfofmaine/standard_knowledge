#[test]
fn cli_tests() {
    // Get updated outputs with `TRYCMD=overwrite cargo test`
    let t = trycmd::TestCases::new();
    t.case("tests/cmd/*.toml");
}
