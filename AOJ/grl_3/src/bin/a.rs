// keywords :

use collection_lib::union_find::UnionFind;
use io_lib::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    for i in 0..n {
        let mut uf = UnionFind::new(n);

        for &(s, t) in edges.iter() {
            if s != i && t != i {
                uf.unite(s, t);
            }
        }

        let mut root = None;
        for j in 0..n {
            if i != j {
                match root {
                    Some(k) => {
                        if uf.root(j) != k {
                            println!("{}", i);
                            break;
                        }
                    }
                    None => {
                        root = Some(uf.root(j));
                    }
                };
            }
        }
    }
}
