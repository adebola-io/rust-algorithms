use std::hash::Hash;

/// ### Prompt
/// Write a function that removes the duplicate items in the array.
/// ### Solution
/// ## Example
/// ```rust
/// use rust_algorithms::remove_duplicates;
/// assert_eq!(remove_duplicates(&["foo", "foo", "bar", "bar"]), vec!["foo", "bar"]);
/// ```
pub fn remove_duplicates<T: Eq + Clone + Hash>(input: &[T]) -> Vec<T> {
    let mut map = std::collections::HashMap::new();
    let mut output = vec![];
    for val in input {
        if map.get(val) == None {
            output.push(val.clone());
            map.insert(val, true);
        }
    }
    output
}

#[test]
fn it_remove_integer_duplicates() {
    assert_eq!(remove_duplicates(&[1, 2, 3, 4, 5, 5]), vec![1, 2, 3, 4, 5]);
}
