// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("result: {:?}, time: {:?}", e5(), now.elapsed());
}

fn e5() -> usize {
    16 * 9 * 5 * 7 * 11 * 13 * 17 * 19
}

// Results
/*
result: 232792560, time: 43ns
*/
