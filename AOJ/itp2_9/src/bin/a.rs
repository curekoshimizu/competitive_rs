// keywords :

use io_lib::*;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: u64,
        a: [u64; n],
        m: u64,
        b: [u64; m],
    }
    let mut a: BTreeSet<u64> = a.into_iter().collect();

    for x in b {
        a.insert(x);
    }

    for x in a {
        println!("{}", x);
    }
}
