#![feature(int_abs_diff)]

mod d;
mod util;

use std::time::Instant;

use crate::d::d7::*;

fn main() {
    let before = Instant::now();
    d7a();
    println!("Elapsed time: {:.2?}", before.elapsed());
    let before = Instant::now();
    d7b();
    println!("Elapsed time: {:.2?}", before.elapsed());
}
