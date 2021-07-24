// keywords :

use io_lib::*;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    println!("{:032b}", a & b);
    println!("{:032b}", a | b);
    println!("{:032b}", a ^ b);
}
