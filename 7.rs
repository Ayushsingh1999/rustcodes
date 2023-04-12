// Return the kth smallest element in an array:
fn kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
    arr.sort();
    arr.get(k - 1).cloned()
}

