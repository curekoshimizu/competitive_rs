// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        e: usize,
        r: usize,
        edges: [(usize, usize, i32); e],
    }

    let mut g = vec![Vec::new(); n];
    for &(x, y, w) in edges.iter() {
        g[x].push((y, w));
    }

    let mut dist = vec![i32::max_value(); n];
    dist[r] = 0;

    let mut updated = true;
    let mut count = 0;

    while updated {
        updated = false;
        for &(s, t, w) in edges.iter() {
            if dist[s] != i32::max_value() {
                let d = dist[s] + w;
                if d < dist[t] {
                    dist[t] = d;
                    updated = true;
                }
            }
        }
        count += 1;

        if count > 2 * n {
            println!("NEGATIVE CYCLE");
            return;
        }
    }

    for d in dist {
        if d == i32::max_value() {
            println!("INF");
        } else {
            println!("{}", d);
        }
    }
}
