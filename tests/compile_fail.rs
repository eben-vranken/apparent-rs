#[test]
fn frame_mismatches_do_not_compile() {
    trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
}
