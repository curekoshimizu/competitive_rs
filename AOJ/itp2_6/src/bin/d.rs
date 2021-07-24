// keywords :

use io_lib::*;
use search_lib::BinarySearch;

fn main() {
    input! {
        n: u64,
        a: [i64; n],
        m: usize,
        q: [i64; m],
    }

    for x in q {
        let ret1 = a.lower_bound(&x).unwrap_or(a.len());
        let ret2 = a.upper_bound(&x).unwrap_or(a.len());
        println!("{} {}", ret1, ret2);
    }
}
