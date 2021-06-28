// keywords : hash, entry, unwrap_or

use argio::argio;
use std::collections::HashMap;

#[argio]
fn main(n: u32, a: [String; n], m: u32, b: [String; m]) -> u32 {
    let mut red_map: HashMap<&String, u32> = HashMap::new();

    for x in a.iter() {
        let entry = red_map.entry(x).or_default(); // you can use .or_insert(0) as well
        *entry += 1;
    }
    for x in b.iter() {
        let entry = red_map.entry(x).or_default();
        if *entry > 0 {
            *entry -= 1;
        }
    }

    *red_map.values().max().unwrap_or(&0)
}
