// Find the sum of all the primes below two million.
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("result: {:?}, time: {:?}", e10(), now.elapsed());
}
use primal::Primes;
fn e10() -> usize {
    Primes::all().take_while(|&p| p < 2000000).sum()
}
