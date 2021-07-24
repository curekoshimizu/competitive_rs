// keywords :

use io_lib::*;
use std::collections::BTreeMap;

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
                let ret = map.get(&x).unwrap_or(&0);
                println!("{}", ret);
            }
            2 => {
                map.remove(&x);
            }
            _ => panic!("!!!!"),
        }
    }
}
