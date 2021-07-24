// keywords :

use io_lib::*;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());

    let n = sc.next::<usize>().unwrap();

    let mut x: u64 = 0;

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
                x |= 1 << i;
            }
            2 => {
                let i = sc.next::<usize>().unwrap();
                x &= !(1 << i);
            }
            3 => {
                let i = sc.next::<usize>().unwrap();

                let y = !x & (1 << i);

                x = (x & !(1 << i)) | y
            }
            4 => {
                let ret = if x == u64::max_value() { 1 } else { 0 };
                println!("{}", ret);
            }
            5 => {
                let ret = if x != 0 { 1 } else { 0 };
                println!("{}", ret);
            }
            6 => {
                let ret = if x == 0 { 1 } else { 0 };
                println!("{}", ret);
            }
            7 => {
                println!("{}", x.count_ones());
            }
            8 => {
                println!("{}", x);
            }
            _ => {
                panic!("!!!!!!!!!!");
            }
        }
    }
}
