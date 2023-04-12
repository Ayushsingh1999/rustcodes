// Check if a string is a palindrome or not:
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
