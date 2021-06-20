use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        m: u32,
    }

    let ans = if n == m { "Yes" } else { "No" };

    println!("{}", ans);
}
