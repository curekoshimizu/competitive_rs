use argio::argio;

#[argio]
fn main(n: u32, a: u32) -> &str {
    if n % 500 <= a {
        "Yes"
    } else {
        "No"
    }
}
