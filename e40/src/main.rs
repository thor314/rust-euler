//
/*
find d_1*d_10*...*d_10^6
of .123456789101112...
    ^d1      ^d10 ...
 */
// Thoughts
/*
10..99 = 89 ints * 2 char/int = 178 chars;
d_100 = index 91 = 4 ->5<-
100.999 = 899 ints * 3 chars/int = 267 chars; running total 267+178+9=456
1000..9999 = 8999 ints * 4 chars/int = 3596 chars
i_544: 544 mod 4 = 0
*/

use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e:{:?}, {:?} seconds", e(), now.elapsed());
		println!
}

fn e() -> u32 {
    let mut s = String::new();
    (1..1000000).for_each(|i| i.to_string().chars().for_each(|c| s.push(c)));
    //println!("{:?}", s.chars().nth(3).unwrap());
    [1, 10, 100, 1000, 10000, 100000, 1000000]
        .iter()
        .inspect(|&x| println!("x: {}", get_char_ind(&s, *x - 1)))
        .fold(1, |acc, x| acc * get_char_ind(&s, *x - 1))
}
fn get_char_ind(s: &str, i: usize) -> u32 {
    s.chars().nth(i).unwrap().to_digit(10).unwrap()
}
