use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn plus_one_mem(digits: Vec<i32>) -> Vec<i32> {
        let mut v = VecDeque::new();
        let mut carry = 1;

        digits.iter().rev().for_each(|i|
            if *i + carry > 9 {
                v.push_front(*i + carry - 10);
            } else {
                v.push_front(*i + carry);
                carry = 0;
            }
        );
        if carry > 0 {
            v.push_front(carry);
            Vec::from(v)
        } else {
            Vec::from(v)
        }
    }

    // stole this from solutions
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len();

        while i > 0 {
            i -= 1;

            if digits[i] == 9 {
                digits[i] = 0;
                continue;
            } else {
                digits[i] += 1;
                return digits;
            }
        }

        digits.insert(0,1);
        return digits;
    }
}

#[cfg(test)] 
mod test {
    use super::Solution;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1,2,3,4]), vec![1,2,3,5]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1,0]);
        assert_eq!(Solution::plus_one(vec![9,8,7,6,5,4,3,2,1,0]), vec![9,8,7,6,5,4,3,2,1,1]);
    }
}