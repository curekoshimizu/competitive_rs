// keywords :

use io_lib::*;
use std::collections::BTreeSet;
use std::ops::Bound::Included;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());

    let n = sc.next::<usize>().unwrap();

    let mut set = BTreeSet::new();

    for _ in 0..n {
        let k = sc.next::<u64>().unwrap();
        let x = sc.next::<u64>().unwrap();

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
            3 => {
                let l = x;
                let r = sc.next::<u64>().unwrap();

                for &e in set.range((Included(&l), Included(&r))) {
                    println!("{}", e);
                }
            }
            _ => panic!("!!!!"),
        }
    }
}
