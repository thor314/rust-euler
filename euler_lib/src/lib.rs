/// functions having to do with primes for Project Euler
// credit goes to github.com/roycrippen/euler_rust for some of these, his style is great

fn len_int(n: u32) -> u32 {
    // 0
    std::iter::repeat_with({
        let mut l = 0;
        // can't call pow on ambiguous numeric type
        move || match n / 10u32.pow(l) {
            // 1
            0 => 0,
            _ => {
                l += 1;
                1
            }
        }
    })
    .take_while(|&x| x != 0)
    // count returns usize
    .count() as u32 // 2
}

#[derive(Debug)]
pub struct Fibonacci {
    a: usize,
    b: usize,
}
impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let next = (self.a + self.b, self.a);
        std::mem::replace(&mut (self.a, self.b), next);
        Some(self.a)
    }
}
impl PartialEq for Fibonacci {
    fn eq(&self, other: &Fibonacci) -> bool {
        self.a == other.a && self.b == other.b
    }

    /// This method tests for `!=`.
    #[inline]
    fn ne(&self, other: &Fibonacci) -> bool {
        !self.eq(other)
    }
}

pub fn fibonacci() -> Fibonacci {
    Fibonacci { a: 1, b: 1 }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn fib_works() {
        assert_eq!(fibonacci(), Fibonacci { a: 1, b: 1 });
    }
}
