pub fn e() -> u32 {
    let mut sum = 0;
    for i in 145..2177280 {
        if is_curious(i) {
            //            println!("debug {}", i);
            sum += i;
        }
    }
    sum
}
pub fn e1() -> u32 {
    let mut sum = 0;
    for i in 145..2177280 {
        if is_curious_alice_fac(i) {
            //            println!("debug {}", i);
            sum += i;
        }
    }
    sum
}

pub fn e2() -> u32 {
    let mut sum = 0;
    for i in 145..2177280 {
        if is_curious_alice_my_fac(i) {
            //            println!("debug {}", i);
            sum += i;
        }
    }
    sum
}

pub fn e3() -> u32 {
    let mut sum = 0;
    for i in 145..2177280 {
        if is_curious_alice_alice(i) {
            //            println!("debug {}", i);
            sum += i;
        }
    }
    sum
}

pub fn e_f() -> u32 {
    (145u32..2177280).filter(|&x| is_curious(x)).sum()
}

pub fn is_curious(x: u32) -> bool {
    // e34
    let sum: u32 = format!("{}", x)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        //        .map(factorial)
        //      .sum();
        .fold(0, |acc, y| factorial(y) + acc);
    sum == x
}
pub fn is_curious_alice_fac(x: u32) -> bool {
    // e34
    let sum: u32 = format!("{}", x)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(factorial)
        .sum();
    //   .fold(0, |acc, y| factorial(y) + acc);
    sum == x
}

pub fn is_curious_alice_my_fac(x: u32) -> bool {
    // e34
    let sum: u32 = format!("{}", x)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        //        .map(factorial)
        //      .sum();
        .fold(0, |acc, y| factorial(y) + acc);
    sum == x
}
pub fn is_curious_alice_alice(x: u32) -> bool {
    // e34
    let sum: u32 = format!("{}", x)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(factorial)
        .sum();
    //   .fold(0, |acc, y| factorial(y) + acc);
    sum == x
}

pub fn is_curious_slow(x: u32) -> bool {
    // using this slows runtime by roughly 2.3 times.
    let s: &str = &format!("{}", x);
    let v: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let sum = v.into_iter().fold(0, |acc, y| factorial(y) + acc);
    if sum == x {
        return true;
    }
    false
}
pub fn factorial_alice(n: u32) -> u32 {
    (2..=n).product()
}

pub fn factorial(x: u32) -> u32 {
    if x <= 1 {
        1
    } else {
        x * factorial(x - 1)
    }
}
