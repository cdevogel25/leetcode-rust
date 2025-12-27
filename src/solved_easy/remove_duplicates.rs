pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_remove_duplicates() {
        let mut v = vec![1,1,2];
        let mut v2 = vec![0,0,1,1,1,2,2,3,3,4];

        assert_eq!(Solution::remove_duplicates(&mut v), 2);
        assert_eq!(Solution::remove_duplicates(&mut v2), 5);
    }
}