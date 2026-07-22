use session3_integers_variables::*;

#[test]
fn test_btc_to_sats_constant() {
    assert_eq!(BTC_TO_SATS, 100_000_000);
}

#[test]
fn test_btc_to_sats() {
    assert_eq!(btc_to_sats(1.0), 100_000_000);
    assert_eq!(btc_to_sats(0.5), 50_000_000);
}

#[test]
fn test_is_above_dust() {
    assert!(!is_above_dust(546));
    assert!(is_above_dust(547));
    assert!(!is_above_dust(0));
}
