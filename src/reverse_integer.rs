pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut i = "".to_string();
        if x < 0 { i.push('-') }
        x.abs().to_string().chars().rev().for_each(|c| i.push(c));

        i.parse().expect("not an integer, you donk")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn reverse() {
        // 321
        let a = 123;
        // -321
        let b = -123;
        // 21
        let c = 120;
        let d = i32::MAX;

        assert_eq!(Solution::reverse(a), 321);
        assert_eq!(Solution::reverse(b), -321);
        assert_eq!(Solution::reverse(c), 21);
        assert_eq!(Solution::reverse(d), 1)
    }
}