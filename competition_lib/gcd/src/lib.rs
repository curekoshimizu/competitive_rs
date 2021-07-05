use std::convert::TryInto;

pub fn gcd(vec: &Vec<u64>) -> u64 {
    assert!(vec.len() >= 2);

    vec.into_iter().fold(0, |a, &b| _gcd_core(a, b))
}

// recursive version
// fn _gcd_core(a: u64, b: u64) -> u64 {
//     if b == 0 {
//         return a;
//     }
//
//     return _gcd_core(b, a % b);
// }

// non-recursive version
// (a, b) -> (b, a % b) -> ..... -> (a, 0) => a
fn _gcd_core(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    return a;
}

/// let (g, x, y) = ext_gcd(a, b);
/// ax + by = y
/// if ab != 0, then |x| <= a, |y| <= b
pub fn ext_gcd(a: u64, b: u64) -> (u64, i64, i64) {
    let (d, x, y) = _ext_gcd_core(a.try_into().unwrap(), b.try_into().unwrap());
    return (d.try_into().unwrap(), x, y);
}

// [ (x, y) 更新式の導出 ]
//   a   = b   q_1 + r_1
//   b   = r_1 q_2 + r_2
//   r_1 = r_2 q_3 + r_3
//      .....
//   r_n = r_(n+1) q_(n+2) + r_(n+2)
//      .....
//   ここで
//   r_(-1) = a, r_0 = b とすると
//
//   r_-1= r_0 q_1 + r_1
//   r_0 = r_1 q_2 + r_2
//   r_1 = r_2 q_3 + r_3
//      .....
//   r_n = r_(n+1) q_(n+2) + r_(n+2)
//   と r_n と q_n の関係式ができる
//
//   r_n が a, b の一次結合 r_n = a x_n + b y_n でかけるとすると
//   r_(n+2) = r_n - r_(n+1) q_(n+2)
//           = (a x_n + b y_n) - (a x_(n+1) + b y_(n+1) ) q_(n+2)
//           = a ( x_n - x_(n+1) q_(n+2) ) + b ( y_n - y_(n+1) q_(n+2) )
//
//   よって
//     x_(n+2)  =  x_n - x_(n+1) q_(n+2)
//     y_(n+2)  =  y_n - y_(n+1) q_(n+2)
//   が得らる。 x_n 側に着目すると
//   r_(-1) = a, r_0 = b と定義したことから
//
//     x_(-1) = 1,   x_0 = 0  となる
//
//    つまり
//     1, 0, .... で x_(n+2) = x_n - x_(n+1) q_(n+2) という式を解けば x の更新式が得られる
//    y 側も同様で 0, 1, ... で 上の更新式
fn _ext_gcd_core(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut x0, mut x1) = (1, 0);
    let (mut y0, mut y1) = (0, 1);

    while b != 0 {
        let q = a / b;
        let r = a % b;
        let x2 = x0 - x1 * q;
        let y2 = y0 - y1 * q;

        x0 = x1;
        x1 = x2;
        y0 = y1;
        y1 = y2;
        a = b;
        b = r;
    }

    return (a, x0, y0);
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
