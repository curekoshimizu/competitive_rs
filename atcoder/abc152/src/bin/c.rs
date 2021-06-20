use argio::argio;

#[argio]
fn main(n: u32, a: [u32; n]) -> u32 {
    let mut min = u32::max_value();

    a.into_iter()
        .map(|x| {
            min = u32::min(min, x);
            (x <= min) as u32
        })
        .sum()
}
