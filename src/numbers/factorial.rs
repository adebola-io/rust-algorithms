/// Get the factorial of a number.
/// ## Example
/// ```rust
/// use rust_algorithms::factorial;
/// assert_eq!(factorial(10), 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1);
/// ```
pub fn factorial(input: usize) -> usize {
    if input == 0 {
        1
    } else {
        input * factorial(input - 1)
    }
}

#[test]
fn it_gets_factorial() {
    assert_eq!(factorial(3), 3 * 2 * 1);
}
