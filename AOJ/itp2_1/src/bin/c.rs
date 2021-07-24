// keywords :

use io_lib::*;
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());

    let n = sc.next::<usize>().unwrap();
    let mut vec = VecDeque::new();

    let mut pos = vec.len() as isize;

    for _ in 0..n {
        let q = sc.next::<u32>().unwrap();
        match q {
            0 => {
                let x = sc.next::<i64>().unwrap();
                if pos < vec.len() as isize {
                    let pos = pos as usize;
                    let mut v1 = vec.drain(pos..).collect::<VecDeque<_>>();
                    vec.push_back(x);
                    vec.append(&mut v1);
                } else {
                    vec.push_back(x);
                }
            }
            1 => {
                let d = sc.next::<isize>().unwrap();
                pos += d;
            }
            2 => {
                if pos < vec.len() as isize {
                    vec.remove(pos as usize);
                } else {
                    vec.pop_back();
                    pos = vec.len() as isize;
                }
            }
            _ => panic!("!"),
        }
    }

    for x in vec {
        println!("{}", x);
    }
}
