// Return the index of the first occurrence of a given number in a sorted array:
fn first_occurrence(arr: &[i32], x: i32) -> Option<usize> {
    arr.iter().position(|&num| num == x)
}
