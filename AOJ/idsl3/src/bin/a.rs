// keywords :

use input_lib::Scanner;

fn main() {
    let mut input = Scanner::new(std::io::stdin().lock());

    let mut stack: Vec<i64> = Vec::new();

    while let Some(v) = input.next::<String>() {
        match v.parse::<i64>() {
            Ok(digit) => stack.push(digit),
            _ => {
                assert_eq!(v.len(), 1);
                match v.chars().nth(0).unwrap() {
                    '+' => {
                        let y = stack.pop().unwrap();
                        let x = stack.pop().unwrap();

                        stack.push(x + y);
                    }
                    '-' => {
                        let y = stack.pop().unwrap();
                        let x = stack.pop().unwrap();

                        stack.push(x - y);
                    }
                    '*' => {
                        let y = stack.pop().unwrap();
                        let x = stack.pop().unwrap();

                        stack.push(x * y);
                    }
                    '/' => {
                        let y = stack.pop().unwrap();
                        let x = stack.pop().unwrap();

                        stack.push(x / y);
                    }
                    _ => {
                        panic!("!!!")
                    }
                }
            }
        }
    }

    println!("{}", stack.pop().unwrap());
}
