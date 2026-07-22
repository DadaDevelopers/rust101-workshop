#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooShort,
    TooLong,
    InvalidCharacter(char),
}

/// Validates a username: 3–20 characters, alphanumeric + underscore only.
/// Returns Ok(()) on success or Err(ValidationError) describing the first problem.
/// TODO: implement this function.
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    todo!()
}
