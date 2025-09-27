// This is an exercise from the book Creative Projects for Rust Programmers C1

use rand::prelude::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    // Print 10 f32 numbers from 100 ~ 400:
    for _ in 0..10 {
        let num: f32 = rng.gen_range(100.0..400.0);
        println!("{}", num);
    }
}