use argio::argio;

#[argio]
fn main(a: u32, b: u32, c: u32, d: u32) -> u32 {
    let x = u32::max(a, c);
    let y = u32::min(b, d);

    if x < y {
        y - x
    } else {
        0
    }
}
