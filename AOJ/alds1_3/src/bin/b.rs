// keywords :

use io_lib::*;
use std::collections::VecDeque;

fn main() {
    input! {
        n: u32,
        limit: u32,
        v: [(String, u32); n],
    }

    let mut current_time = 0;
    let mut queue = VecDeque::new();
    for x in v {
        queue.push_back(x);
    }

    // solve

    while let Some((name, t)) = queue.pop_front() {
        if t <= limit {
            current_time += t;
            println!("{} {}", name, current_time);
        } else {
            current_time += limit;
            queue.push_back((name, t - limit));
        }
    }
}
