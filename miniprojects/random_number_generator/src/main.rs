// This is an exercise from the book Creative Projects for Rust Programmers C1

use rand::prelude::*;
use lazy_static::lazy_static;

// Create a static vector containing the squares of numbers from 1 to 200:
lazy_static! {
    pub static ref SQUARES: Vec<u32> = {
        (1..=200).map(|x| x * x).collect()
    };
}

fn main() {
    let mut rng = rand::rng();

    // Generate 10 f32 numbers from 100 ~ 400:
    for _ in 0..10 {
        let num: f32 = rng.random_range(100.0..400.0);
        println!("{}", num);
    }

    // Generate 10 i32 numbers from 100 ~ 400:
    for _ in 0..10 {
        let num: i32 = rng.random_range(100..400);
        println!("{}", num);
    }

    // Generate 10 numbers from the square of 1 ~ 200:
    let random_squares: Vec<u32> = SQUARES
        .choose_multiple(&mut rng, 10)
        .cloned()
        .collect();

    for square in random_squares {
        println!("{}", square);
    }
}