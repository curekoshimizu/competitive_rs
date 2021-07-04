// keywords :

use input_lib::*;
use prime::prime_factorization;

fn main() {
    input! {
        n: u64,
    }

    let mut primes = prime_factorization(n);
    primes.sort();

    println!(
        "{}: {}",
        n,
        primes
            .iter()
            .map(|&i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
