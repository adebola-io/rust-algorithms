/// ### Prompt
/// Given an array of strings, find the longest substring prefix that is common to all members.
/// ### Solution
/// We first get all the characters in the first string, which we will use to check against all others.
/// If there is no first string . i.e. the input is empty, then we return a new string.
/// We then map through the array of strings and find the index of each string at which the characters are no longer equal to the first string.
/// The result is a vector of numbers, which indicate how close the start of each string is to the first string. All we have to do is get the lowest index.
/// We then slice the first string to our resulting index and return the value.
/// ## Example
/// ```rust
/// use rust_algorithms::longest_common_prefix;
/// let input = ["animal", "antelope", "anthem", "anecdote"];
/// assert_eq!(longest_common_prefix(&input), "an");
/// ```
pub fn longest_common_prefix(input: &[&str]) -> String {
    let first_string: Vec<char>;
    if input.len() > 0 {
        first_string = String::from(input[0]).chars().collect();
    } else {
        first_string = String::new().chars().collect();
    }
    // Compares a string to the first string to find out the index at which they stop being similar.
    let index_comparer = |f: &&str| match f
        .chars()
        .enumerate()
        .find(|_f| Some(&_f.1) != first_string.get(_f.0))
    {
        Some(index) => index.0,
        None => f.len(),
    };
    let lowest_index = match input.into_iter().map(index_comparer).min() {
        Some(val) => val,
        None => 0,
    };
    let common_prefix = match input.get(0) {
        Some(val) => String::from(&val[0..lowest_index]),
        None => String::new(),
    };
    common_prefix
}

#[test]
fn it_finds_longest_common_prefix() {
    assert_eq!(longest_common_prefix(&["barrow", "bard", "barrel"]), "bar");
    assert_eq!(longest_common_prefix(&["s", "s", ""]), "");
    assert_eq!(longest_common_prefix(&[]), "");
}
