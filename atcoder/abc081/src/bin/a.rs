// keywords : string
use argio::argio;

#[argio]
fn main() {
    solver_1()
}

#[argio]
fn solver_1(s: String) -> u32 {
    s.chars().filter(|&x| x == '1').count() as u32
}

#[argio]
fn solver_2(s: String) -> u32 {
    s.chars().map(|x| x.to_digit(10).unwrap()).sum()
}

#[argio]
fn solver_3(s: String) -> u32 {
    s.as_bytes().into_iter().map(|x| x - b'0').sum::<u8>() as u32
}
