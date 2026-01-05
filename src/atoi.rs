/* 
Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer.

The algorithm for myAtoi(string s) is as follows:

    Whitespace: Ignore any leading whitespace (" ").
    Signedness: Determine the sign by checking if the next character is '-' or '+', assuming positivity if neither present.
    Conversion: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
    Rounding: If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then round the integer to remain in the range. Specifically, integers less than -231 should be rounded to -231, and integers greater than 231 - 1 should be rounded to 231 - 1.

Return the integer as the final result.

 

Example 1:

Input: s = "42"

Output: 42

Explanation:

The underlined characters are what is read in and the caret is the current reader position.
Step 1: "42" (no characters read because there is no leading whitespace)
         ^
Step 2: "42" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "42" ("42" is read in)
           ^

Example 2:

Input: s = " -042"

Output: -42

Explanation:

Step 1: "   -042" (leading whitespace is read and ignored)
            ^
Step 2: "   -042" ('-' is read, so the result should be negative)
             ^
Step 3: "   -042" ("042" is read in, leading zeros ignored in the result)
               ^

Example 3:

Input: s = "1337c0d3"

Output: 1337

Explanation:

Step 1: "1337c0d3" (no characters read because there is no leading whitespace)
         ^
Step 2: "1337c0d3" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "1337c0d3" ("1337" is read in; reading stops because the next character is a non-digit)
             ^

Example 4:

Input: s = "0-1"

Output: 0

Explanation:

Step 1: "0-1" (no characters read because there is no leading whitespace)
         ^
Step 2: "0-1" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "0-1" ("0" is read in; reading stops because the next character is a non-digit)
          ^

Example 5:

Input: s = "words and 987"

Output: 0

Explanation:

Reading stops at the first non-digit character 'w'.


*/

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s;
        let mut l = 0;
        let mut b = 0;
        let mut seen = false;
        s.retain(|c| !c.is_whitespace());

        let sign: i32 = if s.chars().next() == Some('-') {
            s = s[1..].to_string();
            -1
        } else {
            1
        };

        for c in s.chars() {
            if !c.is_numeric() {
                break
            } else if c == '0' {
                b += 1
            } else if c.is_numeric() && c != '0' && !seen {
                seen = true;
            }
            l += 1;
        }

        let mut i: i32 = 1;
        let mut r: i32 = 0;
        s = s[b..l].to_string();
        if l > 10 {
            if sign > 0 {
                return i32::MAX
            } else {
                return i32::MAX * sign + 1
            }
        }
        if seen { s.chars().rev().for_each(|c| { r = r.checked_add((c.to_digit(10).unwrap() as i32).checked_mul(i).unwrap()).unwrap(); i =  i.checked_mul(10).unwrap(); }); }
        (r as i32) * sign
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn atoi() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("-42".to_string()), -42);
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("-042".to_string()), -42);
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648)
    }
}