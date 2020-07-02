// What is the largest prime factor of the number 600851475143 ?
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("imperative lazy: {:?}", e3_imperative_lazy(600851475143));
    println!("time:{:?}", now.elapsed());

    let now = Instant::now();
    println!(
        "imperative less lazy: {:?}",
        e3_imperative_less_lazy(600851475143)
    );
    println!("time:{:?}", now.elapsed());

    let now = Instant::now(); // 3.
    println!("imperative: {:?}", e3_imperative(600851475143));
    println!("time:{:?}", now.elapsed());

    let now = Instant::now();
    println!("functional lazy: {:?}", e3_functional_lazy(600851475143));
    println!("time:{:?}", now.elapsed());

    let now = Instant::now();
    println!("recursive: {:?}", e3_recursive(600851475143, 3, 0));
    println!("time:{:?}", now.elapsed());
}

use primes; // primes = "0.2.3"
fn e3_imperative_lazy(n: u64) -> u64 {
    // Factoring. Check up to sqrt n, only update max if the new value is prime.
    let mut max: u64 = n;
    // 0., 2.
    for i in 1..((n as f64).sqrt() as u64) {
        if n % i == 0 && primes::is_prime(i) {
            max = i;
        }
    }
    max
}

fn e3_imperative_less_lazy(n: u64) -> u64 {
    // be slightly less lazy, and don't use primes
    let mut max: u64 = n;
    let mut n_c = n.clone();
    let mut i = 3; // assume 2 not a divisor of n
    while n_c > 1 {
        if n_c % i == 0 {
            n_c /= i;
            max = i;
        } else {
            i += 2; // apparently this is much faster (15-20x) than the approach below
        }
    }
    max
}

use primes::PrimeSet;
fn e3_imperative(n: u64) -> u64 {
    // be even more less lazy, and use the primeSet for iteration
    let mut pset = PrimeSet::new();
    let mut max: u64 = n;
    let mut n_c = n.clone();
    let mut i = pset[1];
    while n_c > 1 {
        if n_c % i == 0 {
            n_c /= i;
            max = i;
        } else {
            pset.expand();
            i = pset[pset.len() - 1];
        }
    }
    max
}

fn e3_functional_lazy(n: u64) -> u64 {
    // Set it up and knock it down, functional style. Could be filtering far fewer elements though. Slow.
    (1..((n as f64).sqrt().floor() as u64))
        .filter(|i| n % i == 0 && primes::is_prime(*i))
        .max()
        .unwrap()
}

fn e3_recursive(n: u64, i: u64, max_seen: u64) -> u64 {
    // Similar to "imperative less lazy", just implemented recursively.
    if n == 1 {
        max_seen
    } else if n % i == 0 {
        e3_recursive(n / i, i, i)
    } else {
        e3_recursive(n, i + 2, max_seen)
    }
}

// Learnings
/*
0. don't even need to call floor after sqrt()
1. I could further accelerate these functions by indexing primes, rather than just 1..sqrt n
2. turns out if we don't check for primality, we get a composite here.
3. Super suprised that the third iterative I wrote was actually slower than the second. Even when I threw in a 9 digit prime, "imperative less lazy" was 14x faster than "imperative".
4. This was the first problem I wrote where the recursive solution wasn't fastest.
*/

// Output
/*
imperative lazy: 6857
time:72.047958ms
imperative less lazy: 6857
time:72.913µs
imperative: 6857
time:1.146705ms
functional lazy: 6857
time:108.920048ms
recursive: 6857
time:238.561µs
*/
