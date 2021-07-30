// keywords :

use search_lib::BinarySearch;

use io_lib::*;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut dp = vec![u64::max_value(); n];

    for x in a {
        let index = dp.lower_bound(&x);
        match index {
            Some(i) => {
                dp[i] = x;
            }
            None => {
                panic!("!!!!");
            }
        }
    }
    let index = dp.lower_bound(&u64::max_value());
    let ret = match index {
        Some(i) => i,
        None => dp.len(),
    };
    println!("{}", ret);
}
