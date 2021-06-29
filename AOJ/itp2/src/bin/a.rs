// keywords :

use input_lib::*;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    if a < b {
        println!("a < b");
    } else if a > b {
        println!("a > b");
    } else {
        println!("a == b");
    }
}
