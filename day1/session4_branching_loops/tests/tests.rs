use session4_branching_loops::*;

#[test]
fn test_fee_tier() {
    assert_eq!(fee_tier(1), "low");
    assert_eq!(fee_tier(5), "medium");
    assert_eq!(fee_tier(10), "high");
    assert_eq!(fee_tier(50), "high");
}

#[test]
fn test_total_value() {
    assert_eq!(total_value(&[]), 0);
    assert_eq!(total_value(&[1000, 2000, 3000]), 6000);
}
