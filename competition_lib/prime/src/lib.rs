/// whether n is prime number
/// O( sqrt(n) )
pub fn is_prime(n: u64) -> bool {
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

/// return all of the number which can divide the input
/// O( sqrt(n) )
/// Example.
///   divisor(3)  ==  vec![1, 3]
///   divisor(24) ==  vec![1, 2, 3, 4, 6, 8, 12, 24]
pub fn divisor(n: u64) -> Vec<u64> {
    let mut ret = Vec::new();
    if n == 1 {
        ret.push(1);
        return ret;
    }

    for i in 1..n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            ret.push(i);
            let m = n / i;
            if m != i {
                ret.push(m);
            }
        }
    }

    ret
}

/// prime factorization
/// O( sqrt(n) )
/// Example.
///   prime_factorization(3)  ==  vec![3]
///   prime_factorization(24) ==  vec![1, 2, 3, 4, 6, 8, 12, 24]
pub fn prime_factorization(mut n: u64) -> Vec<u64> {
    let mut ret = Vec::new();
    for i in 2..n {
        if i * i > n {
            break;
        }
        while n % i == 0 {
            ret.push(i);
            n /= i;
        }
    }

    if n != 1 {
        ret.push(n);
    }

    ret
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
        assert!(!is_prime(295927));
        assert!(!is_prime(508369));
        assert!(!is_prime(48319935));
        assert!(is_prime(93208043));
    }

    #[test]
    fn test_divisor() {
        fn check(input: u64, ans: Vec<u64>) {
            let mut ret = divisor(input);
            ret.sort();
            assert_eq!(ret, ans);
        }
        check(1, vec![1]);
        check(2, vec![1, 2]);
        check(3, vec![1, 3]);
        check(4, vec![1, 2, 4]);
        check(24, vec![1, 2, 3, 4, 6, 8, 12, 24]);
        check(295927, vec![1, 541, 547, 295927]);
    }

    #[test]
    fn test_prime_factorization() {
        fn check(input: u64, ans: Vec<u64>) {
            let mut ret = prime_factorization(input);
            assert!(ret.iter().all(|&i| is_prime(i)));
            assert_eq!(ret.iter().fold(1, |x, y| x * y), input);

            ret.sort();
            assert_eq!(ret, ans);
        }
        check(1, vec![]);
        check(2, vec![2]);
        check(3, vec![3]);
        check(4, vec![2, 2]);
        check(6, vec![2, 3]);
        check(295927, vec![541, 547]);
        check(508369, vec![23, 23, 31, 31]);
        check(48319935, vec![3, 5, 41, 78569]);
        check(93208043, vec![93208043]);
    }
}
