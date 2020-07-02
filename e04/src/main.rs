// Find the largest palindrome made from the product of two 3-digit numbers.
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("imperative: {:?}", e4_imperative());
    println!("time:{:?}", now.elapsed());

    let now = Instant::now();
    println!("functional: {:?}", e4_functional());
    println!("time:{:?}", now.elapsed());

    let now = Instant::now();
    println!("functional2: {:?}", e4_functional2());
    println!("time:{:?}", now.elapsed());

    let now = Instant::now();
    println!("functional3: {:?}", e4_functional3());
    println!("time:{:?}", now.elapsed());

    let now = Instant::now();
    println!("recursive: {:?}", e4_recursive(800, 800, 0));
    println!("time:{:?}", now.elapsed());
}
fn is_pal(n: u32) -> bool {
    // rust does not natively support string indexing, so this looks a lil funny
    let s = n.to_string();
    let half = s.len() / 2;
    s.bytes().take(half).eq(s.bytes().rev().take(half))
}

fn e4_imperative() -> u32 {
    // iterate through all possible combinations of 3 digit ints, check for paly and largest.
    // Takes nearly a second for 100..999, but since we can assume both ints are probably greater than 800, that reduces time to about 50ms.
    let mut lar_pal = 0;
    for i in 800..999 {
        for j in i..999 {
            let t = i * j;
            if is_pal(t) && t > lar_pal {
                lar_pal = t;
            }
        }
    }
    lar_pal
}

use itertools::iproduct;
fn e4_functional() -> u32 {
    // use cartesian product, take products, filter for palindromes, get the max
    iproduct!(800..999, 800..999)
        .map(|p| p.0 * p.1)
        .filter(|&x| is_pal(x))
        .max()
        .unwrap()
}

/*
v.iter()
 .enumerate()
 .flat_map (|(i, a)| v[i+1..].iter().map (move |b| a+b))
 .unique()    // May not be needed or what you really want, see note below
 .sum()
 */

use itertools::Itertools;
fn e4_functional2() -> u32 {
    // use cartesian product, take products, filter for palindromes, get the max.
    // About twice as fast.
    (800..999)
        .tuple_combinations() // itertools, passes duplicates
        .map(|p: (u32, u32)| p.0 * p.1)
        .filter(|&x| is_pal(x))
        .max()
        .unwrap()
}

fn e4_functional3() -> u32 {
    // because we're speed demons, let's push it even faster with flat_map, which is a way of flattening a structure that we'd otherwise have to use a tuple structure like the one above for.
    let v: Vec<u32> = (800..999).collect();
    v.iter()
        .enumerate()
        .flat_map(|(i, a)| v[i..].iter().map(move |fm| a * fm))
        .filter(|&x| is_pal(x))
        .max()
        .unwrap()
}

fn e4_recursive(i: u32, j: u32, max_seen: u32) -> u32 {
    // sortof a tricky problem for recursion.
    // naive implementation
    let m = i * j;
    if i == 1000 {
        max_seen
    } else if j == 1000 {
        e4_recursive(i + 1, i + 1, max_seen)
    } else if is_pal(m) && m > max_seen {
        e4_recursive(i, j + 1, m)
    } else {
        e4_recursive(i, j + 1, max_seen)
    }
}

// Learnings
/*
This is the first time a functional algorithm has won the speed race! Very close each of them though.
 */

// Output
/*
imperative: 906609
time:27.187472ms
functional: 906609
time:45.665353ms
functional2: 906609
time:21.986141ms
functional3: 906609
time:20.65895ms
recursive: 906609
time:22.331524ms
*/
