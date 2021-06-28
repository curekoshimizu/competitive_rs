// keywords :

use argio::argio;

#[argio]
fn main(n: u64, m: u64) -> u64 {
    let a = u64::min(n, m);
    let b = u64::max(n, m);

    if a == 1 && b == 1 {
        return 1;
    }

    if a == 1 {
        return b - 2;
    }

    (a - 2) * (b - 2)
}
