// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("result: {:?}, time: {:?}", e6(), now.elapsed());
}

fn e6() -> usize {
    let sum_of_squares: usize = (1..101).fold(0, |acc, i| acc + i * i);
    let square_of_sums: usize = 5050 * 5050;
    square_of_sums - sum_of_squares
}

// Results
// result: 25164150, time: 15.099µs
