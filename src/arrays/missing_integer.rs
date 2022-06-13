/// ### Prompt
/// Given an array of unsorted numbers, find the missing number in the sorted sequence. Return -1 if no number is missing.
/// ### Solution
/// - Convert the input integer array into a mutable vector so that it can be sorted.
/// - Iterate over and enumerate each integer in the array.
/// - Find the integer in the array, for which its predecessor is not a direct decrement by 1.
/// - Subtract 1 for the number and return it.
/// - If no number is found, return -1.
/// ## Example
/// ```rust
/// use rust_algorithms::missing_integer;
/// assert_eq!(missing_integer(&[1, 3, 4]), 2);
/// assert_eq!(missing_integer(&[2, 3, 4]), -1);
/// ```
pub fn missing_integer(input: &[i32]) -> i32 {
    let mut input: Vec<i32> = input.to_vec();
    input.sort();
    match input
        .iter()
        .enumerate()
        .find(|f: &(usize, &i32)| f.0 != 0 && Some(&(f.1 - 1)) != input.get(f.0 - 1))
    {
        Some(val) => val.1 - 1,
        None => -1,
    }
}

#[test]
fn it_finds_missing_integer() {
    assert_eq!(missing_integer(&[1, 6, 4, 5, 3]), 2);
}
