extern crate rust_study;
mod common;

#[test]
fn add_two_test_test_test() {
    common::setup();
    assert_eq!(4, rust_study::add_two(2));
}
