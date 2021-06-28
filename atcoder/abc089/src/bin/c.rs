// keywords : combinations

use argio::argio;
use itertools::Itertools;

#[argio]
fn main(n: u64, origin: [String; n]) -> u64 {
    let mut table = [0u64; 5];

    origin
        .iter()
        .for_each(|name| match name.chars().nth(0).unwrap() {
            'M' => table[0] += 1,
            'A' => table[1] += 1,
            'R' => table[2] += 1,
            'C' => table[3] += 1,
            'H' => table[4] += 1,
            _ => {}
        });

    table
        .iter()
        .combinations(3)
        .map(|vec| vec[0] * vec[1] * vec[2])
        .sum()
}
