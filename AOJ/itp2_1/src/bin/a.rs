// keywords :

use io_lib::*;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());

    let n = sc.next::<usize>().unwrap();
    let mut vec = Vec::new();

    for _ in 0..n {
        let q = sc.next::<u32>().unwrap();
        match q {
            0 => {
                let x = sc.next::<i64>().unwrap();
                vec.push(x);
            }
            1 => {
                let pos = sc.next::<usize>().unwrap();
                println!("{}", vec[pos]);
            }
            2 => {
                vec.pop();
            }
            _ => panic!("!"),
        }
    }
}
