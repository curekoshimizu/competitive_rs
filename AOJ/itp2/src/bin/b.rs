// keywords :

use io_lib::*;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    if a < b && b < c {
        println!("Yes");
    } else {
        println!("No");
    }
}
