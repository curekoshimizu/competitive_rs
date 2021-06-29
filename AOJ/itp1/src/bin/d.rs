// keywords :

use input_lib::*;

fn main() {
    input! {
        s: u64,
    }

    let h = s / 3600;
    let s = s % 3600;
    let m = s / 60;
    let s = s % 60;

    println!("{}:{}:{}", h, m, s);
}
