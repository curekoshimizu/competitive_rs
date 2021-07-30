// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        max_w: u64,
        a: [(usize, u64); n], // (v, w)
    }

    let max_v = 10000usize;

    let mut dp = vec![u64::max_value(); max_v + 1];
    dp[0] = 0;

    for (v, w) in a {
        for i in (0..=max_v).rev() {
            if i >= v {
                let index = i - v;
                dp[i] = u64::min(dp[index].saturating_add(w), dp[i]);
            }
        }
    }

    for i in (0..=max_v).rev() {
        if dp[i] <= max_w {
            println!("{}", i);
            break;
        }
    }
}
