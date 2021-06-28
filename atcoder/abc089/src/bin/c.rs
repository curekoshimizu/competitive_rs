use argio::argio;

#[argio]
fn main(n: u64, origin: [String; n]) -> u64 {
    let checker = [
        "MAR", "MAC", "MAH", "MRC", "MRH", "MCH", "ARC", "ARH", "RCH", "ACH",
    ];

    let mut table = [0u64; 5];

    origin
        .iter()
        .for_each(|name| match name.chars().nth(0).unwrap() {
            'M' => table[0] += 1,
            'A' => table[1] += 1,
            'R' => table[2] += 1,
            'C' => table[3] += 1,
            'H' => table[4] += 1,
            _ => {}
        });

    let mut result = 0u64;

    for check in checker.iter() {
        let i0 = index(check.chars().nth(0).unwrap());
        let i1 = index(check.chars().nth(1).unwrap());
        let i2 = index(check.chars().nth(2).unwrap());

        let local_result = table[i0] * table[i1] * table[i2];

        result += local_result;
    }

    result
}

fn index(c: char) -> usize {
    match c {
        'M' => 0,
        'A' => 1,
        'R' => 2,
        'C' => 3,
        'H' => 4,
        _ => panic!("!?"),
    }
}
