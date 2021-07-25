// keywords :

use io_lib::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());
    let n = sc.next::<usize>().unwrap();

    let mut g: Vec<Vec<(usize, u64)>> = vec![Vec::new(); n];

    for _ in 0..n {
        let i = sc.next::<usize>().unwrap();
        let k = sc.next::<usize>().unwrap();
        for _ in 0..k {
            let j = sc.next::<usize>().unwrap();
            let w = sc.next::<u64>().unwrap();
            g[i].push((j, w));
        }
    }

    // dijkstra
    let mut dist = vec![u64::max_value(); n];
    let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, 0)));

    while let Some(Reverse((w, e))) = heap.pop() {
        if dist[e] != u64::max_value() {
            continue;
        }
        dist[e] = w;

        for &(vv, ww) in g[e].iter() {
            let d = dist[e].saturating_add(ww);
            heap.push(Reverse((d, vv)));
        }
    }

    for i in 0..n {
        println!("{} {}", i, dist[i]);
    }
}
