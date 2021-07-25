// keywords :

use io_lib::*;

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

    let mut visited = vec![-1; g.len()];
    let mut finish = vec![-1; g.len()];

    let mut t = 0;
    for i in 0..n {
        if visited[i] == -1 {
            t = dfs(i, &g, &mut visited, &mut finish, t + 1);
        }
    }

    for i in 0..n {
        println!("{} {} {}", i + 1, visited[i], finish[i]);
    }
}

fn dfs(
    v: usize,
    g: &Vec<Vec<usize>>,
    visited: &mut Vec<i32>,
    finish: &mut Vec<i32>,
    mut t: i32,
) -> i32 {
    if visited[v] == -1 {
        visited[v] = t;
    }
    for &i in g[v].iter() {
        if visited[i] == -1 {
            t = dfs(i, g, visited, finish, t + 1);
        }
    }
    finish[v] = t + 1;

    t + 1
}
