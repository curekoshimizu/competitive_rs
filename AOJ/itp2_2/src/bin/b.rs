// keywords :

use io_lib::*;
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());

    let n = sc.next::<usize>().unwrap();
    let mut vec = vec![VecDeque::new(); n];
    let q = sc.next::<usize>().unwrap();

    for _ in 0..q {
        let q = sc.next::<u32>().unwrap();
        match q {
            0 => {
                let t = sc.next::<usize>().unwrap();
                let x = sc.next::<i64>().unwrap();
                vec[t].push_back(x);
            }
            1 => {
                let t = sc.next::<usize>().unwrap();
                if !vec[t].is_empty() {
                    println!("{}", vec[t].front().unwrap());
                }
            }
            2 => {
                let t = sc.next::<usize>().unwrap();
                vec[t].pop_front();
            }
            _ => panic!("!"),
        }
    }
}
