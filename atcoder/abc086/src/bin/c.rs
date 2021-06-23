use argio::argio;

#[argio]
fn main(n: u32, a: [(i32, i32, i32); n]) -> &str {
    let (mut t, mut x, mut y) = (0, 0, 0);

    for (t1, x1, y1) in a.into_iter() {
        let dt = t1 - t;
        let dx = (x1 - x).abs();
        let dy = (y1 - y).abs();

        let ans = dt - dx - dy;
        if !(ans >= 0 && ans % 2 == 0) {
            return "No";
        }

        t = t1;
        x = x1;
        y = y1;
    }

    return "Yes";
}
