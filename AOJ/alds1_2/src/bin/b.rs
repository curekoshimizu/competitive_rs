// keywords :

// 1 selectionSort(A, N) // N個の要素を含む0-オリジンの配列A
// 2   for i が 0 から N-1 まで
// 3     minj = i
// 4     for j が i から N-1 まで
// 5       if A[j] < A[minj]
// 6         minj = j
// 7     A[i] と A[minj] を交換

use io_lib::*;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut count = 0;
    let mut a = a;

    for i in 0usize..n {
        let mut min_j = i;
        for j in i..n {
            if a[j] < a[min_j] {
                min_j = j;
            }
        }
        if i != min_j {
            count += 1;
            let tmp = a[i];
            a[i] = a[min_j];
            a[min_j] = tmp;
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
