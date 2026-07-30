use session5_panics::*;

#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10, 2), 5);
}

#[test]
#[should_panic(expected = "Cannot divide by zero!")]
fn test_safe_divide_panics() {
    safe_divide(10, 0);
}

#[test]
fn test_first_fee() {
    assert_eq!(first_fee(&[250, 180, 320]), 250);
}

#[test]
#[should_panic]
fn test_first_fee_panics_on_empty() {
    first_fee(&[]);
}
