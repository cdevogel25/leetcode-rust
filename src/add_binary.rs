pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a;
        let mut b = b;
        let mut i = if a.len() > b.len() { a.len() } else { b.len() };

        let mut r = String::new();
        let mut c = '0';

        while i > 0 {
            i -= 1;

            match (a.pop(), b.pop()) {
                (Some(x), Some(y)) => {
                    match (x,y,c) {
                        ('0','0','0') => { r.insert(0,'0'); c = '0' }
                        ('0','0','1') => { r.insert(0,'1'); c = '0' }
                        ('0','1','0') => { r.insert(0,'1'); c = '0' }
                        ('0','1','1') => { r.insert(0,'0'); c = '1' }
                        ('1','0','0') => { r.insert(0,'1'); c = '0' }
                        ('1','0','1') => { r.insert(0,'0'); c = '1' }
                        ('1','1','0') => { r.insert(0,'0'); c = '1' }
                        ('1','1','1') => { r.insert(0,'1'); c = '1' }
                        (_,_,_) => break,

                    }
                }
                (Some(x), None) => {
                    match (x,c) {
                        ('0','0') => { r.insert(0,'0'); c = '0' }
                        ('0','1') => { r.insert(0,'1'); c = '0' }
                        ('1','0') => { r.insert(0,'1'); c = '0' }
                        ('1','1') => { r.insert(0,'0'); c = '1' }
                        (_,_) => break,
                    }
                }
                (None, Some(y)) => {
                    match (y,c) {
                        ('0','0') => { r.insert(0,'0'); c = '0' }
                        ('0','1') => { r.insert(0,'1'); c = '0' }
                        ('1','0') => { r.insert(0,'1'); c = '0' }
                        ('1','1') => { r.insert(0,'0'); c = '1' }
                        (_,_) => break,
                    }
                }
                (None, None) => break,
            }
        }

        if c == '1' { r.insert(0,'1') }
        if r == "".to_string() { r.push('0') }

        r
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn add_binary() {
        assert_eq!(Solution::add_binary("11".to_string(), "1".to_string()), "100");
    }
}