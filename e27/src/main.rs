// I screwed around a lot here.
//  learned how to time a function!
/*
use std::time::Instant;
let now = Instant::now();
println!("time:{:?}", now.elapsed());
 */
// actually a key insight to making this code work: If I began with a &str, then the items would have known size at compile time; but with String typing they do not. Thus this has to happen in 2 steps, String-> &str, &str -> u32.
use primes::is_prime;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::time::Instant;

#[macro_use]
extern crate itertools;

fn main() -> std::io::Result<()> {
    let now = Instant::now();
    // get first 10k primes
    let mut file = File::open("first10kprimes.txt")?; // largest is 104729
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v: Vec<i32> = contents
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let vlt1000 = v
        .iter()
        .copied()
        .take_while(|&x| x < 1000)
        .collect::<Vec<_>>();
    // Problem starts here, rest above not used
    // search space
    let ss: Vec<(i32, &i32)> = iproduct!((-1000..1000), &vlt1000)
        .filter(|prod| prod.0 + prod.1 > 0)
        .filter(|prod| is_prime((1 + prod.0 + prod.1) as u64))
        .filter(|prod| 36 + 6 * prod.0 + prod.1 > 0)
        .filter(|prod| is_prime((36 + 6 * prod.0 + prod.1) as u64))
        .filter(|prod| 49 + 7 * prod.0 + prod.1 > 0)
        .filter(|prod| is_prime((49 + 7 * prod.0 + prod.1) as u64))
        .collect();
    // down to 493 pairs the dumb way

    let mut max = (0, 0);
    let mut maxv = 0;
    for (a, b) in ss {
        let fv = f((a, *b));
        if fv > maxv {
            maxv = fv;
            max = (a, *b);
        }
    }
    println!("max is {}, w values {:?}", maxv, max);
    //    let l: i32 = ssm.iter().for_each(|x|

    //    println!(" {:?}, {}", ssm, ssm.len());

    //    println!("{:?}", ss);
    println!("time:{:?}", now.elapsed());
    Ok(())
}
fn f(n: (i32, i32)) -> u64 {
    let mut z: i32 = 0;
    for i in 1..100 {
        //        println!("{}, {}", n.0, n.1);
        if !is_prime((i * i + i * n.0 + n.1) as u64) {
            //+ n.1) {
            z = i - 1;
            break;
        }
    }
    z as u64
}
// below here not used

fn _get_int_from_str() {
    //testing ways to get an int from a str: parse and from_str. directly from docs.

    // from_str (have to import it), is simple
    let s = "54321";
    let x = i32::from_str(s).unwrap();

    // parse, works with str and String
    let my_string = "27".to_string(); // `parse()` works with `&str` and `String`!
    let my_int = my_string.parse::<i32>().unwrap();
    let _my_u8: u8 = "42".parse::<u8>().unwrap();
    let _my_u32: u32 = "42".parse::<u32>().unwrap();

    // parse with Err matching safety
    match "foobar".parse::<i32>() {
        Ok(n) => println!("n was {}", n),
        Err(_e) => println!("weep_and_moan"),
    }

    println!("x was {}; my_int was {}", x, my_int);
}

fn _printy(v: Vec<&str>) {
    // testing getting the length of a vector
    println!(
        "length of v is {}, and v is, {:?}",
        v.len(),
        &v[v.len() - 20..]
    );
}
