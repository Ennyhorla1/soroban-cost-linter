use crate::unnecessary_string_to_bytes;

#[test] 
fn test_unnecessary_string_to_bytes_metadata() {
    // Unit tests covering edge cases and error paths for unnecessary_string_to_bytes lint
    assert_eq!(unnecessary_string_to_bytes::UNNECESSARY_STRING_TO_BYTES.name, "unnecessary_string_to_bytes");
}
