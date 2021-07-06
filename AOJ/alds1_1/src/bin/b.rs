// keywords :

use gcd::gcd;
use io_lib::*;

fn main() {
    input! {
        x: u64,
        y:u64,
    };

    let vec = vec![x, y];
    println!("{}", gcd(&vec));
}
