// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u64,
        k: u64,
        w_seq: [u64; n],
    }

    println!("{}", n);
}

fn simulation(w_seq: &Vec<u64>, k: u64, p: u64) -> bool {
    let mut cur = 0;
    let mut count = 0;

    for &w in w_seq {
        if cur + (w as u64) <= p {
            cur += w;
        } else {
            count += 1;
        }
    }

    count < k
}
