// Predicates for Project Euler

fn is_palindrome(s: &'_ str) -> bool {
    // compare the first half of a string to the second half -^\o/^-
    let half = s.len() / 2;
    s.bytes().take(half).eq(s.bytes().rev().take(half))
}

fn is_pandigital(s: &'_ str) -> bool {
    // ie, does s contain only unique digits
    use ::std::ops::Not;
    s.chars()
        .enumerate()
        .all(|(i, v)| s[(i + 1)..].contains(v).not())
}

fn is_left_trunk(p: u64) -> bool {
    use primal::is_prime;
    let b: bool = std::iter::repeat_with({
        let mut i = 0;
        move || {
            i += 1;
            p / 10_u64.pow(i)
        }
    })
    //.inspect(|&p| println!("lt: {}", p))
    .take((len_int(p) - 1) as usize)
    .fold(true, |acc, x| acc && is_prime(x));
    b
}
fn is_right_trunk(p: u64) -> bool {
    use primal::is_prime;
    let b: bool = std::iter::repeat_with({
        let mut i = len_int(p) as u32;
        move || {
            i -= 1;
            p % 10_u64.pow(i)
            // eg p 297 % 10^(3-2) = 97, then 7.
        }
    })
    //.inspect(|&p| println!("rt: {}", p))
    .take((len_int(p) - 1) as usize)
    .fold(true, |acc, x| acc && is_prime(x));
    b
}

fn is_odd_digits(p: &usize) -> bool {
    // true if p is composed solely of odd digits.
    let s: &str = &format!("{}", p); // 0
                                     // 1
    !s.contains("0")
        && !s.contains("2")
        && !s.contains("4")
        && !s.contains("5")
        && !s.contains("6")
        && !s.contains("8")
}

fn is_cycle_prime(p: &usize) -> bool {
    // if every cyclic permutation of p is prime, give true.
    let s: String = format!("{}", p);
    for i in 1..s.len() {
        let mut st = String::from(&s[i..]);
        st.push_str(&s[..i]);
        if !is_prime(st.parse().unwrap()) {
            return false;
        }
    }
    true
}
