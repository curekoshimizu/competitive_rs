// keywords :

use collection_lib::union_find::UnionFind;
use io_lib::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        seq: [(usize, usize, usize); q],
    }

    let mut uf = UnionFind::new(n);

    for v in seq {
        let (q, x, y) = v;
        match q {
            0 => uf.unite(x, y),
            1 => {
                let ret = if uf.is_same(x, y) { 1 } else { 0 };
                println!("{}", ret);
            }
            _ => panic!("!!!!!"),
        }
    }
}
