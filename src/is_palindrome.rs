pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        let s_rev: String = s.chars().rev().collect();

        s == s_rev
    }
}