pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut _m: i64 = 0;
        let mut l: i64 = 1;
        let mut h: i64 = x as i64;
        let mut r: i64 = 0;
        while l <= h {
            _m = (l+h)/2;
            if _m*_m <= x as i64{
                r = _m;
                l = _m + 1;
            } else {
                h = _m - 1;
            }
        }
        r as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(9), 3);
        assert_eq!(Solution::my_sqrt(144), 12);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}