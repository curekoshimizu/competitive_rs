// keywords :

use io_lib::*;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    println!("{} {}", a * b, 2 * a + 2 * b);
}
