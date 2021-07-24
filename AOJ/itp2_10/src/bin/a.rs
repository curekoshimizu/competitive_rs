// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u32,
    }
    println!("{:032b}", n);
    println!("{:032b}", !n);
    println!("{:032b}", n << 1);
    println!("{:032b}", n >> 1);
}
