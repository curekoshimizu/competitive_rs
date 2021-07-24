// keywords :

use io_lib::*;
use search_lib::BinarySearch;

fn main() {
    input! {
        n: u64,
        a: [u64; n],
        m: usize,
        q: [u64; m],
    }

    for k in q {
        if let Some(i) = a.lower_bound(&k) {
            if a[i] == k {
                println!("1");
            } else {
                println!("0");
            }
        } else {
            println!("0");
        }
    }
}
