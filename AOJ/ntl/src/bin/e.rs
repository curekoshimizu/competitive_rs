// keywords :

use gcd::ext_gcd;
use input_lib::*;

fn norm(x: i64, y: i64) -> i64 {
    x.abs() + y.abs()
}

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let (_, mut x, mut y) = ext_gcd(a, b);

    let a = a as i64;
    let b = b as i64;

    loop {
        let c0 = norm(x, y);
        let c1 = norm(x + b * y, y - a * x);
        let c2 = norm(x - b * y, y + a * x);
        if c0 < c1 && c0 < c2 {
            break;
        }
        if c1 < c0 && c1 < c2 {
            let z = x + b * y;
            let w = y - a * x;
            x = z;
            y = w;
        } else {
            let z = x - b * y;
            let w = y + a * x;
            x = z;
            y = w;
        }
    }
    println!("{} {}", x, y);
}
