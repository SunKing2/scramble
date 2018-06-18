fn score_word(word: &str) -> i32  {
    [0, 0, 2, 4, 6, 9, 15, 30, 55, 90, 140, 200, 280, 400, 550, 800, 1201][word.len()] 
}
fn main() {
    println!("Hello, world!");
    println!("score of {} is {} and score of {} is {}", "cow", score_word("cow"), "hi", score_word("hi"));
}
