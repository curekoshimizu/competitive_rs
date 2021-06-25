use argio::argio;

#[argio]
fn main(
    na: u32, // 500
    nb: u32, // 100
    nc: u32, // 50
    x: u32,
) -> u32 {
    let mut count = 0;
    for a in 0..=na {
        for b in 0..=nb {
            for c in 0..=nc {
                if x == 500 * a + 100 * b + 50 * c {
                    count += 1
                }
            }
        }
    }

    count
}
