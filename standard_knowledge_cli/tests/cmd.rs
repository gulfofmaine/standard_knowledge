#[test]
fn cli_tests() {
    // Get updated outputs with `TRYCMD=dump cargo test`
    let t = trycmd::TestCases::new();
    // let cli_bin_path = trycmd::cargo::cargo_bin("standard_knowledge_cli");
    // let cli_bin_path = trycmd::cargo_bin!("standard_knowledge");
    // t.register_bin("standard_knowledge", cli_bin_path);
    let bin_path = trycmd::cargo::cargo_bin("standard_knowledge");
    t.register_bin("standard_knowledge", bin_path);
    // panic!("{:?}", bin);
    t.case("tests/cmd/*.toml");
    // trycmd::TestCases::new().case("tests/cmd/*.toml");
}
