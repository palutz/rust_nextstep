///
/// Chapter 01 - Foundations
///
#[warn(dead_code)]

fn chapter_one() {
    let mut x;
    x = 42;

    let y = &x;

    // x = 43;

    assert_eq!(*y, 42);
}
