// keywords :

use io_lib::*;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());
    let n = sc.next::<usize>().unwrap();

    let mut g = vec![vec![0; n]; n];

    for _ in 0..n {
        let i = sc.next::<usize>().unwrap();
        let k = sc.next::<usize>().unwrap();
        for _ in 0..k {
            let j = sc.next::<usize>().unwrap();
            g[i - 1][j - 1] = 1;
        }
    }

    for x in g {
        println!(
            "{}",
            x.iter()
                .map(|&i| i.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
