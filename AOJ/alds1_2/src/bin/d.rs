// keywords :

// 1  insertionSort(A, n, g)
// 2      for i = g to n-1
// 3          v = A[i]
// 4          j = i - g
// 5          while j >= 0 && A[j] > v
// 6              A[j+g] = A[j]
// 7              j = j - g
// 8              cnt++
// 9          A[j+g] = v
// 10
// 11 shellSort(A, n)
// 12     cnt = 0
// 13     m = ?
// 14     G[] = {?, ?,..., ?}
// 15     for i = 0 to m-1
// 16         insertionSort(A, n, G[i])

use io_lib::*;

fn main() {
    input! {
        n: u64,
        a: [u64; n],
    }

    let mut a = a;

    let gap = make_gap(n);

    println!("{}", gap.len());
    println!(
        "{}",
        gap.iter()
            .rev()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    // shell sort
    let total_cnt: i32 = gap.iter().rev().map(|&g| insertion_sort(&mut a, g)).sum();
    println!("{}", total_cnt);

    a.iter().for_each(|&i| println!("{}", i));
}

fn make_gap(n: u64) -> Vec<u64> {
    // 1, 4, 13, ...
    let mut gap: Vec<u64> = vec![];

    let mut prev = 1u64;
    let mut cur = 1u64;

    while cur <= n {
        gap.push(cur);

        prev = 3 * cur + prev;
        std::mem::swap(&mut prev, &mut cur);
    }

    gap
}

fn insertion_sort(a: &mut Vec<u64>, g: u64) -> i32 {
    let mut cnt = 0;
    let n = a.len() as isize;
    let g = g as isize;
    for i in g..n {
        let v = a[i as usize];
        let mut j = i - g;
        while j >= 0 && a[j as usize] > v {
            a[(j + g) as usize] = a[j as usize];
            j = j - g;
            cnt += 1;
        }
        a[(j + g) as usize] = v;
    }

    cnt
}
