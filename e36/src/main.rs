/*
The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
(Please note that the palindromic number, in either base, may not include leading zeros.)
 */
//
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e36:{:?}, {:?} seconds", e(), now.elapsed());
    let now = Instant::now();
    println!("e36':{:?}, {:?} seconds", e_1(), now.elapsed());
}
//
fn e() -> u32 {
    (1_u32..1000000_u32 as u32)
        .filter(|x| is_palindrome(&format!("{}", x)))
        .filter(|&x| is_palindrome(&format!("{}", to_binary(x, 0, String::new()))))
        .sum()
}

fn e_1() -> u32 {
    (1_u32..1000000_u32 as u32)
        .filter(|x| is_palindrome(&format!("{}", x)))
        .filter(|&x| is_palindrome(&format!("{}", to_binary_mut(x))))
        .sum()
}

fn is_palindrome(s: &'_ str) -> bool {
    // compare the first half of a string to the second half -^\o/^-
    let half = s.len() / 2;
    s.bytes().take(half).eq(s.bytes().rev().take(half))
}
fn to_binary(x: u32, i: u32, s: String) -> String {
    // best set i i to zero son. usage: to_binary(x,0,"")
    if x == 0 {
        return s;
    } else {
        match x % 2u32.pow(i + 1) {
            0 => to_binary(x, i + 1, s + "0"),
            _ => to_binary(x - 2u32.pow(i), i + 1, s + "1"),
        }
    }
}

fn to_binary_mut(x: u32) -> String {
    let mut xc = x.clone();
    let mut b = String::new();
    let mut i = 0;
    while xc > 0 {
        if xc % 2u32.pow(i + 1) == 0 {
            b.push('0');
        } else {
            xc -= 2u32.pow(i);
            b.push('1');
        }
        i += 1;
    }
    return b;
}
// Results
/*
e36:872187, 539.900292ms seconds
e36':872187, 534.356925ms seconds
*/
