// keywords :

use io_lib::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        m_seq: [i64; q],
    }

    let mut a = a;
    a.sort_by_key(|&x| std::cmp::Reverse(x));

    for m in m_seq {
        if solve(&a, m, 0) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn solve(a: &Vec<i64>, m: i64, index: usize) -> bool {
    if m == 0 {
        return true;
    }
    if m < 0 {
        return false;
    }

    if !(index < a.len()) {
        return false;
    }

    let ret = solve(a, m, index + 1);
    if ret {
        return true;
    }

    return solve(a, m - a[index], index + 1);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test() {
//     }
// }
