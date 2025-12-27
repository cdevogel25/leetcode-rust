pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = "".to_string();
        let mut sv = true;

        if matches!(s.chars().next().unwrap(), ')' | ']' | '}') { return false }

        s.chars().for_each(|x| match x {
            '(' | '{' | '[' => { stack.push(x); }
            ')' => { if stack.ends_with('(') { stack.pop(); } else { sv = false; return }}
            ']' => { if stack.ends_with('[') { stack.pop(); } else { sv = false; return }}
            '}' => { if stack.ends_with('{') { stack.pop(); } else { sv = false; return }}
            _ => return
        });
        return stack == "".to_string() && sv
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_is_valid() {
        let t: String = "(){}[]".to_string();
        let t2: String = "([)]".to_string();
        let t3: String = "]".to_string();
        let t4: String = "(])".to_string();

        assert!(Solution::is_valid(t));
        assert!(!Solution::is_valid(t2));
        assert!(!Solution::is_valid(t3));
        assert!(!Solution::is_valid(t4));
    }
}
