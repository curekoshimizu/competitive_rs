// keywords :

use input_lib::*;

/// whether n is prime number or not
/// O( sqrt(n) )
fn is_prime(n: u32) -> bool {
    if n == 1 {
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

fn main() {
    input! {
        num: u32,
        a: [u32; num],
    }

    let result = a.iter().filter(|&i| is_prime(*i)).count();

    println!("{}", result);
}
