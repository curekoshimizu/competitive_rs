// keywords : iproduct

use argio::argio;
use itertools::iproduct;

#[argio]
fn main(a: u32, b: u32) -> usize {
    iproduct!(0..10, 0..10, 0..10)
        .filter(|(x, y, z)| {
            let target = x * 10000 + y * 1000 + z * 100 + y * 10 + x;
            a <= target && target <= b
        })
        .count()
}
