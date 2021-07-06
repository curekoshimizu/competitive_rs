// keywords :

use io_lib::*;

fn main() {
    let mut stack: Vec<i32> = Vec::new();
    let mut ans: Vec<(i32, i32)> = Vec::new();

    input! {
        s: String,
    }

    let mut index = 0;

    for c in s.chars() {
        match c {
            '\\' => {
                stack.push(index);
            }
            '/' => {
                if let Some(start) = stack.pop() {
                    ans.push((start, index));
                }
            }
            '_' => {}
            _ => panic!("!!!!!"),
        }
        index += 1;
    }

    // make answer

    if let Some(mut current) = ans.pop() {
        let mut areas: Vec<_> = Vec::new();
        let mut area = current.1 - current.0;

        while let Some(next) = ans.pop() {
            if current.0 < next.0 && next.1 < current.1 {
                area += next.1 - next.0;
            } else {
                areas.push(area);
                current = next;
                area = current.1 - current.0;
            }
        }

        areas.push(area);

        // print
        println!("{}", areas.iter().sum::<i32>());

        println!(
            "{} {}",
            areas.len(),
            areas
                .iter()
                .rev()
                .map(|&i| i.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else {
        println!("{}", 0);
        println!("{}", 0);
    }
}
