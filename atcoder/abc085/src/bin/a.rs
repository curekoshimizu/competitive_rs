use argio::argio;

#[argio]
fn main(s: String) -> String {
    s.replace("2017", "2018")
}
