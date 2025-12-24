pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum: i32 = 0;
        let mut prev_char: char = s.chars().last().unwrap();
        for character in s.chars().rev() {
            match character {
                'I' => {
                    if prev_char == 'V' || prev_char == 'X' { sum -= 1 }
                    else { sum += 1 }
                }
                'V' => {
                    sum += 5;
                }
                'X' => {
                    if prev_char == 'L' || prev_char == 'C' { sum -= 10 }
                    else { sum += 10 }
                }
                'L' => {
                    sum += 50;
                }
                'C' => {
                    if prev_char == 'D' || prev_char == 'M' { sum -= 100 }
                    else { sum += 100 }
                }
                'D' => {
                    sum += 500
                }
                'M' => {
                    sum += 1000
                }
                _ => continue,
            }
            prev_char = character;
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("I".to_string()), 1);
        assert_eq!(Solution::roman_to_int("II".to_string()), 2);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("CD".to_string()), 400);
    }
}