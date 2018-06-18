extern crate scramble;
use scramble::{score_word};

fn main() {
    println!("score of {} is {} and score of {} is {}", "leaf", score_word("leaf"), "hi", score_word("hi"));
}
