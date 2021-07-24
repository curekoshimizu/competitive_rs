// keywords :

use io_lib::*;
use std::collections::HashSet;

fn main() {
    input! {
        n: u64,
        a: [i64; n],
        m: u64,
        b: [i64; m],
    }
    let a = a.iter().collect::<HashSet<_>>();

    let ret = b.iter().all(|x| a.contains(&x));

    if ret {
        println!("1")
    } else {
        println!("0")
    }
}
