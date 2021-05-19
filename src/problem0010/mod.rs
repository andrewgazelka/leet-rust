use crate::Solution;

#[allow(unused_variables)]

impl Solution {
    pub fn is_match(string: String, pattern: String) -> bool {
        let char_vec: Vec<char> = string.chars().collect();
        for i in (0..char_vec.len()).rev() {
            let c = char_vec[i];
            if c == '*' {
                let next = char_vec[i - 1];

            }
        }
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome(1331), true);
        assert_eq!(Solution::is_palindrome(13331), true);
        assert_eq!(Solution::is_palindrome(135331), false);
    }
}
