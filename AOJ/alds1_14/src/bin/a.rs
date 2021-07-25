// keywords :

use io_lib::*;

fn main() {
    input! {
        t: String,
        p: String,
    }

    let n = t.len();
    let m = p.len();
    if n >= m {
        for i in 0..=(n - m) {
            if t[i..(i + m)] == p {
                println!("{}", i);
            }
        }
    }
}
