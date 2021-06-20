use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let c = u32::min(a, b);
    let d = u32::max(a, b);

    for _ in 0..d {
        print!("{}", c);
    }
}
