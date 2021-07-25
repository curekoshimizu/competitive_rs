// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u64,
    }

    println!("{}", fibo(n));
}

fn fibo(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        n => {
            let (mut m, mut m_1) = (1u64, 1u64);

            for _ in 0..(n - 1) {
                let tmp = m;
                m += m_1;
                m_1 = tmp;
            }

            m
        }
    }
}
