// keywords :

use collection_lib::union_find::UnionFind;
use io_lib::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n*n],
    }

    let mut g = vec![Vec::new(); n];

    for i in 0..n {
        for j in 0..n {
            let w = a[i * n + j];
            if w >= 0 {
                g[i].push((j, w));
            }
        }
    }
    let ret = kruskal(&g);
    println!("{}", ret);
}

fn kruskal(g: &Vec<Vec<(usize, i64)>>) -> i64 {
    let n = g.len();

    let mut cost_edges = Vec::new();
    for start in 0..n {
        let v = &g[start];
        for &(end, w) in v {
            if start < end {
                cost_edges.push((w, start, end));
            }
        }
    }
    cost_edges.sort();

    let mut total_cost = 0;

    let mut uf = UnionFind::new(n);

    for (w, start, end) in cost_edges {
        if !uf.is_same(start, end) {
            total_cost += w;
            uf.unite(start, end);
        }
    }

    total_cost
}
