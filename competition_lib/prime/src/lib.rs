/// whether n is prime number or not
/// O( sqrt(n) )
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(508369));
        assert!(!is_prime(48319935));
        assert!(is_prime(93208043));
    }
}
