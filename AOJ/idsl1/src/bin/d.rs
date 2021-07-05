// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut min_table = vec![0; n];
    let mut max_table = vec![0; n];

    min_table[0] = a[0];
    for i in 1..n {
        min_table[i] = i64::min(min_table[i - 1], a[i]);
    }
    max_table[n - 1] = i64::min_value();
    for i in (0..(n - 1)).rev() {
        max_table[i] = i64::max(max_table[i + 1], a[i + 1]);
    }

    let mut r_table = vec![0; n];
    for i in 0..n {
        let r = max_table[i].saturating_sub(min_table[i]);
        r_table[i] = r;
    }

    let result = r_table.iter().max().unwrap();

    println!("{}", result);
}
