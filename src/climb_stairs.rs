pub struct Solution;

impl Solution {
    // function to exponentiate the fibonacci matrix
    // needs to just return the next matrix in the series, not the whole damn one
    fn fib_exp(mut m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let i = vec![vec![1,1],
                                    vec![1,0]];

        let mut inter = m.clone();
        inter[0][0] = (m[0][0] * i[0][0]) + (m[0][1] * i[1][0]);
        inter[0][1] = (m[0][0] * i[0][1]) + (m[0][1] * i[1][1]);
        inter[1][0] = (m[1][0] * i[0][0]) + (m[1][1] * i[1][0]);
        inter[1][1] = (m[1][0] * i[0][1]) + (m[1][1] * i[1][1]);
        m = inter;

        m
    }
    pub fn climb_stairs(_n: i32) -> i32 {
        let f_init = vec![vec![1],
                                         vec![0]];
        let mut exp = vec![vec![1,1],
                                          vec![1,0]];

        0

    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_fib() {
        assert_eq!(Solution::climb_stairs(0), 89);
    }
}