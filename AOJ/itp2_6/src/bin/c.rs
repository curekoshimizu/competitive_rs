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
        if let Some(p) = a.lower_bound(&x) {
            println!("{}", p);
        } else {
            println!("{}", a.len());
        }
    }
}
