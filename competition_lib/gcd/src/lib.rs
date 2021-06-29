use std::convert::TryInto;

pub fn gcd(vec: &Vec<u64>) -> u64 {
    assert!(vec.len() >= 2);

    vec.into_iter().fold(0, |a, &b| _gcd_core(a, b))
}

fn _gcd_core(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    return _gcd_core(b, a % b);
}

pub fn ext_gcd(a: u64, b: u64) -> (u64, i64, i64) {
    let (d, x, y) = _ext_gcd_core(a.try_into().unwrap(), b.try_into().unwrap());
    return (d.try_into().unwrap(), x, y);
}

fn _ext_gcd_core(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (d, mut y, x) = _ext_gcd_core(b, a % b);

    y -= (a / b) * x;

    return (d, x, y);
}

pub fn lcm(vec: &Vec<u64>) -> u64 {
    vec.into_iter().fold(1, |a, &b| _lcm_core(a, b))
}

fn _lcm_core(a: u64, b: u64) -> u64 {
    let gcd = _gcd_core(a, b);
    return (a / gcd) * b;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(_gcd_core(0, 5), 5);
        assert_eq!(_gcd_core(5, 0), 5);
        assert_eq!(_gcd_core(7, 5), 1);
        assert_eq!(_gcd_core(2, 4), 2);
        assert_eq!(_gcd_core(4, 2), 2);
        assert_eq!(gcd(&vec!(2, 2, 2)), 2);
        assert_eq!(gcd(&vec!(2, 8, 2)), 2);
    }
    #[test]
    fn test_extgcd() {
        fn check(a: i64, b: i64) -> bool {
            let (d, x, y) = ext_gcd(a as u64, b as u64);

            a * x + b * y == d.try_into().unwrap()
        }
        assert!(check(0, 5));
        assert!(check(5, 0));
        assert!(check(7, 5));
        assert!(check(2, 4));
        assert!(check(4, 2));
        assert!(check(2, 2));
    }
    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&vec!(2, 2)), 2);
        assert_eq!(lcm(&vec!(2, 2, 2)), 2);
        assert_eq!(lcm(&vec!(2, 4, 2)), 4);
        assert_eq!(lcm(&vec!(4, 8, 4)), 8);
        assert_eq!(lcm(&vec!(4, 8, 5)), 40);
    }
}
