/// #### Prompt
/// Write a rust function that reverses a given string.
/// #### Solution
/// First simulate all characters of the string into an iterator.
/// Reverse the iterator and collect the iterator values as a String.
/// ## Example
/// ```rust
/// use rust_algorithms::reverse_string;
/// assert_eq!(reverse_string("I am"), "ma I");
/// ```
pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

#[test]
fn it_reverses_string() {
    assert_eq!(
        reverse_string("Hello from the other side."),
        ".edis rehto eht morf olleH"
    );
}
