// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        vec: [(usize, usize, i64); q],
    }

    for (x, y, k) in vec {
        let count = a[x..y].iter().filter(|&&i| i == k).count();
        println!("{}", count);
    }
}
