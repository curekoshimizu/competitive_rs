// keywords :

// merge(A, left, mid, right)
//   n1 = mid - left;
//   n2 = right - mid;
//   L[0...n1], R[0...n2] を生成
//   for i = 0 to n1-1
//     L[i] = A[left + i]
//   for i = 0 to n2-1
//     R[i] = A[mid + i]
//   L[n1] = INFTY
//   R[n2] = INFTY
//   i = 0
//   j = 0
//   for k = left to right-1
//     if L[i] <= R[j]
//       A[k] = L[i]
//       i = i + 1
//     else
//       A[k] = R[j]
//       j = j + 1
//
// mergeSort(A, left, right)
//   if left+1 < right
//     mid = (left + right)/2;
//     mergeSort(A, left, mid)
//     mergeSort(A, mid, right)
//     merge(A, left, mid, right)

use io_lib::*;

fn main() {
    input! {
        n: u32,
        a: [u64; n],
    }
    let mut a = a;

    let right = a.len();
    let ret = merge_sort(&mut a, 0, right);

    println!(
        "{}",
        a.iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    println!("{}", ret);
}

fn merge_sort(a: &mut Vec<u64>, left: usize, right: usize) -> u64 {
    let mut count = 0;
    if left + 1 < right {
        let mid = (left + right) / 2;
        count += merge_sort(a, left, mid);
        count += merge_sort(a, mid, right);
        count += merge(a, left, mid, right);
    }

    return count;
}

fn merge(a: &mut Vec<u64>, left: usize, mid: usize, right: usize) -> u64 {
    let mut l = a[left..mid].to_vec();
    let mut r = a[mid..right].to_vec();

    l.push(u64::max_value());
    r.push(u64::max_value());

    let mut i = 0;
    let mut j = 0;

    let mut count = 0;

    for k in left..right {
        count += 1;
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
        }
    }

    count
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test() {
//     }
// }
