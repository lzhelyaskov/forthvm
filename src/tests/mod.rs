use crate::align;

mod forthvm_tests;
mod word_tests;

#[test]
fn test_align() {
    let r = align(1);
    assert_eq!(r, 4);

    let r = align(4);
    assert_eq!(r, 4);

    let r = align(6);
    assert_eq!(r, 8);

    let r = align(7);
    assert_eq!(r, 8)
}
