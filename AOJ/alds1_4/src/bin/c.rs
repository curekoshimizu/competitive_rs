// keywords :

use io_lib::*;
use std::collections::HashSet;

fn main() {
    input! {
        n: u32,
        operations: [ (String, String); n],
    }

    let mut set: HashSet<(u64, usize)> = HashSet::new();

    for op in operations {
        match &op.0[..] {
            "insert" => {
                set.insert(hash(&op.1));
            }
            "find" => {
                if set.contains(&hash(&op.1)) {
                    println!("yes")
                } else {
                    println!("no")
                }
            }
            _ => panic!("!!!!"),
        }
    }
}

fn hash(x: &String) -> (u64, usize) {
    let mut result = 0u64;
    for (i, c) in x.chars().enumerate() {
        let y: u64 = match c {
            'A' => 0x0,
            'C' => 0x1,
            'G' => 0x2,
            'T' => 0x3,
            _ => panic!("!!!"),
        } << (i * 2);
        result |= y;
    }

    (result, x.len())
}
