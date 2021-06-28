// keywords : chars

use argio::argio;
use proconio::marker::Chars;

#[argio]
fn main(s: [Chars; 3]) -> String {
    format!("{}{}{}", s[0][0], s[1][1], s[2][2])
}
