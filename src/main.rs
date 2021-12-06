mod d;
mod util;

use std::time::Instant;

use crate::d::d6::*;

fn main() {
    let before = Instant::now();
    d6_slow(80);
    println!("Elapsed time: {:.2?}", before.elapsed());
    let before = Instant::now();
    d6_slow_recursive(80);
    println!("Elapsed time: {:.2?}", before.elapsed());
    let before = Instant::now();
    d6_fast(80);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
