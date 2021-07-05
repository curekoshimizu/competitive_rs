// keywords :

use input_lib::*;
use prime::euler_phi;

fn main() {
    input! {
        n: u64,
    }

    println!("{}", euler_phi(n));
}
