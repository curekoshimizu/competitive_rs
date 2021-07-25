// keywords :

use io_lib::*;
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());
    let n = sc.next::<usize>().unwrap();

    let mut g = vec![Vec::new(); n];

    for _ in 0..n {
        let i = sc.next::<usize>().unwrap();
        let k = sc.next::<usize>().unwrap();
        for _ in 0..k {
            let j = sc.next::<usize>().unwrap();
            g[i - 1].push(j - 1);
        }
    }

    let mut dist = vec![-1; g.len()];

    bfs(0, &g, &mut dist);

    for i in 0..n {
        println!("{} {}", i + 1, dist[i]);
    }
}

fn bfs(v: usize, g: &Vec<Vec<usize>>, dist: &mut Vec<i32>) {
    let mut queue = VecDeque::new();
    queue.push_back((v, 0));
    dist[0] = 0;

    while let Some((v, d)) = queue.pop_front() {
        for &i in g[v].iter() {
            if dist[i] == -1 {
                dist[i] = d + 1;
                queue.push_back((i, d + 1));
            }
        }
    }
}
