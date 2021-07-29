// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        ww: usize,
        q: [(u64, usize); n],
    }

    let mut dp_prev = vec![0; ww + 1];

    for (v, w) in q {
        let mut dp_new = vec![0; ww + 1];
        for j in 0..=ww {
            if j >= w {
                let index = j - w;
                let x = dp_prev[index] + v;
                let y = dp_prev[j];
                dp_new[j] = u64::max(x, y);
            } else {
                dp_new[j] = dp_prev[j];
            }
        }
        dp_prev = dp_new;
    }
    println!("{}", dp_prev[ww]);
}
