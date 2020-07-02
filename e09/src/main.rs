// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//Find the product abc.

use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("result: {:?}, time: {:?}", e9(), now.elapsed());
    let now = Instant::now();
    println!("result: {:?}, time: {:?}", e9_i(), now.elapsed());
}

fn e9() -> usize {
    let x = (1..1000)
        .flat_map(|i| (i..1000).map(move |j| (i, j)))
        .filter(|(x, y)| {
            (((x * x + y * y) as f64).sqrt()) - (((x * x + y * y) as f64).sqrt()).ceil() == 0 as f64
        })
        .filter(|(x, y)| (((x * x + y * y) as f64).sqrt()) as usize + x + y == 1000)
        .collect::<Vec<(usize, usize)>>();
    &x[0].1 * &x[0].0 * ((x[0].0 * x[0].0 + x[0].1 * x[0].1) as f64).sqrt() as usize
}

fn e9_i() -> usize {
    for i in 1..1000 {
        for j in i..(1000 - i) {
            let k = 1000 - (i + j);
            if i * i + j * j == k * k {
                return i * j * k;
            }
        }
    }
    return 0;
}

// Result
// result: 31875000, time: 61.922854ms
