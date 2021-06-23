use argio::argio;

#[argio]
fn main(a: u32, b: u32) -> &str {
    if a % 2 == 0 {
        return "Even";
    }
    if b % 2 == 0 {
        return "Even";
    }

    return "Odd";
}
