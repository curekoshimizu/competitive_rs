// keywords :

use input_lib::*;

fn solve() -> bool {
    input! {
        w: i32,
        h: i32,
        x: i32,
        y: i32,
        r: i32,
    }

    if x - r < 0 {
        return false;
    }
    if x + r > w {
        return false;
    }
    if y - r < 0 {
        return false;
    }
    if y + r > h {
        return false;
    }

    return true;
}

fn main() {
    if solve() {
        println!("Yes");
    } else {
        println!("No");
    }
}
