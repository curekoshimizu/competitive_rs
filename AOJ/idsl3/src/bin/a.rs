// keywords :

use input_lib::*;

fn main() {
    input! {
        n: u32,
        s: [char; n],
    }

    println!("{}", s[0]);
}
