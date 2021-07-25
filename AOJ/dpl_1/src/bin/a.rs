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

    for &x in c[1..].iter() {
        dp[x as usize] = 1;
        dp[x as usize + 1] = 1;
        for _i in 0..=n {}
    }
}
