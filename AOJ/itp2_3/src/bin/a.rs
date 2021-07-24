// keywords :

use io_lib::*;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let min = i64::min(i64::min(a, b), c);
    let max = i64::max(i64::max(a, b), c);
    println!("{} {}", min, max);
}
