#[test]
fn thing_derive_rejects_unsupported_fields() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/unsupported_field.rs");
}
