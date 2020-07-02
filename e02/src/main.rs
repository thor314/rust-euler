// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

fn main() {
    let now = Instant::now();
    println!("imperative: {}", e2_imperative());
    println!("time:{:?}", now.elapsed());

    let now = Instant::now();
    println!("functional: {}", e2_functional());
    println!("time:{:?}", now.elapsed());

    let now = Instant::now();
    println!("functional2: {}", e2_functional2());
    println!("time:{:?}", now.elapsed());

    let now = Instant::now(); // 1.
    println!("recursive: {}", e2_recursive(4000000, 1, 1));
    println!("time:{:?}", now.elapsed());

    let now = Instant::now(); // 0.
    println!(
        "recursive2: {:?}",
        e2_recursive2(4000000, 1, 1, [].to_vec())
            .iter()
            .sum::<u32>()
    );
    println!("time:{:?}", now.elapsed());
}

struct Fib {
    index: u32,
    a: u32,
    b: u32,
}

impl Fib {
    fn new() -> Fib {
        Fib {
            index: 0,
            a: 1,
            b: 1,
        }
    }
    fn next(&mut self) {
        self.index += 1;
        let temp = self.a;
        self.a = self.a + self.b;
        self.b = temp;
    }
}

fn e2_imperative() -> u32 {
    // run through fibs less than 4m.
    let mut sum: u32 = 0;
    let mut f = Fib::new();
    while f.a < 4000000 {
        if f.a % 2 == 0 {
            sum += f.a;
        }
        f.next();
    }
    sum
}

fn e2_functional() -> u32 {
    // Use scan to capture fibonacci state, with take_while and filter to capture evens less than 4m
    let sum: u32 = (1..)
        .scan((1, 1), |state, _| {
            // inefficient, since not using (1..), see next for improvement
            let temp = state.0;
            state.0 = state.1 + state.0;
            state.1 = temp;
            Some(state.0)
        })
        .take_while(|&x| x < 4000000)
        .filter(|x| x % 2 == 0)
        .sum();
    sum
}

fn e2_functional2() -> u32 {
    // 0
    // Same as above, with a bit more Rust sugar-sophistication, 50ish% faster
    let sum: u32 = std::iter::repeat_with({
        // do stateful, repeated computation
        let mut state = (1, 1);
        move || {
            // capture state, move inside scope
            let next = (state.1, state.0 + state.1);
            std::mem::replace(&mut state, next).0 // moves next into state, returns state.0
        }
    })
    .take_while(|&x| x < 4000000)
    .filter(|x| x % 2 == 0)
    .sum();
    sum
}

fn e2_recursive(lim: u32, f0: u32, f1: u32) -> u32 {
    if f0 > lim {
        0
    } else if f0 % 2 == 0 {
        f0 + e2_recursive(lim, f0 + f1, f0)
    } else {
        e2_recursive(lim, f0 + f1, f0)
    }
}

fn e2_recursive2(lim: u32, f0: u32, f1: u32, mut v: Vec<u32>) -> Vec<u32> {
    if f0 > lim {
        v
    } else if f0 % 2 == 0 {
        v.push(f0);
        e2_recursive2(lim, f0 + f1, f0, v)
    } else {
        e2_recursive2(lim, f0 + f1, f0, v)
    }
}

// Learnings
/*
0. Recursive solution that adds, rather than creating a vector is interestingly much faster
1. Recursive solutions continue to be surprisingly fast
*/

// Output
/*
imperative: 4613732
time:34.237µs
functional: 4613732
time:13.705µs
functional2: 4613732
time:6.168µs
recursive: 4613732
time:1.869µs
recursive2: 4613732
time:15.49µs
*/
