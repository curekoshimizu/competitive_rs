// keywords :

use io_lib::*;

fn main() {
    input! {
        n: u32,
        a: [Chars ; n],
    }

    let result_a = make_graph(&a);

    let mut b = a.clone();
    let mut c = a.clone();

    bubble_sort(&mut b);
    let result_b = make_graph(&b);

    selection_sort(&mut c);
    let result_c = make_graph(&c);

    println!(
        "{}",
        b.iter()
            .map(|s| format!("{}{}", s[0], s[1]))
            .collect::<Vec<String>>()
            .join(" ")
    );

    assert!(is_same_graph(&result_a, &result_b));
    println!("Stable");

    println!(
        "{}",
        c.iter()
            .map(|s| format!("{}{}", s[0], s[1]))
            .collect::<Vec<String>>()
            .join(" ")
    );

    if is_same_graph(&result_a, &result_c) {
        println!("Stable");
    } else {
        println!("Not stable");
    }
}

fn make_graph(a: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![vec![]; 10];

    for s in a {
        let c = s[0];
        let index = s[1].to_digit(10).unwrap() as usize;

        result[index].push(c);
    }

    result
}

fn is_same_graph(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    for i in 0..10 {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

fn cmp(lhs: &Vec<char>, rhs: &Vec<char>) -> bool {
    lhs[1].to_digit(10).unwrap() < rhs[1].to_digit(10).unwrap()
}

fn bubble_sort(a: &mut Vec<Vec<char>>) {
    let n = a.len();

    let mut flag = true;
    while flag {
        flag = false;
        for j in (1..n).rev() {
            let j = j as usize;
            if cmp(&a[j], &a[j - 1]) {
                a.swap(j, j - 1);
                flag = true;
            }
        }
    }
}

fn selection_sort(a: &mut Vec<Vec<char>>) {
    let n = a.len();

    for i in 0usize..n {
        let mut min_j = i;
        for j in i..n {
            if cmp(&a[j], &a[min_j]) {
                min_j = j;
            }
        }
        if i != min_j {
            a.swap(i, min_j);
        }
    }
}
