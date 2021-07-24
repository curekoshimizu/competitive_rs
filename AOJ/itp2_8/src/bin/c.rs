// keywords :

use io_lib::*;
use std::collections::BTreeMap;
// use std::ops::Bound::Included;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());

    let n = sc.next::<usize>().unwrap();

    let mut map = BTreeMap::new();

    for _ in 0..n {
        let k = sc.next::<u64>().unwrap();
        let x = sc.next::<String>().unwrap();

        match k {
            0 => {
                let v = sc.next::<u64>().unwrap();
                map.insert(x, v);
            }
            1 => {
                let ret = map.get(&x).unwrap();
                println!("{}", ret);
            }
            2 => {
                todo!();
                // let l = x;
                // let r = sc.next::<String>().unwrap();
                //
                // for (&e1, &e2) in map.range((Included(&l), Included(&r))) {
                //     println!("{} {}", e1, e2);
                // }
            }
            _ => panic!("!!!!"),
        }
    }
}
