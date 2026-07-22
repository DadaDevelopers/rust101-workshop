use session11_stack_heap::*;

#[test]
fn test_boxed_double() {
    assert_eq!(*boxed_double(5), 10);
}

#[test]
fn test_double_vec() {
    assert_eq!(double_vec(vec![1, 2, 3]), vec![2, 4, 6]);
}
