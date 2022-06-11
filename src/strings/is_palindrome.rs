/// Checks if a given string is a palindrome. i.e. if it has the same order when it is reversed.
/// ## Example
/// ```rust
/// use rust_algorithms::is_palindrome;
///
/// assert!(is_palindrome("hannah"));
/// ```
pub fn is_palindrome(input: &str) -> bool {
    input.chars().rev().collect::<String>() == input
}

#[test]
fn it_tests_for_palindrome() {
    assert!(is_palindrome("ababa") && !is_palindrome("adesola"));
}
