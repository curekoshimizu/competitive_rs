// keywords : set

use argio::argio;

use std::collections::HashSet;

#[argio]
fn main(n: u32, a: [char; n]) -> &str {
    let set = a.iter().collect::<HashSet<_>>();

    match set.len() {
        3 => "Three",
        4 => "Four",
        _ => panic!("!?"),
    }
}
