pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words = s.split_whitespace().last();
        words.unwrap().len() as i32
    }
}