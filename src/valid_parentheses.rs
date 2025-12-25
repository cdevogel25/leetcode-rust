pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = "".to_string();

        s.chars().for_each(|x| match x {
            '(' | '{' | '[' => { stack.push(x); }
            ')' => { if stack.ends_with('(') { stack.pop(); continue; } else { return false }}
            ']' => { if stack.ends_with('[') { stack.pop(); continue; } else { return false }}
            '}' => { if stack.ends_with('{') { stack.pop(); continue; } else { return false }}
            _ => return false
        })
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_is_valid() {
        let t: String = "(){}[]".to_string();

        assert!(Solution::is_valid(t));
    }
}
