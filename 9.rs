// reverse a string
fn reverse_string(s: &mut String) {
    let len = s.len();
    let mid = len / 2;
    let bytes = s.as_bytes();
    for i in 0..mid {
        let j = len - 1 - i;
        let tmp = bytes[i];
        s.as_bytes_mut()[i] = bytes[j];
        s.as_bytes_mut()[j] = tmp;
    }
}
