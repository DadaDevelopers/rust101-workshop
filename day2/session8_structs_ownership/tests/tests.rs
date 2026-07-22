use session8_structs_ownership::Rectangle;

#[test]
fn test_area() {
    let r = Rectangle::new(4.0, 5.0);
    assert_eq!(r.area(), 20.0);
}

#[test]
fn test_can_hold() {
    let big = Rectangle::new(10.0, 10.0);
    let small = Rectangle::new(5.0, 5.0);
    assert!(big.can_hold(&small));
    assert!(!small.can_hold(&big));
}
