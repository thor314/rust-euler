/* What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
Where lexicographic permutation is defined the ordered sequence of permutations, ie 01, 10, or 012, 021, 102,...
Observe that we may simply find the solution with a factorial function.
 */

fn factorial(a: u8) -> i64 {
    if a == 0 {
        1 as i64
    } else {
        let agg: i64 = (1..a as i64 + 1).product();
        agg
    }
}

#[test]
fn factorial_test() {
    for a in 1..10 {
        println!("fac test: {}! is {}", a, factorial(a));
    }
}
/*
fn facto_string(k: &i64, guess: u8) -> String {
    ///
    let s: String::new();
    while{

    }
    s
}
*/

// turns out that I just did this one by hand.
fn main() {
    let mut want: i64 = 1000000;
    let mut s = String::from("");
    for i in (0..10_u8).rev() {
        for _ in 0..10 {
            if want - factorial(i - 1) > 0 {
                s.push(char::from(i));
                want -= factorial(i - 1) * i as i64;
            }
        }
    }
    println!("the resultant string was {}", s);
}
