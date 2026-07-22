use session7_capstone::*;

#[test]
fn test_stats() {
    let (min, max, mean) = stats(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(min, 1.0);
    assert_eq!(max, 5.0);
    assert_eq!(mean, 3.0);
}

#[test]
#[should_panic]
fn test_stats_empty() {
    stats(&[]);
}

#[test]
fn test_is_prime() {
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(7));
    assert!(!is_prime(9));
    assert!(is_prime(97));
}
