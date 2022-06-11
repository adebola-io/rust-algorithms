/// Prompt: Write a rust function that reverses a given string.
/// ## Example
/// ```rust
/// assert(reverse_string("I am"), "ma I");
/// ```
pub fn reverse_string(input: &str) -> String {
    // First simulate all characters of the string into an iterator.
    // Reverse the iterator and collect the iterator values as a String.
    input.chars().rev().collect()
}

#[test]
fn it_reverses_string() {
    let input = "Hello from the other side.";
    let reversed = reverse_string(&input);
    assert_eq!(".edis rehto eht morf olleH", reversed);
}
