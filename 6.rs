// Find the longest common prefix of a set of strings:
fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    if strs.is_empty() {
        return prefix;
    }
    for i in 0..strs[0].len() {
        let c = strs[0].as_bytes()[i];
        if strs.iter().any(|s| s.as_bytes().get(i) != Some(&c)) {
            break;
        }
        prefix.push(c as char);
    }
    prefix
}
