// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u64,
        a: [i64; n],
    }

    let mut a = a;
    a.dedup();
    println!(
        "{}",
        a.iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
