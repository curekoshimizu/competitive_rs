// keywords :

use io_lib::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        e: usize,
        r: usize,
        edges: [(usize, usize, i32); e],
    }

    let mut g = vec![Vec::new(); n];
    for (x, y, w) in edges {
        g[x].push((y, w));
    }

    let mut dist = vec![-1; n];

    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, r)));

    while let Some(Reverse((w, v))) = heap.pop() {
        if dist[v] != -1 {
            continue;
        }
        dist[v] = w;

        let d = dist[v];

        for &(vv, ww) in g[v].iter() {
            heap.push(Reverse((d + ww, vv)));
        }
    }

    for d in dist {
        if d == -1 {
            println!("INF");
        } else {
            println!("{}", d);
        }
    }
}
