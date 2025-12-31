use std::collections::{ HashMap, HashSet };

pub struct Solution;

impl Solution {
    // I'm pretty happy with this solution, but it can be done better: see below
    // what is going on right now?
    pub fn _length_of_longest_substring(s: String) -> i32 {
        let mut last_index: HashMap<char, i32> = HashMap::new();
        let mut start = 0;
        let mut longest = 0;

        for (i,c) in s.chars().enumerate() {
            // if it's in there already:
            if last_index.get(&c).is_some() {
                // move start up to here
                start = start.max(last_index.get(&c).unwrap() + 1);
                last_index.entry(c).and_modify(|v| *v = i as i32);
            } else {
                last_index.insert(c, i as i32);
            }
            longest = longest.max(i as i32 - start + 1);
        }
        longest
    }

    // replace HashMap with HashSet, operate on the string as bytes (i will annotate this with my understanding)
    pub fn length_of_longest_substring(s: String) -> i32 {
        // HashSet uses less memory than HashMap, since it doesn't have to store keys
        let mut set: HashSet<u8> = HashSet::new();

        // this stays the same
        let mut start = 0;
        let mut longest = 0;
        // this changes, but we're effectively doing the same thing as before, iterate over the string
        for (i,c) in s.as_bytes().iter().enumerate() {
            // here's where the magic happens
            // i don't think this needs to be a while loop, right?
            // it feels wrong
            while set.contains(&c) {
                set.remove(&(s.as_bytes()[start]));
                start += 1;
            }

            set.insert(*c);
            longest = longest.max(i as i32 - start as i32 + 1);
        }

        longest
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}