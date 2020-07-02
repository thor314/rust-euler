// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?

use std::time::Instant;
fn main() {
    let now = Instant::now();
    println!("e:{:?}, {:?} seconds", e(), now.elapsed());
}
/*
i x (1,..,n).
i = 1, n=9, we have 123..9.
i = 2, we won't get a 9 digit pandigital.
i = 3 => 36912 ;15
i = 5 => 510 ;15
i = 7 => 714 ;21
i = 9 => 918273645 ; that's not the answer, so we gotta beat that.
*/
fn e() -> Option<u32> {
    (918u32..988)
        .map(|x| format!("{}{}{}", x, x * 2, x * 3))
        .filter(|x| is_pandigital(x))
        .chain({
            (9180..9877)
                .map(|x| format!("{}{}", x, x * 2))
                .filter(|x| is_pandigital(x))
        })
        .map(|x| x.parse().unwrap())
        .inspect(|x| println!("{}", x))
        .max()
}

fn is_pandigital(s: &'_ str) -> bool {
    // ie, does s contain only unique digits
    use ::std::ops::Not;
    s.chars()
        .enumerate()
        .all(|(i, v)| s[(i + 1)..].contains(v).not())
}
// Results
// e:Some(935218704), 1.152537ms seconds
