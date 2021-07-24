// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u64,
        a: [(i64, i64); n],
    }
    let mut a = a;
    a.sort();
    a.iter().for_each(|&i| println!("{} {}", i.0, i.1));
}
