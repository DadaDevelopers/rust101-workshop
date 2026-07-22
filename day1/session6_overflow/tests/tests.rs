use session6_overflow::*;

#[test]
fn test_safe_add() {
    assert_eq!(safe_add(100, 200), Some(300));
    assert_eq!(safe_add(u64::MAX, 1), None);
}

#[test]
fn test_fee_amount() {
    assert_eq!(fee_amount(10, 250), Some(2500));
    assert_eq!(fee_amount(u64::MAX, 2), None);
}
