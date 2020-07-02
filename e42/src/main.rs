//
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e:{:?}, {:?} seconds", e(), now.elapsed());
}

fn e() -> usize {
    let v = get_triangle_nums_up_to(20);
    let file = include_str!("./p042_words.txt") // nifty lil macro
        .chars()
        .filter(|&x| x != '\"' && x != '\n' && x != '\"')
        .collect::<String>();
    file.split(',')
        //.take(10)
        //.inspect(|s| println!("got {}", s))
        .map(|s| word_score(s))
        //.inspect(|n| println!("has score {}", n))
        .filter(|n| v.contains(n))
        .count()
}

fn get_triangle_nums_up_to(nth: u32) -> Vec<u32> {
    (1..nth + 1).map(|i| i * (i + 1) / 2).collect()
}

fn word_score(s: &'_ str) -> u32 {
    s.chars().map(|c| c.to_digit(36).unwrap() - 9).sum()
}

// Results
// e:162, 7.733987ms seconds
