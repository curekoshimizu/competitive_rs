// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        ww: usize,
        q: [(u64, usize); n],
    }

    let mut dp = vec![0; ww + 1];

    for (v, w) in q {
        for j in (0..=ww).rev() {
            if j >= w {
                let index = j - w;
                let x = dp[index] + v;
                let y = dp[j];
                dp[j] = u64::max(x, y);
            }
        }
    }
    println!("{}", dp[ww]);
}
