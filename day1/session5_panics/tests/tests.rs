use session5_panics::*;

#[test]
fn test_get_output() {
    assert_eq!(get_output(&[1000, 2000], 0), 1000);
    assert_eq!(get_output(&[1000, 2000], 1), 2000);
}

#[test]
#[should_panic]
fn test_get_output_out_of_bounds() {
    get_output(&[1000], 5);
}

#[test]
fn test_change() {
    assert_eq!(change(10_000, 8_000, 500), 1_500);
}

#[test]
#[should_panic]
fn test_change_overspend() {
    change(1_000, 900, 200);
}
