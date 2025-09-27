// This is an exercise from the book Creative Projects for Rust Programmers C1

use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    // Print 10 f32 numbers from 100 ~ 400:
    loop {
        println!("{}", rng.gen::<f32>(100, 400));
    }
}