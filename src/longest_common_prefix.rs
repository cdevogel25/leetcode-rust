pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest_prefix: String = "".to_string();
        let first = strs.first().unwrap().to_string();
        for entry in strs {
            match entry.contains(first) {
                
            }
        }
        longest_prefix
    }
}