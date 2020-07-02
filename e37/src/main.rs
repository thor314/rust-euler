//Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e37:{:?}, {:?} seconds", e(), now.elapsed());
    //println!("debug:{:?}", is_left_trunk(3797));
    //println!("debug:{:?}", is_right_trunk(3797));
}

fn e() -> usize {
    use primal::Primes;
    Primes::all()
        .filter(|&p| p > 10)
        .filter(|&p| is_left_trunk(p as u64)) // left
        //.inspect(|&p| println!("b: {}", p))
        .filter(|&p| is_right_trunk(p as u64)) // right
        //.inspect(|&p| println!("c: {}", p))
        .take(11)
        //.collect()
        .sum()
}

fn is_left_trunk(p: u64) -> bool {
    use primal::is_prime;
    let b: bool = std::iter::repeat_with({
        let mut i = 0;
        move || {
            i += 1;
            p / 10_u64.pow(i)
        }
    })
    //.inspect(|&p| println!("lt: {}", p))
    .take((len_int(p) - 1) as usize)
    .fold(true, |acc, x| acc && is_prime(x));
    b
}
fn is_right_trunk(p: u64) -> bool {
    use primal::is_prime;
    let b: bool = std::iter::repeat_with({
        let mut i = len_int(p) as u32;
        move || {
            i -= 1;
            p % 10_u64.pow(i)
            // eg p 297 % 10^(3-2) = 97, then 7.
        }
    })
    //.inspect(|&p| println!("rt: {}", p))
    .take((len_int(p) - 1) as usize)
    .fold(true, |acc, x| acc && is_prime(x));
    b
}

fn len_int(n: u64) -> u64 {
    std::iter::repeat_with({
        let mut l = 0;
        move || match n / 10u64.pow(l) {
            0 => 0,
            _ => {
                l += 1;
                1
            }
        }
    })
    .take_while(|&x| x != 0)
    .count() as u64
}
