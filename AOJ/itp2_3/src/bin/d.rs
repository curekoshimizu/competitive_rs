// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        m: usize,
        b: [i64; m],
    }

    let mut i = 0;

    loop {
        if i >= a.len() && i >= b.len() {
            println!("0");
            break;
        }

        let x = if i < a.len() { a[i] } else { -1 };
        let y = if i < b.len() { b[i] } else { -1 };

        if x < y {
            println!("1");
            break;
        } else if x > y {
            println!("0");
            break;
        } else {
        }
        i += 1
    }
}
