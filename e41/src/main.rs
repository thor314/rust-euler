//
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e:{:?}, {:?} seconds", e(), now.elapsed());
}

use primal::Primes;
fn e() -> usize {
    // 30s
    let p: Vec<usize> = Primes::all()
        .take_while(|&p| p < 10000000) //guess on size, fiddle with it til right
        .filter(|&p| is_n_pandigital(&p.to_string(), 8)) / /fiddle with the 8
        .collect();
    *p.iter().max().unwrap()
}

fn is_n_pandigital(s: &'_ str, n: usize) -> bool {
    // ie, does s contain only unique digits
    use ::std::ops::Not;
    let pan = s
        .chars()
        .enumerate()
        .all(|(i, v)| s[(i + 1)..].contains(v).not());
    pan && (1..n).all(|i| s.contains(&i.to_string()))
}

#[allow(dead_code)]
fn is_pandigital(s: &'_ str) -> bool {
    // ie, does s contain only unique digits
    use ::std::ops::Not;
    s.chars()
        .enumerate()
        .all(|(i, v)| s[(i + 1)..].contains(v).not())
}

// Results
// e:7652413, 1.068651521s seconds
