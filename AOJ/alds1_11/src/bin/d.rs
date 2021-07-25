// keywords :

use collection_lib::union_find::UnionFind;
use io_lib::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        r: [(usize, usize); m],
        k: usize,
        q: [(usize, usize); k],
    }

    let mut uf = UnionFind::new(n);
    for (x, y) in r {
        uf.unite(x, y);
    }
    for (x, y) in q {
        let ret = if uf.is_same(x, y) { "yes" } else { "no" };
        println!("{}", ret);
    }
}
