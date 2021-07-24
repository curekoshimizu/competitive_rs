// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        vec: [(usize, usize, usize); q],
    }

    for (q, x, y) in vec {
        if q == 0 {
            let min = a[x..y].iter().min().unwrap();
            println!("{}", min);
        } else {
            let max = a[x..y].iter().max().unwrap();
            println!("{}", max);
        }
    }
}
