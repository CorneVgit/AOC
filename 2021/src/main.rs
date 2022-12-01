#![feature(int_abs_diff)]

mod d;
mod util;

use std::time::Instant;

use crate::d::d9::*;

fn main() {
    let before = Instant::now();
    d9a();
    println!("Elapsed time: {:.2?}", before.elapsed());
    let before = Instant::now();
    d9b();
    println!("Elapsed time: {:.2?}", before.elapsed());
}
