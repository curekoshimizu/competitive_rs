// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        a: [(usize, usize, String, usize, String); n],
    }

    let mut a = a;
    a.sort();
    for x in a {
        println!("{} {} {} {} {}", x.0, x.1, x.2, x.3, x.4);
    }
}
