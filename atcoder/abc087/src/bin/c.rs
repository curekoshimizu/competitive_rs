use argio::argio;

#[argio]
fn main(n: u32, a: [u32; n], b: [u32; n]) -> u32 {
    let mut a_total = vec![0; n as usize];
    let mut b_total = vec![0; n as usize];

    let mut prev = 0;
    for (i, s) in a.iter().zip(a_total.iter_mut()) {
        *s = prev + i;
        prev = *s;
    }

    let mut prev = 0;
    for (i, s) in b.iter().rev().zip(b_total.iter_mut().rev()) {
        *s = prev + i;
        prev = *s;
    }

    let mut c_total = vec![0; n as usize];

    for i in 0..a_total.len() {
        c_total[i] = a_total[i] + b_total[i];
    }

    c_total.into_iter().max().unwrap()
}
