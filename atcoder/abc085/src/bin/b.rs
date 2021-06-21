use argio::argio;
use std::collections::HashSet;

#[argio]
fn main(n: u32, a: [u32; n]) -> u32 {
    a.iter().collect::<HashSet<_>>().len() as u32
}
