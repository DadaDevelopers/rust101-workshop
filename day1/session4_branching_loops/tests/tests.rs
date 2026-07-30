use session4_branching_loops::*;

#[test]
fn test_fee_tier() {
    assert_eq!(fee_tier(1), "low");
    assert_eq!(fee_tier(5), "medium");
    assert_eq!(fee_tier(10), "high");
    assert_eq!(fee_tier(50), "high");
}

#[test]
fn test_is_confirmed() {
    assert!(!is_confirmed(0));
    assert!(!is_confirmed(5));
    assert!(is_confirmed(6));
    assert!(is_confirmed(100));
}
