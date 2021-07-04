// keywords :

use input_lib::*;
use prime::is_prime;

fn main() {
    input! {
        num: u32,
        a: [u64; num],
    }

    let result = a.iter().filter(|&i| is_prime(*i)).count();

    println!("{}", result);
}

// fail...
// because n = 10^8 and SieveEratosthenes order is O(n loglog n)
// use input_lib::*;
// use prime::SieveEratosthenes;
//
// fn main() {
//     input! {
//         num: u32,
//         a: [u32; num],
//     }
//
//     let sieve = SieveEratosthenes::new(*a.iter().max().unwrap() as usize);
//
//     let result = a.iter().filter(|&i| sieve.is_prime[*i as usize]).count();
//
//     println!("{}", result);
// }
