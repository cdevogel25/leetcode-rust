use::std::cmp::Ordering;

pub struct Solution;

impl Solution {
    // this is my original
    pub fn _my_sqrt(x: i32) -> i32 {
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

    // original with alterations for (hopefully) improved memory performance
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x
        }

        let mut l: i64 = 1;
        let mut h: i64 = x as i64 / 2;
        let x = x as i64;

        while l <= h {
            let m = l + (h - l) / 2;
            let s = m*m;

            if s == x {
                return m as i32;
            } else if s < x {
                l = m + 1;
            } else {
                h = m - 1;
            }
        }
        h as i32
    }

    // better solution (stolen)
    // not good for memory somehow
    pub fn stolen_my_sqrt(x: i32) -> i32 {
        // x is strictly >= 0, can be 0
        if x == 0 {
            return 0
        }

        // integer sqrt of anything less than 3 is 1
        if x <= 3 {
            return 1
        }

        let mut low = 2;
        let mut high = 46340.min(x / 2);


        // max possible high * high is i32.MAX
        if x >= high * high {
            return high
        }

        while high - low > 1 {
            let mid = (high + low) / 2;
            let pow = mid * mid;
            match pow.cmp(&x) {
                Ordering::Less => { low = mid; }
                Ordering::Greater => { high = mid; }
                _ => return mid
            }
        }
        low
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