use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = num_map.get(&num) {
                return vec![j, i as i32];
            } else {
                num_map.insert(target - num, i as i32);
            }
        }
        vec![]
    }
}