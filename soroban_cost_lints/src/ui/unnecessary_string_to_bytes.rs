use super::unnecessary_string_to_bytes;

#[test]
fn test_unnecessary_string_to_bytes_lint_struct() {
    let lint = unnecessary_string_to_bytes();
    assert!(!lint.name.is_empty());
    assert!(!lint.desc.is_empty());
}

#[test]
fn test_unnecessary_string_to_bytes_properties() {
    let lint = unnecessary_string_to_bytes();
    // Verify basic metadata of the lint to ensure correctness
    assert_eq!(lint.name, "unnecessary_string_to_bytes");
}
