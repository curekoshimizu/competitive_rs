// keywords :

use io_lib::*;

// 1 bubbleSort(A, N) // N 個の要素を含む 0-オリジンの配列 A
// 2   flag = 1 // 逆の隣接要素が存在する
// 3   while flag
// 4     flag = 0
// 5     for j が N-1 から 1 まで
// 6       if A[j] < A[j-1]
// 7         A[j] と A[j-1] を交換
// 8         flag = 1

fn main() {
    input! {
        n: u32,
        a: [u32; n],
    }
    let mut a = a;

    let mut count = 0;
    let mut flag = true;
    while flag {
        flag = false;
        for j in (1..n).rev() {
            let j = j as usize;
            if a[j] < a[j - 1] {
                let tmp = a[j];
                a[j] = a[j - 1];
                a[j - 1] = tmp;
                flag = true;
                count += 1;
            }
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|&i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("{}", count);
}
