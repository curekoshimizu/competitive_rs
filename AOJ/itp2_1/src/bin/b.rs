// keywords :

use io_lib::*;
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());

    let n = sc.next::<usize>().unwrap();
    let mut vec = VecDeque::new();

    for _ in 0..n {
        let q = sc.next::<u32>().unwrap();
        match q {
            0 => {
                let req = sc.next::<usize>().unwrap();
                let x = sc.next::<i64>().unwrap();
                if req == 0 {
                    vec.push_front(x);
                } else {
                    vec.push_back(x);
                }
            }
            1 => {
                let pos = sc.next::<usize>().unwrap();
                println!("{}", vec[pos]);
            }
            2 => {
                let req = sc.next::<usize>().unwrap();
                if req == 0 {
                    vec.pop_front();
                } else {
                    vec.pop_back();
                }
            }

            _ => panic!("!"),
        }
    }
}
