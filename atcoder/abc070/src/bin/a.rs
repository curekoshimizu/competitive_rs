// keywords : zip
use argio::argio;

#[argio]
fn main(s: String) -> &str {
    for (x, y) in s.chars().zip(s.chars().rev()) {
        if x != y {
            return "No";
        }
    }

    return "Yes";
}
