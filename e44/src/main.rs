/*
Find the pair of pentagonal numbers, Pj and Pk, for which their sum and difference are pentagonal and D = |Pk − Pj| is minimised; what is the value of D?
*/
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e:{:?}, {:?} seconds", _better_e(), now.elapsed());
}

//use std::thread;
use itertools::Itertools;
fn e() {
    // get next pentagonal number
    let pent_clos = |i: i64| i * (i * 3 - 1) / 2;
    // turns out membership checks is pretty damn slow, much better to just have a predicate
    // could maybe do this better with a hash map if I /had/ to do this though
    // let first_1000_pentagonals = (1..2500).map(|x| pent_clos(x)).collect::<Vec<i64>>();
    let first_1000_it = (1..2500_i64).map(|x| pent_clos(x));
    let it_clone = first_1000_it.clone();
    // create the cartesian product of the first 1000 pentagonals, filter by conditions, collect into result.
    let res_it = first_1000_it
        .cartesian_product(it_clone)
        .filter(|(x, y)| is_pentagonal(x + y) && is_pentagonal(x - y));
    let res = res_it.collect::<Vec<(i64, i64)>>();
    println!("{:?}", res);
}

//predicates predicates
fn is_pentagonal(p: i64) -> bool {
    // p is pentagonal if p = n(3n-1)/2 has a whole number solution for n; or rather
    // (3/2)n^2 - (1/2)n - p = 0; apply quad formula
    // 3n^2 - n - 2p = 0; only want positive soln
    // n = (1/6)*(1+ sqrt(1 +4*3*2*p)) <- if that exists
    let n = (1.0 + (1.0 + 24.0 * p as f64).sqrt()) / 6.0; // the .0 bit is important here
                                                          // fun fact, the f64 std lib is great
    use std::f64::EPSILON;
    n - n.floor() < EPSILON
}

fn _better_e() {
    // a better solution wouldn't have an upper bound of 2500, would just stop when it found a value.
    //
    let pent_clos = |i: i64| i * (i * 3 - 1) / 2;
    for (i, j) in (1..).map(|x| (x, pent_clos(x))) {
        // i = index, j = ith pentagonal.
        for k in (1..i).map(|x| pent_clos(x)) {
            //
            if is_pentagonal(j - k) && is_pentagonal(k + j) {
                println!("{}", j - k);
                return;
            }
        }
    }
}

// Here begins some unrelated practice with structs and stuff

#[derive(Debug)]
struct Pentagonal {
    i: usize, // 0,1,2, 3, 4..
    n: usize, // 0,1,5,12,22..
}

impl Pentagonal {
    #[allow(dead_code)]
    fn from(i: usize) -> Self {
        Pentagonal {
            i: i + 1,
            n: i * (i * 3 - 1) / 2,
        }
    }
    #[allow(dead_code)]
    fn new() -> Self {
        Pentagonal { i: 1, n: 1 }
    }
    #[allow(dead_code)]
    fn is_sum_and_diff_pent(&self, rhs: &Pentagonal) -> bool {
        Pentagonal::is_pentagonal((self.n - rhs.n) as isize)
            && Pentagonal::is_pentagonal((self.n + rhs.n) as isize)
    }

    fn is_pentagonal(n: isize) -> bool {
        let (x0, x1) = Pentagonal::factor_quadratic(3, -1, -2 * n);
        // check roots
        let zero = |x| 3 * x * x - x - 2 * n == 0;
        zero(x0) || zero(x1)
    }
    fn factor_quadratic(a: isize, b: isize, c: isize) -> (isize, isize) {
        // sign is +-1
        let quad = |sign: isize| {
            (b * -1 + sign * f64::sqrt((b * b - 4 * a * c) as f64) as isize) / (2 * a)
        };
        (quad(1), quad(-1))
    }
    #[allow(dead_code)]
    fn are_unseen_pent_sum_and_diff(
        self,
        seen: Vec<Pentagonal>,
    ) -> Option<(Pentagonal, Pentagonal)> {
        // if sum AND diff of new pent and any old pent are both pent, return tuple
        let res = seen
            .iter()
            .map(|i| {
                (
                    seen.contains(&(self + *i)) && seen.contains(&(self - *i)),
                    i.i,
                )
            })
            .fold((false, 0), |acc, (x, i)| (x || acc.0, i));

        match res.0 {
            false => None,
            true => Some((self, Pentagonal::from(res.1))),
        }
    }
}

impl Iterator for Pentagonal {
    type Item = Pentagonal;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Pentagonal {
            i: self.i + 1,
            n: self.i * (self.i * 3 - 1) / 2,
        })
    }
}
impl PartialEq for Pentagonal {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}
impl Copy for Pentagonal {}

impl Clone for Pentagonal {
    fn clone(&self) -> Self {
        *self
    }
}
use std::ops::Add;
impl Add for Pentagonal {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            i: self.i + other.i,
            n: self.n + other.n,
        }
    }
}

use std::ops::Sub;
impl Sub for Pentagonal {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            i: self.i - other.i,
            n: self.n - other.n,
        }
    }
}
