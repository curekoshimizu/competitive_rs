// keywords :

use io_lib::*;
use std::collections::HashSet;

fn main() {
    input! {
        n: u64,
        q: [(u64, u64); n],
    }

    let mut set = HashSet::new();

    for (k, x) in q {
        match k {
            0 => {
                set.insert(x);
                println!("{}", set.len());
            }
            1 => {
                let ret = if set.contains(&x) { 1 } else { 0 };
                println!("{}", ret);
            }
            2 => {
                set.remove(&x);
            }
            _ => panic!("!!!!"),
        }
    }
}
