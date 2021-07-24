// keywords :

use io_lib::*;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());

    let n = sc.next::<usize>().unwrap();
    let mut vec = vec![Vec::new(); n];

    let q = sc.next::<usize>().unwrap();
    for _ in 0..q {
        let q = sc.next::<u32>().unwrap();
        match q {
            0 => {
                let t = sc.next::<usize>().unwrap();
                let x = sc.next::<i64>().unwrap();
                vec[t].push(x);
            }
            1 => {
                let t = sc.next::<usize>().unwrap();
                println!(
                    "{}",
                    vec[t]
                        .iter()
                        .map(|&i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
            2 => {
                let s = sc.next::<usize>().unwrap();
                let t = sc.next::<usize>().unwrap();
                let mut u = vec[s].drain(..).collect();
                vec[t].append(&mut u);
            }
            _ => panic!("!"),
        }
    }
}
