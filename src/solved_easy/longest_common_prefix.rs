pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut first = strs.first().unwrap().to_string();
        let mut _t = 0;
        
        loop {
            _t = strs.iter().filter(|s| s.starts_with(&first)).count();
            if _t != strs.len() { first.pop() } else { return first };
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_longest_common_prefix() {
        let strs: Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];

        assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());
    }
}