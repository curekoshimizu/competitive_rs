// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u64,
        m: usize,
        c: [u64; m],
    }

    let mut c = c;
    c.sort();

    assert_eq!(c[0], 1);

    let mut dp: Vec<u64> = (0..=n).collect();

    for &c in c[1..].iter() {
        for i in 0..=n {
            if i >= c {
                let index = (i - c) as usize;
                let i = i as usize;
                dp[i] = u64::min(dp[index].saturating_add(1), dp[i]);
            }
        }
    }
    println!("{}", dp[n as usize]);
}
