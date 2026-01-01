pub struct Solution;

impl Solution {
    fn fib_mult(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = vec![vec![0,0], vec![0,0]];

        i[0][0] = a[0][0] * b[0][0] + a[0][1] * b[1][0];
        i[0][1] = a[0][0] * b[0][1] + a[0][1] * b[1][1];
        i[1][0] = a[1][0] * b[0][0] + a[1][1] * b[1][0];
        i[1][1] = a[1][0] * b[0][1] + a[1][1] * b[1][1];

        return i
    }
    // function to exponentiate the fibonacci matrix
    // needs to just return the next matrix in the series, not the whole damn one
    fn fib_exp(mut m: &mut Vec<Vec<i32>>, mut exp: i32) -> Vec<Vec<i32>> {
        let mut r = vec![vec![1,0],vec![0,1]];

        while exp > 0 {
            if (exp & 1) != 0 {
                r = Self::fib_mult(r,m);
            }
            r = Self::fib_mult(m, m);
            exp >>= 1;
        }

        return r
        
    }
    pub fn climb_stairs(n: i32) -> i32 {
        let f_init = vec![vec![1],
                                         vec![0]];
        let mut r = f_init.clone();
        let mut exp = vec![vec![1,1],
                                          vec![1,0]];
        
        if n == 1 {
            return 1
        } else if n == 2 {
            return 2
        }

        for _ in 2..n {
            let r_n = r.clone();
            exp = Self::fib_exp(exp);
            r[0][0] = (r_n[0][0] * exp[0][0]) + (r_n[0][0] * exp[1][0]);
            r[1][0] = (r_n[1][0] * exp[1][0]) + (r_n[1][0] * exp[1][1]);
        }

        return r[0][0]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_fib() {
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(5), 5);
        // 1,1,2,3,5,8,13,21...
    }
}