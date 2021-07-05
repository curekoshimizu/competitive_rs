// keywords :

use io_lib::*;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }

    let mut v = vec![a, b, c];
    v.sort();

    let ans = v
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}
