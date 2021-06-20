use argio::argio;

#[argio]
fn main(n: u32, m: u32) -> &str {
    if n == m { "Yes" } else { "No" }
}
