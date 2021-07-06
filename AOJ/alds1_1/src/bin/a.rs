// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
    }

    let mut a = a;

    // 1 insertionSort(A, N) // N個の要素を含む0-オリジンの配列A
    // 2   for i が 1 から N-1 まで
    // 3     v = A[i]
    // 4     j = i - 1
    // 5     while j >= 0 かつ A[j] > v
    // 6       A[j+1] = A[j]
    // 7       j--
    // 8     A[j+1] = v

    show(&a);

    for i in 1..n {
        let i = i as usize;
        let v = a[i];
        let mut j = i - 1;

        while a[j] > v {
            a[j + 1] = a[j];
            a[j] = v;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        show(&a);
    }
}

fn show(a: &Vec<u32>) {
    println!(
        "{}",
        a.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
