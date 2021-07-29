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

    let min = Monoid::new(0, |&x, &y| x + y);
    let mut tree = SegmentTree::new(n, min);

    for (q, s, t) in q {
        match q {
            0 => {
                let s = s as usize - 1;
                let x = tree.get(s);
                tree.update(s, x + t);
            }
            1 => {
                let ret = tree.query(s as usize - 1, t as usize);
                println!("{}", ret);
            }
            _ => panic!("!!!!"),
        }
    }
}
