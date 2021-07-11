// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u32,
        s: [u64; n],
        q: u32,
        t: [u64; q],
    }

    let result = t.iter().filter(|&&x| s.iter().any(|&y| x == y)).count();

    println!("{}", result);
}
