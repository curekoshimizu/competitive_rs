// keywords :

use gcd::lcm;
use input_lib::*;

fn main() {
    input! {
        n: u64,
        v: [u64; n],
    }

    let ret = lcm(&v);

    println!("{}", ret);
}
