use argio::argio;

#[argio]
fn main(s: String) -> String {
    s.chars()
        .map(|s| {
            if s == '1' {
                return "9".to_string();
            } else if s == '9' {
                return "1".to_string();
            } else {
                return s.to_string();
            }
        })
        .collect()
}
