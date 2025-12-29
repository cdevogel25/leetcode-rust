pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (i,n) in nums.iter().enumerate() {
            if *n == target { return i as i32 }
            else {
                if *n > target {
                    return (i - 1) as i32
                }
            }
        };
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_search_insert() {
        assert_eq!(Solution::search_insert(vec![1,2,3,5], 4), 2);
    }
}