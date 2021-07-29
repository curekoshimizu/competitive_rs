// keywords :

use collection_lib::segment_tree::Monoid;
use collection_lib::segment_tree::SegmentTree;
use io_lib::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: [(u64, u64, u64); m],
    }

    let min = Monoid::new(2147483647, |&x, &y| u64::min(x, y));
    let mut tree = SegmentTree::new(n, min);

    for (q, s, t) in q {
        match q {
            0 => {
                tree.update(s as usize, t);
            }
            1 => {
                let ret = tree.query(s as usize, t as usize + 1);
                println!("{}", ret);
            }
            _ => panic!("!!!!"),
        }
    }
}
