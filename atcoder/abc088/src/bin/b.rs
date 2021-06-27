// keywords : reverse, step_by

use argio::argio;

#[argio]
fn main(n: u32, mut a: [u32; n]) -> u32 {
    a.sort_by_key(|&x| std::cmp::Reverse(x));

    let alice: u32 = a.iter().step_by(2).sum();

    let mut b = a.iter();

    b.next();

    let bob: u32 = b.step_by(2).sum();

    alice - bob
}
