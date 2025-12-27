pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_remove_element() {
        let mut v = vec![2,3,3,2];

        assert_eq!(Solution::remove_element(&mut v, 3), 2);
    }
}