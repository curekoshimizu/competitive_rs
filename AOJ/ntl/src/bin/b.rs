// keywords :

use io_lib::*;
use prime::mod_power;

const MOD: u64 = 1000000007;

fn main() {
    input! {
        m: u64,
        n: u64,
    }

    println!("{}", mod_power(m, n, MOD));
}
