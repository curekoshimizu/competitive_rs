// keywords :

use io_lib::*;
use search_lib::binary_search_position;

fn main() {
    input! {
        n: u64,
        k: u64,
        w_seq: [u64; n],
    }

    let pred = |p| simulation(&w_seq, k, p);
    let result = binary_search_position(1, u64::max_value(), pred).unwrap();

    println!("{}", result);
}

fn simulation(w_seq: &Vec<u64>, k: u64, p: u64) -> bool {
    let mut cur: u64 = 0;
    let mut count = 0;

    for &w in w_seq {
        if w > p {
            return false;
        }

        if cur.saturating_add(w) <= p {
            cur += w;
        } else {
            count += 1;
            cur = w;
        }
    }
    if cur > 0 {
        count += 1;
    }

    count <= k
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simulation() {
        let w = vec![8, 1, 7, 3, 9];
        assert!(simulation(&w, 3, 10));
        assert!(!simulation(&w, 3, 9));
    }
}
