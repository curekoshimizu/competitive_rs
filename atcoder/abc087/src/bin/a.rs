use argio::argio;

#[argio]
fn main(x: u32, a: u32, b: u32) -> u32 {
    let y = x - a;

    y - (y / b) * b
}
