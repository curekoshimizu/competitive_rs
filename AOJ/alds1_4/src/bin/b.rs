// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u32,
        s: [u64; n],
        q: u32,
        t: [u64; q],
    }

    let result = t.iter().filter_map(|&x| s.binary_search(&x).ok()).count();

    println!("{}", result);
}
