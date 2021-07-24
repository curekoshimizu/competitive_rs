// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u64,
        a: [i64; n],
        m: usize,
        q: [(usize, usize, usize); m],
    }
    let mut a = a;

    for (b, e, t) in q {
        let n = e - b;
        for k in 0..n {
            let x = b + k;
            let y = t + k;
            a.swap(x, y);
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
