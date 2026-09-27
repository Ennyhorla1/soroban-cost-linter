#![warn(unnecessary_string_to_bytes)]

pub mod soroban_sdk {
    pub struct String;
    impl String {
        pub fn to_bytes(&self) -> Bytes {
            Bytes
        }
    }
    pub struct Bytes;
}
use soroban_sdk::String;

fn bad_to_bytes(s: String) {
    let _ = s.to_bytes(); //~ WARNING unnecessary String to Bytes conversion
}

/// Multiple conversions in the same function should each trigger a warning.
fn multiple_bad_to_bytes(s1: String, s2: String) {
    let _b1 = s1.to_bytes(); //~ WARNING unnecessary String to Bytes conversion
    let _b2 = s2.to_bytes(); //~ WARNING unnecessary String to Bytes conversion
}

#[test]
fn test_unnecessary_string_to_bytes_ui() {
    // Ensure test coverage and robust lint checking for unnecessary string to bytes conversion.
    let s = soroban_sdk::String;
    let _ = s.to_bytes();
}

/// Conversion inside a helper scope / block expression.
fn block_scoped_to_bytes(s: String) {
    let _ = {
        s.to_bytes() //~ WARNING unnecessary String to Bytes conversion
    };
}

#[allow(unnecessary_string_to_bytes)]
fn good_required_to_bytes(s: String) {
    // False positive: when an external trait strictly requires Bytes and cannot be changed
    takes_bytes(s.to_bytes());
}

fn takes_bytes(_b: soroban_sdk::Bytes) {}

fn main() {}
