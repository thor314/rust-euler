/*
If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

{20,48,52}, {24,45,51}, {30,40,50}

For which value of p ≤ 1000, is the number of solutions maximised?
 */
/*
First we'll write a fn getting the right triangles for p.
Then we'll run p from 1 to 1000 and take the max.
*/
use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e:{:?}, {:?} seconds", e(), now.elapsed());
}

fn e() -> usize {
    let h = get_right_triangles_of_perimiter(500); // 1000 over 2

    let mut max_val = 0;
    for (k, val) in h.iter() {
        if *val > max_val {
            println!("{},{}", k, val);
            max_val = *val;
        }
    }
    max_val
}

use std::collections::HashMap;
fn get_right_triangles_of_perimiter(lim: usize) -> HashMap<usize, usize> {
    // note: because of the implementation, this function only works for even p.
    // lim should be half of the total desired perimeter of the triangle.
    let mut h: HashMap<usize, usize> = HashMap::new();
    (1..lim + 1)
        .flat_map(|a| (a..lim + 1).map(move |b| (a, b)))
        .flat_map(|(a, b)| (b..lim + 1).map(move |c| (a, b, c)))
        .filter(|(a, b, c)| a * a + b * b == c * c)
        .for_each(|(a, b, c)| {
            match h.get(&(a + b + c)) {
                None => h.insert(a + b + c, 1),
                _ => h.insert(a + b + c, 1 + h.get(&(a + b + c)).unwrap()),
            };
        });
    h
}
