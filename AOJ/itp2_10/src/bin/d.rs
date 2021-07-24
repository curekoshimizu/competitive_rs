// keywords :

use io_lib::*;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());

    let n = sc.next::<usize>().unwrap();
    let mut masks = vec![0u64; n];
    for index in 0..n {
        let m = sc.next::<usize>().unwrap();
        for _ in 0..m {
            let i = sc.next::<usize>().unwrap();
            masks[index] |= 1 << i;
        }
    }

    let mut x: u64 = 0;

    let n = sc.next::<usize>().unwrap();
    for _ in 0..n {
        let q = sc.next::<usize>().unwrap();
        match q {
            0 => {
                let i = sc.next::<usize>().unwrap();
                let ret = if (x & (1 << i)) > 0 { 1 } else { 0 };
                println!("{}", ret);
            }
            1 => {
                let i = sc.next::<usize>().unwrap();
                let m = masks[i];
                x |= m;
            }
            2 => {
                let i = sc.next::<usize>().unwrap();
                let m = masks[i];
                x &= !m;
            }
            3 => {
                let i = sc.next::<usize>().unwrap();
                let m = masks[i];
                let y = !x & m;
                x = (x & !m) | y;
            }
            4 => {
                let i = sc.next::<usize>().unwrap();
                let m = masks[i];
                let ret = if x & m == m { 1 } else { 0 };
                println!("{}", ret);
            }
            5 => {
                let i = sc.next::<usize>().unwrap();
                let m = masks[i];
                let ret = if x & m != 0 { 1 } else { 0 };
                println!("{}", ret);
            }
            6 => {
                let i = sc.next::<usize>().unwrap();
                let m = masks[i];
                let ret = if x & m == 0 { 1 } else { 0 };
                println!("{}", ret);
            }
            7 => {
                let i = sc.next::<usize>().unwrap();
                let m = masks[i];
                println!("{}", (x & m).count_ones());
            }
            8 => {
                let i = sc.next::<usize>().unwrap();
                let m = masks[i];
                println!("{}", x & m);
            }
            _ => {
                panic!("!!!!!!!!!!");
            }
        }
    }
}
