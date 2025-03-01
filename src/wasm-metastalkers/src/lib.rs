#[unsafe(no_mangle)]
pub extern "C" fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        match n.checked_mul(factorial(n - 1)) {
            Some(result) => result,
            None => 0, // Use 0 to represent overflow/None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        // Factorial of 0 should be 1
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_one() {
        // Factorial of 1 should be 1
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_factorial_small_number() {
        // Factorial of 5 should be 120
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_factorial_large_number() {
        // Factorial of 10 should be 3628800
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_factorial_recursive() {
        // Test recursive nature of the function
        assert_eq!(factorial(6), 720); // 6! = 720
        assert_eq!(factorial(7), 5040); // 7! = 5040
    }

    #[test]
    fn test_factorial_overflow() {
        // Test for overflow (u32 can only handle up to 12!)
        assert_eq!(factorial(13), 0); // 13! overflows u32
        assert_eq!(factorial(20), 0); // Any larger number should also return 0
    }

    #[test]
    fn test_factorial_edge_cases() {
        // Test edge cases
        assert_eq!(factorial(2), 2); // 2! = 2
        assert_eq!(factorial(3), 6); // 3! = 6
        assert_eq!(factorial(12), 479001600); // 12! is the largest u32 factorial
    }
}
