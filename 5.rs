// Return the median of a sorted array:
fn median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();
    if len == 0 {
        return None;
    }
    if len % 2 == 0 {
        let mid = len / 2;
        Some((arr[mid - 1] + arr[mid]) as f64 / 2.0)
    } else {
        Some(arr[len / 2] as f64)
    }
}
