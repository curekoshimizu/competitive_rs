// keywords : gcd
use argio::argio;
use gcd::lcm;

#[argio]
fn main(n: u64, t: [u64; n]) -> u64 {
    lcm(&t)
}
