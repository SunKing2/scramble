// returns a score based on the length of the passed string
// note: there is also a score_bonus() to be applied at the end of a round
pub fn score_word(word: &str) -> i32  {
    [0, 0, 2, 4, 6, 9, 15, 30, 55, 90, 140, 200, 280, 400, 550, 800, 1201][word.len()] 
}
