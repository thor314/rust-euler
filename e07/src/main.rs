//What is the 10 001st prime number?
use primal::Primes;
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("result: {:?}, time: {:?}", e7(), now.elapsed());
}

fn e7() -> usize {
    let p: usize = Primes::all().nth(10001 - 1).unwrap();
    p
}

// Results
// result: 104743, time: 9.343731ms
