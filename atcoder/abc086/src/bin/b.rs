use argio::argio;

#[argio]
fn main(a: String, b: String) -> &str {
    let c = a + &b;
    let n = c.parse().unwrap();

    for i in 0..n {
        let m = i * i;
        if m > n {
            break;
        }
        if m == n {
            return "Yes";
        }
    }

    return "No";
}
