use session10_modules::{math, text};

#[test]
fn test_gcd() {
    assert_eq!(math::gcd(48, 18), 6);
    assert_eq!(math::gcd(7, 3), 1);
}

#[test]
fn test_lcm() {
    assert_eq!(math::lcm(4, 6), 12);
}

#[test]
fn test_reverse_words() {
    assert_eq!(text::reverse_words("hello world"), "world hello");
    assert_eq!(text::reverse_words("one"), "one");
}
