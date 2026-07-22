use session2_syntax::*;

#[test]
fn test_greet() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}

#[test]
fn test_string_length() {
    assert_eq!(string_length("rust"), 4);
    assert_eq!(string_length(""), 0);
}
