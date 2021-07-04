// keywords :

use input_lib::Scanner;

use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());
    let n = sc.next::<u64>().unwrap();

    let mut list = VecDeque::new();
    for _ in 0..n {
        let command = sc.next::<String>().unwrap();
        match &command as &str {
            "insert" => {
                let key = sc.next::<u64>().unwrap();
                list.push_front(key);
            }
            "delete" => {
                let key = sc.next::<u64>().unwrap();
                match list.iter().position(|&x| x == key) {
                    Some(at) => {
                        if at == 0 {
                            list.pop_front();
                        } else if at == list.len() {
                            list.pop_back();
                        } else {
                            list.remove(at);
                        }
                    }
                    None => {}
                };
            }
            "deleteFirst" => {
                list.pop_front();
            }
            "deleteLast" => {
                list.pop_back();
            }
            _ => panic!("!!!!!"),
        };
    }

    println!(
        "{}",
        list.iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
