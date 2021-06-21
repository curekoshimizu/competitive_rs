use argio::argio;

#[argio]
fn main(s: String) -> String {
    s.chars()
        .map(|s| match s {
            '1' => '9',
            '9' => '1',
            _ => s,
        })
        .collect()
}
