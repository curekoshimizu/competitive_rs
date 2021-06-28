// keywords :

use argio::argio;

#[argio]
fn main(a: u32, b: u32, c: u32) -> &str {
    if a + b >= c {
        "Yes"
    } else {
        "No"
    }
}
