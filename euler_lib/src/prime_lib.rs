// credit goes to github.com/roycrippen/euler_rust for some of these, his style is great
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
fn fibonacci() -> Fibonacci {
    Fibonacci { a: 1, b: 1 }
}

pub fn prime_factors_recursive(n: usize, start_at: usize, mut v: Vec<usize>) -> Vec<usize> {
    //Call f(n, 2,Vec<usize>::new())
    if start_at < 2 {
        // weird input, start at 2
        prime_factors_recursive(n, 2, v)
    } else if start_at > n / 2 {
        // base case
        v
    } else if start_at == 2 {
        // special case, can't add 2
        if n % 2 == 0 {
            v.push(2);
            prime_factors_recursive(n / 2, 2, v)
        } else {
            prime_factors_recursive(n, 3, v)
        }
    } else {
        // general case
        if n % start_at == 0 {
            v.push(start_at);
            prime_factors_recursive(n / start_at, start_at, v)
        } else {
            prime_factors_recursive(n, start_at + 2, v)
        }
    }
}

pub fn prime_factors_unique(n: usize) -> Vec<usize> {
    let mut xs = prime_factors_recursive(n, 2, Vec::<usize>::new());
    xs.dedup(); // removes duplicates
    xs
}

#[cfg(test)]
mod tests {
    #[test]
    fn prime_factors_works() {
        assert_eq!(prime_factors_unique(156 as usize), vec![2 as usize, 3, 13])
    }
}
