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
    let a: BTreeSet<u64> = a.into_iter().collect();
    let b: BTreeSet<u64> = b.into_iter().collect();

    for x in a.intersection(&b) {
        println!("{}", x);
    }
}
