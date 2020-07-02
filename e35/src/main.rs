use primal::{is_prime, Primes};
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e35:{:?}, {:?} seconds", e(), now.elapsed());
}

fn e() -> usize {
    Primes::all()
        .take_while(|&p| p < 1000000)
        .filter(|p| is_odd_digits(p))
        .filter(|p| is_cycle_prime(p))
        .count()
}

fn is_odd_digits(p: &usize) -> bool {
    // true if p is composed solely of odd digits.
    let s: &str = &format!("{}", p); // 0
                                     // 1
    !s.contains("0")
        && !s.contains("2")
        && !s.contains("4")
        && !s.contains("5")
        && !s.contains("6")
        && !s.contains("8")
}

fn is_cycle_prime(p: &usize) -> bool {
    // if every cyclic permutation of p is prime, give true.
    let s: String = format!("{}", p);
    for i in 1..s.len() {
        let mut st = String::from(&s[i..]);
        st.push_str(&s[..i]);
        if !is_prime(st.parse().unwrap()) {
            return false;
        }
    }
    true
}
// Results
//e35:53, 82.8052ms seconds

// This would be much faster if I
// rewrote my predicates to use int manipulation, rather
// than strings. But more tedious.
