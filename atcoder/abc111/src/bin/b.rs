// keywords : flat_map
use argio::argio;

#[argio]
fn main(n: u32) -> u32 {
    (0..10)
        .flat_map(|i| {
            let num = 100 * i + 10 * i + i;
            if n <= num {
                return Some(num);
            }

            return None;
        })
        .next()
        .unwrap()
}
