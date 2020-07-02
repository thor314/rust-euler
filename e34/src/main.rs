// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
use e34::*;
use std::time::Instant;
fn main() {
    assert!(is_curious(145));
    let now = Instant::now();
    println!("e34: {:?}, {:?} seconds", e(), now.elapsed());
    let now = Instant::now();
    println!("e34_f: {:?}, {:?} seconds", e_f(), now.elapsed());
}

// How might we know the upper limit of this?
// Well. 9! = 362880. So 999,999 -> 2,177,280. We can stop at 2177280.
