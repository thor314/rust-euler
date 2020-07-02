// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
// Written two ways: recursively and functionally
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e30, functional: {:?}", e30());
    println!("time:{:?}", now.elapsed());

    use std::thread; // Standard thread cap is 100,000, need about 360,000
    let now = Instant::now();
    thread::Builder::new()
        .stack_size(360424 * 0xFF)
        .spawn(move || {
            println!("e30, recursive: {:?}", e30_recursive(2, 0));
        })
        .unwrap()
        .join() // sort of like collect for threads
        .unwrap();
    println!("time:{:?}", now.elapsed());
}

// Observe that 9^5 = 59049, and 6*that=354294. Thus we only need to search up to 354294.

fn e30() -> u32 {
    //
    // A dandy little helper closure to get our digits' fifth power
    let digits_fifth_pow = |mut n: u32| -> u32 {
        let mut sum = 0;
        while n != 0 {
            sum += (n % 10).pow(5);
            n /= 10;
        }
        sum
    };

    let ans: u32 = (2..354294).fold(0, |acc, x| {
        if x == digits_fifth_pow(x) {
            x + acc
        } else {
            acc
        }
    });
    ans
}

fn digits_fifth_pow(mut n: u32) -> u32 {
    let mut sum = 0;
    while n != 0 {
        sum += (n % 10).pow(5);
        n /= 10;
    }
    sum
}

fn e30_recursive(cur: u32, run_sum: u32) -> u32 {
    if cur >= 354294 {
        run_sum
    } else if cur == digits_fifth_pow(cur) {
        e30_recursive(cur + 1, run_sum + cur)
    } else {
        e30_recursive(cur + 1, run_sum)
    }
}

// Results
/*
e30, functional: 443839
time:54.826842ms
e30, recursive: 443839
time:46.443784ms
*/
