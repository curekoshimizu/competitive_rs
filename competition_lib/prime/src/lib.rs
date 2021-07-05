use std::collections::HashSet;

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

/// Sieve of Eratosthenes
/// O( n log log n ) to make is_prime table
/// Example.
///   prime_factorization(3)  ==  vec![3]
///   prime_factorization(24) ==  vec![1, 2, 3, 4, 6, 8, 12, 24]
pub struct SieveEratosthenes {
    pub is_prime: Vec<bool>,
}

impl SieveEratosthenes {
    pub fn new(n: usize) -> SieveEratosthenes {
        // solver
        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..=n {
            if is_prime[i] {
                let mut j = 2 * i;
                while j <= n {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }

        return SieveEratosthenes { is_prime };
    }
}

/// m^n % modulo
/// O(log n)
pub fn mod_power(m: u64, mut n: u64, modulo: u64) -> u64 {
    let mut ans: u64 = 1;

    let mut cur: u64 = m;

    while n != 0 {
        if n & 1 == 1 {
            ans = (ans * cur) % modulo;
        }

        cur = (cur * cur) % modulo;
        n >>= 1;
    }

    ans
}

/// m^n
/// O(log n)
pub fn power(m: u64, mut n: u64) -> u64 {
    let mut ans: u64 = 1;

    let mut cur: u64 = m;

    while n != 0 {
        if n & 1 == 1 {
            ans *= cur;
        }

        cur *= cur;
        n >>= 1;
    }

    ans
}

// phi(n)
// O( n log(n) )
pub fn euler_phi(n: u64) -> u64 {
    let primes = prime_factorization(n);
    let set = primes.iter().collect::<HashSet<_>>();

    let mut ans = n;
    for &p in set.iter() {
        ans = ans / p * (p - 1);
    }

    ans
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

    #[test]
    fn test_sieve_of_eratothenes() {
        let ret = SieveEratosthenes::new(100);
        assert!(ret.is_prime[2]);
        assert!(ret.is_prime[3]);
        assert!(!ret.is_prime[4]);
        assert!(ret.is_prime[5]);
        assert!(!ret.is_prime[6]);
        assert!(!ret.is_prime[24]);
        assert_eq!(ret.is_prime.iter().filter(|&i| *i).count(), 25);
    }

    #[test]
    fn test_mod_power() {
        let modulo: u64 = 1000000007;
        assert_eq!(mod_power(2, 3, modulo), 8);
        assert_eq!(mod_power(5, 8, modulo), 390625);
        assert_eq!(mod_power(31, 8, modulo), 891031477);
        assert_eq!(mod_power(99, 999999991, modulo), 631782380);
        assert_eq!(mod_power(1, 999999570, modulo), 1);
        assert_eq!(mod_power(83, 999999570, modulo), 642657703);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2, 3), 8);
        assert_eq!(power(5, 8), 390625);
        assert_eq!(power(31, 7), 27512614111);
    }

    #[test]
    fn test_euler_phi() {
        assert_eq!(euler_phi(6), 2);
        assert_eq!(euler_phi(39), 24);
        assert_eq!(euler_phi(999999995), 789955584);
        assert_eq!(euler_phi(999999991), 985074552);
    }
}
