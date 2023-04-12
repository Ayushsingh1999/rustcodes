// Check if a number is prime or not:
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
