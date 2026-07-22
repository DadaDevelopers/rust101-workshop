use session9_validation::{validate_username, ValidationError};

#[test]
fn test_valid_username() {
    assert_eq!(validate_username("alice_42"), Ok(()));
}

#[test]
fn test_too_short() {
    assert_eq!(validate_username("ab"), Err(ValidationError::TooShort));
}

#[test]
fn test_too_long() {
    assert_eq!(validate_username(&"a".repeat(21)), Err(ValidationError::TooLong));
}

#[test]
fn test_invalid_char() {
    assert_eq!(validate_username("bad name"), Err(ValidationError::InvalidCharacter(' ')));
}
