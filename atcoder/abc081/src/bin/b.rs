// keywords : gcd
use argio::argio;
use gcd::gcd;

fn main() {
    solver_1();
}

#[argio]
fn solver_1(n: u64, a: [u64; n]) -> u64 {
    let mut x = gcd(&a);

    let mut count = 0;
    while x % 2 == 0 {
        count += 1;
        x /= 2;
    }

    count
}

#[argio]
fn solver_2(n: u64, a: [u64; n]) -> u64 {
    fn num_div2(mut x: u64) -> u64 {
        let mut count = 0;
        while x % 2 == 0 {
            count += 1;
            x /= 2;
        }

        count
    }

    a.iter().map(|&x| num_div2(x)).min().unwrap()
}
