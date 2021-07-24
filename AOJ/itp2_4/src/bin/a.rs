// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u64,
        a: [i64; n],
        m: u64,
        q: [(usize, usize); m],
    }

    let mut a = a;
    for (x, y) in q {
        a[x..y].reverse();
    }

    println!(
        "{}",
        a.iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
