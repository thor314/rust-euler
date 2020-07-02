/// Common functions in Project Euler
fn get_char_ind(s: &str, i: usize) -> u32 {
    s.chars().nth(i).unwrap().to_digit(10).unwrap()
}

#[derive(Debug)]
struct Fraction {
    n: u32,
    d: u32,
}
impl Fraction {
    fn new(n: u32, d: u32) -> Option<Fraction> {
        match n <= d {
            true => Some(Fraction { n, d }),
            false => None, // just say no kids
        }
    }
    fn is_curious(&self) -> bool {
        if self.d % 10 == 0 {
            false
        } else if self.n % 10 != self.d / 10 {
            false
        } else {
            self.n as f32 / self.d as f32 == (self.n / 10) as f32 / (self.d % 10) as f32
        }
    }
    fn reduce(self) -> Fraction {
        // De-juiced, reduced new fraction on the loose
        let div = Fraction::gcd(self.n, self.d);
        Fraction {
            n: self.n / div,
            d: self.d / div,
        }
    }
    fn gcd(a: u32, b: u32) -> u32 {
        if a == b || b == 0 {
            return a;
        }
        if b == 0 {
            return b;
        }
        match (a % 2 == 0, b % 2 == 0) {
            (true, true) => Fraction::gcd(a / 2, b / 2) * 2,
            (true, false) => Fraction::gcd(a / 2, b),
            (false, true) => Fraction::gcd(a, b / 2),
            (false, false) => {
                if a > b {
                    Fraction::gcd((a - b) / 2, b)
                } else {
                    Fraction::gcd((b - a) / 2, a)
                }
            }
        }
    }
}
use std::ops::Mul;
impl Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let (n, d) = (self.n * rhs.n, self.d * rhs.d);
        let div = Fraction::gcd(n, d);
        Fraction::new(n / div, d / div).unwrap()
    }
}
use std::ops::MulAssign;
impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Fraction) {
        let (n, d) = (self.n * rhs.n, self.d * rhs.d);
        let div = Fraction::gcd(n, d);
        std::mem::replace(self, Fraction::new(n / div, d / div).unwrap());
    }
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

pub fn divisor_sum(n: usize) -> usize {
    // get the sum of all of n's divisors
    match n {
        0 | 1 => 0,
        _ => {
            let max = ((n as f64).sqrt() + 1.0) as usize;
            (2..max).fold(1, |acc, x| {
                if n % x == 0 {
                    // fast algorithm to find all of n's factors
                    let d = n / x;
                    if d == x {
                        // special case: x^2 = n = d^2, don't add d twice
                        acc + d
                    } else {
                        acc + x + d
                    }
                } else {
                    acc
                }
            })
        }
    }
}

pub fn divisor_sum_list(limit: usize) -> Vec<usize> {
    // Get the array of the number of times x goes into n
    let mut xs = vec![0; limit + 1];
    for i in 1..limit / 2 + 1 {
        let mut j = 2 * i;
        while j <= limit {
            xs[j] += i;
            j += i
        }
    }
    xs
}

pub fn sum_of_digits(s: &'_ str) -> usize {
    // sum of the digits of s. call: f(&format!(some_num))
    s.chars()
        .iter()
        .fold(0, |acc, x| acc + x.to_digit(10).unwrap()) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        true
    }
}
